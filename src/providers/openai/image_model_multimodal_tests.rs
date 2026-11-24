use crate::core::messages::UserImage;
use crate::core::{DynamicModel, ImageModelRequest};
use crate::error::Error;
use crate::providers::openai::OpenAI;
use serde_json::json;
use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const CHAT: &str = "/v1/chat/completions";
const WAN: &str = "/ali/api/v1/services/aigc/image-generation/generation";
const TASK: &str = "/ali/api/v1/tasks/task-123";
const INLINE: &str = "data:image/png;base64,Zm9v";

fn model(server: &MockServer, model_id: &str, endpoint: &str) -> OpenAI<DynamicModel> {
    let mut model = OpenAI::model_name(model_id);
    model.settings.base_url = server.uri();
    model.settings.api_key = "test-key".into();
    model.settings.path = Some(endpoint.into());
    model
}

#[tokio::test]
async fn media_image_provider_gemini_extracts_chat_images_and_preserves_order() {
    let server = MockServer::start().await;
    let reference = "https://images.example.com/reference.png";
    let result = format!("{}/result?signature=fixture", server.uri());
    Mock::given(method("POST"))
        .and(path(CHAT))
        .and(header("authorization", "Bearer test-key"))
        .and(body_partial_json(json!({
            "model": "deployment-alias",
            "stream": false,
            "modalities": ["image", "text"],
            "messages": [{
                "role": "user",
                "content": [
                    {"type": "text", "text": "edit this photo"},
                    {"type": "image_url", "image_url": {"url": INLINE}},
                    {"type": "image_url", "image_url": {"url": reference}}
                ]
            }]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [
                {"message": {
                    "images": [{"image_url": {"url": INLINE}}],
                    "content": [
                        {"type": "text", "text": "done"},
                        {"type": "image_url", "image_url": {"url": INLINE}},
                        {"type": "image_url", "image_url": {"url": result}}
                    ]
                }},
                {"message": {"content": "![another](data:image/jpeg;base64,YmF6)"}}
            ],
            "usage": {"prompt_tokens": 2, "completion_tokens": 3, "total_tokens": 5}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/result"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("bar", "image/png"))
        .expect(1)
        .mount(&server)
        .await;

    let response = ImageModelRequest::builder()
        .model(model(&server, "deployment-alias", CHAT))
        .prompt("edit this photo")
        .file(UserImage::new(INLINE))
        .file(UserImage::new(reference))
        .build()
        .generate_image()
        .await
        .unwrap();

    assert_eq!(response.images(), ["Zm9v", "YmFy", "YmF6"]);
    let usage = response.usage().unwrap();
    assert_eq!(usage.input_tokens, Some(2));
    assert_eq!(usage.output_tokens, Some(3));
    assert_eq!(usage.total_tokens, Some(5));
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 2);
    assert!(!requests[1].headers.contains_key("authorization"));
    assert_eq!(requests[1].url.query(), Some("signature=fixture"));
}

#[tokio::test]
async fn media_image_provider_gemini_does_not_resubmit_on_uncertain_outcomes() {
    for reply in [
        ResponseTemplate::new(502),
        ResponseTemplate::new(307).insert_header("location", "/v1/images/generations"),
        ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{"message": {"content": "text only"}}]
        })),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(CHAT))
            .and(body_partial_json(json!({
                "messages": [{"role": "user", "content": [{"type": "text", "text": "draw a dog"}]}]
            })))
            .respond_with(reply)
            .expect(1)
            .mount(&server)
            .await;

        let result = ImageModelRequest::builder()
            .model(model(&server, "gemini-2.5-flash-image", CHAT))
            .prompt("draw a dog")
            .build()
            .generate_image()
            .await;

        assert!(matches!(result, Err(Error::ApiError { .. })));
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
}

#[tokio::test]
async fn media_image_provider_dashscope_submits_references_and_polls_success() {
    let server = MockServer::start().await;
    let reference = "https://images.example.com/reference.png";
    Mock::given(method("POST"))
        .and(path(WAN))
        .and(header("authorization", "Bearer test-key"))
        .and(header("x-dashscope-async", "enable"))
        .and(body_partial_json(json!({
            "model": "wan2.6-image",
            "input": {"messages": [{
                "role": "user",
                "content": [{"image": INLINE}, {"image": reference}, {"text": "retouch"}]
            }]},
            "parameters": {"n": 2, "size": "1280*720", "seed": 42, "negative_prompt": "blur"}
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "output": {"task_id": "task-123", "task_status": "PENDING"}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(TASK))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "output": {"task_id": "task-123", "task_status": "RUNNING"}
        })))
        .with_priority(1)
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    let result = format!("{}/result", server.uri());
    Mock::given(method("GET"))
        .and(path(TASK))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "output": {
                "task_id": "task-123",
                "task_status": "SUCCEEDED",
                "choices": [{"message": {"content": [{"text": "done"}, {"image": result}]}}],
                "results": [{"url": result}]
            },
            "usage": {"image_count": 1}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/result"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("result", "image/png"))
        .expect(1)
        .mount(&server)
        .await;

    let response = ImageModelRequest::builder()
        .model(model(&server, "wan2.6-image", WAN))
        .prompt("retouch")
        .file(UserImage::new(INLINE))
        .file(UserImage::new(reference))
        .n(2)
        .size("1280x720")
        .seed(42u32)
        .body(json!({"parameters": {"negative_prompt": "blur"}}))
        .build()
        .generate_image()
        .await
        .unwrap();

    assert_eq!(response.images(), ["cmVzdWx0"]);
    assert!(response.usage().is_none());
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 4);
    for poll in &requests[1..3] {
        assert_eq!(poll.url.path(), TASK);
        assert!(!poll.headers.contains_key("x-dashscope-async"));
    }
    assert!(!requests[3].headers.contains_key("authorization"));
}

#[tokio::test]
async fn media_image_provider_dashscope_reports_terminal_states_without_resubmission() {
    for (status, expected) in [
        ("FAILED", "FAILED"),
        ("CANCELED", "CANCELED"),
        ("UNKNOWN", "UNKNOWN"),
        ("SUCCEEDED", "no results"),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(WAN))
            .and(header("x-dashscope-async", "enable"))
            .and(body_partial_json(json!({
                "input": {"messages": [{"role": "user", "content": [{"text": "draw a dog"}]}]},
                "parameters": {"n": 1, "size": "720*1280"}
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "output": {"task_id": "task-123", "task_status": "PENDING"}
            })))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(TASK))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "output": {"task_id": "task-123", "task_status": status}
            })))
            .expect(1)
            .mount(&server)
            .await;

        let error = ImageModelRequest::builder()
            .model(model(&server, "wan2.7-image-pro", WAN))
            .prompt("draw a dog")
            .aspect_ratio("9:16")
            .build()
            .generate_image()
            .await
            .unwrap_err();

        let Error::ApiError { details, .. } = error else {
            panic!("expected an API error for {status}");
        };
        assert!(details.contains(expected), "{details}");
        assert!(details.contains("task-123"), "{details}");
        assert_eq!(server.received_requests().await.unwrap().len(), 2);
    }
}

#[tokio::test]
async fn media_image_provider_dashscope_missing_task_id_does_not_resubmit() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(WAN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "request_id": "req-123",
            "output": {"task_status": "PENDING"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let error = ImageModelRequest::builder()
        .model(model(&server, "wan2.7-image-pro", WAN))
        .prompt("draw a dog")
        .build()
        .generate_image()
        .await
        .unwrap_err();

    let Error::ApiError { details, .. } = error else {
        panic!("expected an API error");
    };
    assert!(details.contains("task ID"), "{details}");
    assert!(details.contains("req-123"), "{details}");
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
