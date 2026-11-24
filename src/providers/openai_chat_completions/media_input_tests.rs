use crate::core::{
    DynamicModel, LanguageModel, Message, UserMessage,
    language_model::LanguageModelOptions,
    messages::{UserAudio, UserVideo},
    tools::ToolResultInfo,
};
use crate::providers::openai_chat_completions::OpenAIChatCompletions;
use serde_json::{Value, json};
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_model(
    server: &MockServer,
    provider: &str,
    model_name: &str,
) -> OpenAIChatCompletions<DynamicModel> {
    let mut model = OpenAIChatCompletions::model_name(model_name);
    model.settings.provider_name = provider.into();
    model.settings.base_url = server.uri();
    model.settings.api_key = "test-key".into();
    model
}

async fn expect_request(server: &MockServer, body: Value) {
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .and(body_partial_json(body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "test",
            "object": "chat.completion",
            "created": 0,
            "model": "test",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": "ok"},
                "finish_reason": "stop"
            }]
        })))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn media_native_input_openai_uses_raw_base64_and_audio_format() {
    let server = MockServer::start().await;
    expect_request(
        &server,
        json!({
            "model": "gpt-audio",
            "messages": [{"role": "user", "content": [
                {"type": "text", "text": "Describe"},
                {"type": "input_audio", "input_audio": {"data": "AQID", "format": "mp3"}}
            ]}]
        }),
    )
    .await;

    let mut model = test_model(&server, "openai", "gpt-audio");
    model
        .generate_text(LanguageModelOptions {
            messages: vec![
                Message::User(
                    UserMessage::new("Describe")
                        .with_audio(UserAudio::from_bytes("audio/mpeg", vec![1, 2, 3])),
                )
                .into(),
            ],
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn media_native_input_dashscope_preserves_image_and_tool_history() {
    let server = MockServer::start().await;
    expect_request(
        &server,
        json!({
            "model": "qwen3.8-omni-flash",
            "messages": [
                {"role": "tool", "name": "lookup", "tool_call_id": "call-1", "content": "42"},
                {"role": "user", "content": [
                    {"type": "text", "text": "Compare"},
                    {"type": "image_url", "image_url": {"url": "https://example.com/image.png"}},
                    {"type": "input_audio", "input_audio": {
                        "data": "data:audio/wav;base64,AQID",
                        "format": "wav"
                    }},
                    {"type": "input_audio", "input_audio": {
                        "data": "https://example.com/audio.mp3",
                        "format": "mp3"
                    }},
                    {"type": "video_url", "video_url": {"url": "data:video/mp4;base64,BAUG"}},
                    {"type": "video_url", "video_url": {"url": "https://example.com/video.mp4"}}
                ]}
            ]
        }),
    )
    .await;

    let mut tool = ToolResultInfo::new("lookup");
    tool.id("call-1");
    tool.output = Ok(json!(42));
    let user = UserMessage::new("Compare")
        .with_image_url("https://example.com/image.png")
        .with_audio(UserAudio::from_bytes("audio/wav", vec![1, 2, 3]))
        .with_audio(UserAudio::from_url(
            "audio/mpeg",
            "https://example.com/audio.mp3",
        ))
        .with_video(UserVideo::from_bytes("video/mp4", vec![4, 5, 6]))
        .with_video(UserVideo::from_url(
            "video/mp4",
            "https://example.com/video.mp4",
        ));
    let mut model = test_model(&server, "AlibabaCn", "qwen3.8-omni-flash");
    model
        .generate_text(LanguageModelOptions {
            messages: vec![Message::Tool(tool).into(), Message::User(user).into()],
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn media_native_input_chat_rejects_unsupported_deployments_and_sources() {
    let server = MockServer::start().await;
    let cases = [
        (
            "other",
            "gpt-audio",
            UserAudio::from_bytes("audio/wav", vec![1]),
        ),
        (
            "openai",
            "gpt-4o",
            UserAudio::from_bytes("audio/wav", vec![1]),
        ),
        (
            "openai",
            "gpt-audio",
            UserAudio::from_url("audio/wav", "https://example.com/a.wav"),
        ),
        (
            "openai",
            "gpt-audio",
            UserAudio::from_bytes("audio/ogg", vec![1]),
        ),
        (
            "alibaba",
            "qwen3-vl-plus",
            UserAudio::from_bytes("audio/wav", vec![1]),
        ),
    ];

    for (provider, model_name, audio) in cases {
        let mut model = test_model(&server, provider, model_name);
        let result = model
            .stream_text(LanguageModelOptions {
                messages: vec![
                    Message::User(UserMessage::new("Describe").with_audio(audio)).into(),
                ],
                ..Default::default()
            })
            .await;
        assert!(matches!(result, Err(crate::Error::InvalidInput(_))));
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "openai")]
#[tokio::test]
async fn media_native_input_responses_rejects_unimplemented_media() {
    let server = MockServer::start().await;
    let mut model = crate::providers::OpenAI::<DynamicModel>::model_name("gpt-audio");
    model.settings.base_url = server.uri();
    model.settings.api_key = "test-key".into();
    let result = model
        .generate_text(LanguageModelOptions {
            messages: vec![
                Message::User(UserMessage::from_parts(vec![
                    UserVideo::from_bytes("video/mp4", vec![1]).into(),
                ]))
                .into(),
            ],
            ..Default::default()
        })
        .await;
    assert!(matches!(result, Err(crate::Error::InvalidInput(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "alibaba")]
#[test]
fn media_native_input_dynamic_alibaba_uses_its_own_deployment() {
    let model = crate::providers::alibaba::Alibaba::model_name("qwen3.8-omni-flash");
    assert_eq!(model.inner.settings.base_url, model.settings.base_url);
    assert_eq!(
        model.inner.settings.provider_name,
        model.settings.provider_name
    );
}
