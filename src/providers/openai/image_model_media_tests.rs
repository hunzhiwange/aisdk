use crate::core::messages::UserImage;
use crate::core::{DynamicModel, ImageModelRequest};
use crate::error::Error;
use crate::providers::openai::OpenAI;
use serde_json::json;
use wiremock::matchers::{body_partial_json, body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn model(server: &MockServer) -> OpenAI<DynamicModel> {
    let mut model = OpenAI::model_name("gpt-image-1");
    model.settings.base_url = server.uri();
    model.settings.api_key = "test-key".into();
    model
}

#[tokio::test]
async fn media_image_wire_generates_inline_and_url_results_in_order() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/images/generations"))
        .and(header("authorization", "Bearer test-key"))
        .and(body_partial_json(json!({
            "model": "gpt-image-1",
            "prompt": "draw two birds",
            "n": 2,
            "response_format": "url"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [
                { "b64_json": "Zm9v", "url": null },
                { "b64_json": null, "url": format!("{}/result?signature=fixture", server.uri()) }
            ],
            "usage": { "total_tokens": 5 }
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/result"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(b"\x89PNG\r\n\x1a\n".to_vec(), "application/octet-stream"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let response = ImageModelRequest::builder()
        .model(model(&server))
        .prompt("draw two birds")
        .n(2)
        .body(json!({ "response_format": "url" }))
        .build()
        .generate_image()
        .await
        .expect("both wire representations should decode");

    assert_eq!(response.images, ["Zm9v", "iVBORw0KGgo="]);
    assert_eq!(response.usage.unwrap().total_tokens, Some(5));

    let requests = server.received_requests().await.unwrap();
    let download = requests
        .iter()
        .find(|request| request.url.path() == "/result")
        .unwrap();
    assert!(!download.headers.contains_key("authorization"));
    assert_eq!(download.url.query(), Some("signature=fixture"));
}

#[tokio::test]
async fn media_image_wire_edits_mime_typed_inline_and_remote_references() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/reference"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("remote-image", "image/webp"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/images/edits"))
        .and(header("authorization", "Bearer test-key"))
        .and(body_string_contains(
            "name=\"image\"; filename=\"image.jpg\"",
        ))
        .and(body_string_contains(
            "Content-Type: image/jpeg\r\n\r\ninline-image",
        ))
        .and(body_string_contains(
            "name=\"image\"; filename=\"image.webp\"",
        ))
        .and(body_string_contains(
            "Content-Type: image/webp\r\n\r\nremote-image",
        ))
        .and(body_string_contains("name=\"mask\"; filename=\"mask.png\""))
        .and(body_string_contains("name=\"quality\""))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{ "url": format!("{}/edited", server.uri()) }]
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/edited"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("edited", "image/png"))
        .expect(1)
        .mount(&server)
        .await;

    let response = ImageModelRequest::builder()
        .model(model(&server))
        .prompt("retouch the image")
        .file(UserImage::new("data:image/jpeg;base64,aW5saW5lLWltYWdl"))
        .file(UserImage::new(format!("{}/reference", server.uri())))
        .mask(UserImage::new("data:image/png;base64,bWFzaw=="))
        .body(json!({ "quality": "high" }))
        .build()
        .generate_image()
        .await
        .expect("the edit should upload both reference images and return its result");

    assert_eq!(response.images, ["ZWRpdGVk"]);
}

#[tokio::test]
async fn media_image_wire_rejects_invalid_results() {
    use wiremock::matchers::query_param;

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/redirect"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/redirect-target"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/text"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not an image", "text/plain"))
        .expect(1)
        .mount(&server)
        .await;

    for (case, data) in [
        ("empty", json!([])),
        ("missing", json!([{}])),
        ("blank", json!([{ "b64_json": "" }])),
        ("malformed", json!([{ "b64_json": "%%%" }])),
        (
            "unsafe",
            json!([{ "url": "https://169.254.169.254/media" }]),
        ),
        (
            "redirect",
            json!([{ "url": format!("{}/redirect", server.uri()) }]),
        ),
        (
            "non-image",
            json!([{ "url": format!("{}/text", server.uri()) }]),
        ),
    ] {
        Mock::given(method("POST"))
            .and(path("/v1/images/generations"))
            .and(query_param("case", case))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": data })))
            .expect(1)
            .mount(&server)
            .await;

        let mut model = model(&server);
        model.settings.path = Some(format!("/v1/images/generations?case={case}"));
        let result = ImageModelRequest::builder()
            .model(model)
            .prompt("draw a fox")
            .build()
            .generate_image()
            .await;
        assert!(matches!(result, Err(Error::ApiError { .. })), "case {case}");
    }
    let requests = server.received_requests().await.unwrap();
    assert!(
        !requests
            .iter()
            .any(|request| request.url.path() == "/redirect-target")
    );
}

#[cfg(feature = "google")]
#[tokio::test]
async fn media_image_wire_google_returns_only_native_inline_images() {
    use crate::providers::google::Google;

    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1beta/models/gemini-2.5-flash-image:generateContent",
        ))
        .and(header("x-goog-api-key", "test-key"))
        .and(body_partial_json(json!({
            "contents": [{
                "role": "user",
                "parts": [
                    { "text": "edit this image" },
                    { "inlineData": { "mimeType": "image/png", "data": "Zm9v" } }
                ]
            }],
            "generationConfig": { "responseModalities": ["IMAGE"] }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [
                        { "text": "edited" },
                        { "inlineData": { "mimeType": "audio/wav", "data": "%%%" } },
                        { "inlineData": { "mimeType": "image/webp", "data": "YmFy" } }
                    ]
                }
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let mut model = Google::model_name("gemini-2.5-flash-image");
    model.settings.base_url = server.uri();
    model.settings.api_key = "test-key".into();
    let response = ImageModelRequest::builder()
        .model(model)
        .prompt("edit this image")
        .file(UserImage::new("data:image/png;base64,Zm9v"))
        .build()
        .generate_image()
        .await
        .expect("the native response should contain a WebP image");

    assert_eq!(response.images, ["YmFy"]);
}

#[tokio::test]
async fn media_image_wire_limits_download_body_size() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/image"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("too large", "image/png"))
        .expect(1)
        .mount(&server)
        .await;

    let response = reqwest::Client::builder()
        .no_proxy()
        .build()
        .unwrap()
        .get(format!("{}/image", server.uri()))
        .send()
        .await
        .unwrap();

    assert!(matches!(
        super::read_limited_body(response, 4).await,
        Err(Error::ApiError { .. })
    ));
}
