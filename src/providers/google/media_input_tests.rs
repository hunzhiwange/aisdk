use crate::core::{
    DynamicModel, LanguageModel, Message, UserMessage,
    language_model::LanguageModelOptions,
    messages::{UserAudio, UserVideo},
    tools::ToolResultInfo,
};
use crate::providers::google::Google;
use serde_json::json;
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_model(server: &MockServer, name: &str) -> Google<DynamicModel> {
    let mut model = Google::model_name(name);
    model.settings.base_url = server.uri();
    model.settings.api_key = "test-key".into();
    model
}

#[tokio::test]
async fn media_native_input_google_preserves_mime_image_and_tool_history() {
    let server = MockServer::start().await;
    let file_url = "https://generativelanguage.googleapis.com/v1beta/files/sample";
    Mock::given(method("POST"))
        .and(path("/v1beta/models/gemini-2.5-flash:generateContent"))
        .and(body_partial_json(json!({
            "contents": [
                {"role": "user", "parts": [{"functionResponse": {
                    "name": "lookup", "response": {"result": 42}
                }}]},
                {"role": "user", "parts": [
                    {"text": "Compare"},
                    {"inlineData": {"mimeType": "image/png", "data": "AQID"}},
                    {"inlineData": {"mimeType": "audio/wav", "data": "BAUG"}},
                    {"inlineData": {"mimeType": "video/webm", "data": "BwgJ"}},
                    {"fileData": {"mimeType": "video/mp4", "fileUri": file_url}}
                ]}
            ]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "candidates": [{"content": {"role": "model", "parts": [{"text": "ok"}]}}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let mut tool = ToolResultInfo::new("lookup");
    tool.output = Ok(json!(42));
    let user = UserMessage::new("Compare")
        .with_image_url("data:image/png;base64,AQID")
        .with_audio(UserAudio::from_bytes("audio/wav", vec![4, 5, 6]))
        .with_video(UserVideo::from_bytes("video/webm", vec![7, 8, 9]))
        .with_video(UserVideo::from_url("video/mp4", file_url));
    let mut model = test_model(&server, "gemini-2.5-flash");
    model
        .generate_text(LanguageModelOptions {
            messages: vec![Message::Tool(tool).into(), Message::User(user).into()],
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn media_native_input_google_rejects_unsupported_models_and_invalid_parts() {
    let server = MockServer::start().await;
    let cases = [
        (
            "gemini-2.5-flash-preview-tts",
            UserVideo::from_bytes("video/mp4", vec![1]),
        ),
        (
            "gemini-2.5-flash",
            UserVideo::from_bytes("audio/wav", vec![1]),
        ),
        (
            "gemini-2.5-flash",
            UserVideo::from_bytes("video/mp4", Vec::new()),
        ),
        (
            "gemini-2.5-flash",
            UserVideo::from_url("video/mp4", "file:///tmp/video.mp4"),
        ),
    ];

    for (name, video) in cases {
        let mut model = test_model(&server, name);
        let result = model
            .generate_text(LanguageModelOptions {
                messages: vec![
                    Message::User(UserMessage::new("Describe").with_video(video)).into(),
                ],
                ..Default::default()
            })
            .await;
        assert!(matches!(result, Err(crate::Error::InvalidInput(_))));
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}
