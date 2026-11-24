use super::OpenAI;
use super::video_model::{OpenAIVideoCapabilities, OpenAIVideoModel};
use crate::core::video_model::{
    VideoJobStatus, VideoModel, VideoModelError, VideoModelOptions, VideoReferenceImage,
};
use reqwest::StatusCode;
use serde_json::json;
use wiremock::matchers::{body_json, body_string_contains, header, header_regex, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn model(server: &MockServer) -> OpenAIVideoModel {
    OpenAI::builder()
        .model_name("vendor/raw-video-v2")
        .base_url(server.uri())
        .api_key("video-key")
        .build()
        .unwrap()
        .video_model(OpenAIVideoCapabilities {
            image_reference: true,
            cancel: true,
            content: true,
        })
        .unwrap()
}

fn input() -> VideoModelOptions {
    VideoModelOptions {
        prompt: "Animate a lighthouse".into(),
        seconds: Some(4),
        size: Some("1280x720".into()),
        input_reference: None,
    }
}

#[tokio::test]
async fn media_video_wire_json_status_and_content() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/videos"))
        .and(header("authorization", "Bearer video-key"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "model": "vendor/raw-video-v2",
            "prompt": "Animate a lighthouse",
            "seconds": "4",
            "size": "1280x720"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "vid-1", "status": "queued", "progress": 0
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/videos/vid-1"))
        .and(header("authorization", "Bearer video-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "vid-1", "status": "completed", "progress": 100,
            "video_url": "https://assets.example.test/video?signature=test"
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/videos/vid-1/content"))
        .and(header("authorization", "Bearer video-key"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(b"mp4-data".to_vec(), "video/mp4"))
        .expect(2)
        .mount(&server)
        .await;

    let model = model(&server);
    let job = model.submit_video(input()).await.unwrap();
    assert_eq!(job.id, "vid-1");
    assert_eq!(job.status, VideoJobStatus::Queued);
    assert_eq!(job.progress, Some(0.0));

    let finished = model.video_status(&job.id).await.unwrap();
    assert_eq!(finished.status, VideoJobStatus::Succeeded);
    assert_eq!(finished.progress, Some(100.0));
    assert_eq!(
        finished.result_url.unwrap().as_str(),
        "https://assets.example.test/video?signature=test"
    );
    let content = model.read_video_content(&job.id, 8).await.unwrap();
    assert_eq!(content.media_type, "video/mp4");
    assert_eq!(content.bytes.as_slice(), b"mp4-data");
    assert_eq!(
        model.read_video_content(&job.id, 7).await.unwrap_err(),
        VideoModelError::ContentTooLarge { limit: 7 }
    );
    assert_eq!(server.received_requests().await.unwrap().len(), 4);
}

#[tokio::test]
async fn media_video_wire_multipart_deployment_path_and_cancel() {
    let server = MockServer::start().await;
    let model = OpenAI::builder()
        .model_name("vendor/raw-video-v2")
        .base_url(format!("{}/deployments/current", server.uri()))
        .path("media/videos")
        .api_key("deployment-key")
        .build()
        .unwrap()
        .video_model(OpenAIVideoCapabilities {
            image_reference: true,
            cancel: true,
            content: false,
        })
        .unwrap();

    Mock::given(method("POST"))
        .and(path("/deployments/current/media/videos"))
        .and(header("authorization", "Bearer deployment-key"))
        .and(header_regex(
            "content-type",
            "multipart/form-data; boundary=.*",
        ))
        .and(body_string_contains(
            "name=\"input_reference\"; filename=\"reference.png\"",
        ))
        .and(body_string_contains("Content-Type: image/png"))
        .and(body_string_contains("reference-image-bytes"))
        .and(body_string_contains("vendor/raw-video-v2"))
        .and(body_string_contains("Animate a lighthouse"))
        .and(body_string_contains("name=\"seconds\"\r\n\r\n4\r\n"))
        .and(body_string_contains("1280x720"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "task_id": "vid/2", "status": "in_progress", "progress": 32.5
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/deployments/current/media/videos/vid%2F2"))
        .and(header("authorization", "Bearer deployment-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "vid/2", "deleted": true
        })))
        .expect(1)
        .mount(&server)
        .await;

    let job = model
        .submit_video(VideoModelOptions {
            input_reference: Some(VideoReferenceImage {
                media_type: "image/png".into(),
                bytes: b"reference-image-bytes".to_vec(),
            }),
            ..input()
        })
        .await
        .unwrap();
    assert_eq!(job.id, "vid/2");
    assert_eq!(job.status, VideoJobStatus::InProgress);
    assert_eq!(job.progress, Some(32.5));
    model.cancel_video(&job.id).await.unwrap();
    assert_eq!(
        model.read_video_content(&job.id, 32).await.unwrap_err(),
        VideoModelError::Unsupported("content")
    );
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[tokio::test]
async fn media_video_wire_unknown_submit_is_sent_once() {
    let server = MockServer::start().await;

    for response in [
        ResponseTemplate::new(503),
        ResponseTemplate::new(408),
        ResponseTemplate::new(200).set_body_string("truncated JSON"),
        ResponseTemplate::new(200).set_body_json(json!({ "status": "queued" })),
    ] {
        Mock::given(method("POST"))
            .and(path("/v1/videos"))
            .respond_with(response)
            .expect(1)
            .mount(&server)
            .await;

        assert_eq!(
            model(&server).submit_video(input()).await.unwrap_err(),
            VideoModelError::SubmissionUnknown
        );
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
        server.verify().await;
        server.reset().await;
    }

    Mock::given(method("POST"))
        .and(path("/v1/videos"))
        .respond_with(ResponseTemplate::new(400))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        model(&server).submit_video(input()).await.unwrap_err(),
        VideoModelError::Http(StatusCode::BAD_REQUEST)
    );
}

#[tokio::test]
async fn media_video_wire_terminal_states_and_unsupported_cancel() {
    let server = MockServer::start().await;
    for (status, expected) in [
        ("failed", VideoJobStatus::Failed),
        ("cancelled", VideoJobStatus::Cancelled),
        ("timed_out", VideoJobStatus::TimedOut),
        ("vendor_status", VideoJobStatus::Unknown),
    ] {
        Mock::given(method("GET"))
            .and(path(format!("/v1/videos/{status}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "task_id": status, "status": status, "progress": 51
            })))
            .expect(1)
            .mount(&server)
            .await;
        let job = model(&server).video_status(status).await.unwrap();
        assert_eq!(job.id, status);
        assert_eq!(job.status, expected);
        assert_eq!(job.progress, Some(51.0));
    }

    Mock::given(method("DELETE"))
        .and(path("/v1/videos/failed"))
        .respond_with(ResponseTemplate::new(405))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        model(&server).cancel_video("failed").await.unwrap_err(),
        VideoModelError::Unsupported("cancel")
    );

    Mock::given(method("DELETE"))
        .and(path("/v1/videos/declined"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "declined", "deleted": false
        })))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        model(&server).cancel_video("declined").await.unwrap_err(),
        VideoModelError::InvalidResponse("cancellation was not confirmed")
    );

    Mock::given(method("GET"))
        .and(path("/v1/videos/slow"))
        .respond_with(ResponseTemplate::new(408))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        model(&server).video_status("slow").await.unwrap_err(),
        VideoModelError::Timeout
    );
}

#[tokio::test]
async fn media_video_wire_result_url_and_redirect_stay_outside_authenticated_requests() {
    let server = MockServer::start().await;
    let external = MockServer::start().await;
    let location = format!("{}/result", external.uri());

    Mock::given(method("GET"))
        .and(path("/v1/videos/vid-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "vid-1", "status": "succeeded", "url": "file:///private/result"
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/videos/vid-1/content"))
        .and(header("authorization", "Bearer video-key"))
        .respond_with(ResponseTemplate::new(307).insert_header("location", location.as_str()))
        .expect(1)
        .mount(&server)
        .await;

    let model = model(&server);
    assert!(
        model
            .video_status("vid-1")
            .await
            .unwrap()
            .result_url
            .is_none()
    );
    assert_eq!(
        model.read_video_content("vid-1", 32).await.unwrap_err(),
        VideoModelError::Http(StatusCode::TEMPORARY_REDIRECT)
    );
    assert!(external.received_requests().await.unwrap().is_empty());
}
