//! Helper functions and conversions for the OpenAI Chat Completions provider.

use crate::core::language_model::{LanguageModelOptions, LanguageModelResponseContentType, Usage};
use crate::core::messages::{
    Message, UserAudio, UserContentPart, UserMediaSource, UserMessage, UserVideo,
};
use crate::core::tools::Tool as SdkTool;
use crate::providers::openai_chat_completions::client::{self, types};
use crate::providers::openai_chat_completions::settings::OpenAIChatCompletionsSettings;
use base64::{Engine, engine::general_purpose::STANDARD};

// ============================================================================
// LanguageModelOptions -> ChatCompletionsOptions
// ============================================================================

#[cfg(test)]
impl TryFrom<LanguageModelOptions> for client::ChatCompletionsOptions {
    type Error = crate::Error;

    fn try_from(options: LanguageModelOptions) -> crate::Result<Self> {
        Self::from_sdk(options, String::new(), ChatMediaProtocol::Unsupported)
    }
}

impl client::ChatCompletionsOptions {
    pub(crate) fn for_model(
        options: LanguageModelOptions,
        model: &str,
        settings: &OpenAIChatCompletionsSettings,
    ) -> crate::Result<Self> {
        // Extra body fields can override the model used by the deployment.
        let effective_model = options
            .body
            .as_ref()
            .and_then(|body| body.get("model"))
            .or_else(|| settings.body.as_ref().and_then(|body| body.get("model")))
            .map_or(model, |value| value.as_str().unwrap_or_default());
        let protocol = ChatMediaProtocol::for_model(settings, effective_model);
        Self::from_sdk(options, model.to_owned(), protocol)
    }

    fn from_sdk(
        options: LanguageModelOptions,
        model: String,
        protocol: ChatMediaProtocol,
    ) -> crate::Result<Self> {
        options.reject_native_tool_protocol("openai_chat_completions")?;
        let extra_body = options.body.clone();
        let extra_headers = options.headers.clone();
        let mut messages: Vec<types::ChatMessage> = Vec::new();

        if let Some(system_prompt) = options.system {
            messages.push(types::ChatMessage {
                role: types::Role::System,
                content: Some(types::ChatMessageContent::text(system_prompt)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            });
        }

        messages.extend(
            options
                .messages
                .into_iter()
                .map(|tagged| types::ChatMessage::from_sdk(tagged.message, protocol))
                .collect::<crate::Result<Vec<types::ChatMessage>>>()?,
        );

        let tools: Option<Vec<types::Tool>> = options.tools.map(|tool_list| {
            tool_list
                .tools
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .iter()
                .map(|t| t.clone().into())
                .collect()
        });

        let response_format = options.schema.map(|schema| {
            let mut json_value = serde_json::to_value(schema).unwrap();

            // Ensure required fields for OpenAI Structured Outputs
            if let serde_json::Value::Object(ref mut obj) = json_value {
                obj.insert(
                    "additionalProperties".to_string(),
                    serde_json::Value::Bool(false),
                );
            }

            types::ResponseFormat::JsonSchema {
                json_schema: types::JsonSchemaDefinition {
                    name: json_value
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Response")
                        .to_string(),
                    schema: json_value.clone(),
                    description: json_value
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    strict: Some(true),
                },
            }
        });

        let reasoning_effort = options
            .reasoning_effort
            .map(|effort| effort.as_str().to_string());

        let tool_choice = if tools.is_some() {
            Some(types::ToolChoice::String("auto".to_string()))
        } else {
            None
        };

        let parallel_tool_calls = if tools.is_some() { Some(true) } else { None };

        Ok(client::ChatCompletionsOptions {
            model,
            messages,
            frequency_penalty: options.frequency_penalty,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_completion_tokens: options.max_output_tokens,
            n: None,
            presence_penalty: options.presence_penalty,
            response_format,
            seed: options.seed,
            stop: options.stop_sequences.map(|seqs| {
                if seqs.len() == 1 {
                    types::StopSequences::Single(seqs[0].clone())
                } else {
                    types::StopSequences::Multiple(seqs.into_iter().take(4).collect())
                }
            }),
            stream: None,
            stream_options: None,
            temperature: options.temperature.map(|t| t as f32 / 100.0),
            top_p: options.top_p.map(|t| t as f32 / 100.0),
            tools,
            tool_choice,
            parallel_tool_calls,
            reasoning_effort,
            verbosity: None,
            extra_body,
            extra_headers,
        })
    }
}

// ============================================================================
// SDK Message -> ChatMessage
// ============================================================================

#[cfg(test)]
impl TryFrom<Message> for types::ChatMessage {
    type Error = crate::Error;

    fn try_from(msg: Message) -> crate::Result<Self> {
        Self::from_sdk(msg, ChatMediaProtocol::Unsupported)
    }
}

impl types::ChatMessage {
    fn from_sdk(msg: Message, protocol: ChatMediaProtocol) -> crate::Result<Self> {
        msg.reject_native_tool_protocol("openai_chat_completions")?;
        Ok(match msg {
            Message::System(s) => types::ChatMessage {
                role: types::Role::System,
                content: Some(types::ChatMessageContent::text(s.content)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            Message::User(u) => types::ChatMessage {
                role: types::Role::User,
                content: Some(types::ChatMessageContent::from_sdk(u, protocol)?),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            Message::Assistant(a) => match a.content {
                LanguageModelResponseContentType::Text(text) => types::ChatMessage {
                    role: types::Role::Assistant,
                    content: Some(types::ChatMessageContent::text(text)),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                LanguageModelResponseContentType::ToolCall(tool_info) => types::ChatMessage {
                    role: types::Role::Assistant,
                    content: Some(types::ChatMessageContent::text("")),
                    name: None,
                    tool_calls: Some(vec![types::ToolCall {
                        id: tool_info.tool.id.clone(),
                        type_: "function".to_string(),
                        function: types::FunctionCall {
                            name: tool_info.tool.name.clone(),
                            arguments: tool_info.input.to_string(),
                        },
                    }]),
                    tool_call_id: None,
                },
                LanguageModelResponseContentType::Reasoning { content, .. } => {
                    // Chat Completions doesn't have separate reasoning messages
                    // Include as text with prefix
                    types::ChatMessage {
                        role: types::Role::Assistant,
                        content: Some(types::ChatMessageContent::text(format!(
                            "[Reasoning]: {content}"
                        ))),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    }
                }
                _ => types::ChatMessage {
                    role: types::Role::Assistant,
                    content: None,
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
            },
            Message::Tool(tool_result) => types::ChatMessage {
                role: types::Role::Tool,
                content: Some(types::ChatMessageContent::text(
                    tool_result
                        .output
                        .unwrap_or_else(|e| serde_json::Value::String(e.to_string()))
                        .to_string(),
                )),
                name: Some(tool_result.tool.name),
                tool_calls: None,
                tool_call_id: Some(tool_result.tool.id),
            },
            Message::ToolSearchOutput(_) => {
                return Err(crate::Error::InvalidInput(
                    "openai_chat_completions does not support the native tool protocol".into(),
                ));
            }
            Message::Developer(d) => types::ChatMessage {
                role: types::Role::Developer,
                content: Some(types::ChatMessageContent::text(d)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        })
    }
}

impl types::ChatMessageContent {
    fn from_sdk(message: UserMessage, protocol: ChatMediaProtocol) -> crate::Result<Self> {
        if let [UserContentPart::Text(text)] = message.content.as_slice() {
            return Ok(Self::Text(text.clone()));
        }

        let parts = message
            .content
            .into_iter()
            .map(|part| {
                Ok(match part {
                    UserContentPart::Text(text) => types::ChatMessageContentPart::Text { text },
                    UserContentPart::Image(image) => types::ChatMessageContentPart::ImageUrl {
                        image_url: types::ChatMessageImageUrl {
                            url: image.image_url,
                        },
                    },
                    UserContentPart::Audio(audio) => protocol.audio_part(audio)?,
                    UserContentPart::Video(video) => protocol.video_part(video)?,
                })
            })
            .collect::<crate::Result<_>>()?;
        Ok(Self::Parts(parts))
    }
}

/// Protocols supported by specific Chat Completions deployments.
///
/// OpenAI accepts raw base64 for `input_audio`. DashScope accepts a data URL
/// or a public HTTPS URL for `input_audio` and `video_url` where supported.
/// See the [OpenAI](https://developers.openai.com/api/reference/resources/chat/subresources/completions/methods/create)
/// and [DashScope](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-chat-completions)
/// Chat Completions references.
#[derive(Clone, Copy)]
enum ChatMediaProtocol {
    Unsupported,
    OpenAiAudio,
    DashScopeVideo,
    DashScopeOmni,
}

impl ChatMediaProtocol {
    fn for_model(settings: &OpenAIChatCompletionsSettings, model: &str) -> Self {
        let provider_name = settings.provider_name.to_ascii_lowercase();
        let provider = match provider_name.as_str() {
            "openaicompatible"
                if reqwest::Url::parse(&settings.base_url).is_ok_and(|url| {
                    url.scheme() == "https" && url.host_str() == Some("api.openai.com")
                }) =>
            {
                "openai"
            }
            name => name,
        };
        match (provider, model) {
            ("openai" | "openai-chat", "gpt-audio" | "gpt-audio-mini" | "gpt-audio-2025-08-28") => {
                Self::OpenAiAudio
            }
            ("alibaba" | "alibaba-cn" | "alibabacn", "qwen3.8-omni-flash") => Self::DashScopeOmni,
            (
                "alibaba" | "alibaba-cn" | "alibabacn",
                "qwen3-vl-plus" | "qwen3-vl-30b-a3b" | "qwen3.5-flash" | "qwen3.5-plus"
                | "qwen3.6-flash" | "qwen3.6-plus" | "qwen3.7-flash" | "qwen3.7-plus"
                | "qwen3.8-flash" | "qwen3.8-max" | "qwen4-max",
            ) => Self::DashScopeVideo,
            _ => Self::Unsupported,
        }
    }

    fn audio_part(self, audio: UserAudio) -> crate::Result<types::ChatMessageContentPart> {
        if !matches!(self, Self::OpenAiAudio | Self::DashScopeOmni) {
            return Err(crate::Error::InvalidInput(
                "this Chat Completions deployment does not support native audio input".into(),
            ));
        }

        audio.source.validate(&audio.mime_type, "audio")?;
        let (format, mime_type) = match audio.mime_type.to_ascii_lowercase().as_str() {
            "audio/wav" | "audio/x-wav" | "audio/wave" => {
                (types::ChatAudioFormat::Wav, "audio/wav")
            }
            "audio/mp3" | "audio/mpeg" => (types::ChatAudioFormat::Mp3, "audio/mpeg"),
            _ => {
                return Err(crate::Error::InvalidInput(
                    "Chat Completions audio input requires WAV or MP3".into(),
                ));
            }
        };
        let data = match (self, audio.source) {
            (Self::OpenAiAudio, UserMediaSource::Bytes(bytes)) => STANDARD.encode(bytes),
            (Self::OpenAiAudio, UserMediaSource::Url(_)) => {
                return Err(crate::Error::InvalidInput(
                    "OpenAI Chat Completions audio input requires inline bytes".into(),
                ));
            }
            (_, source) => media_url(source, mime_type),
        };
        Ok(types::ChatMessageContentPart::InputAudio {
            input_audio: types::ChatMessageInputAudio { data, format },
        })
    }

    fn video_part(self, video: UserVideo) -> crate::Result<types::ChatMessageContentPart> {
        if !matches!(self, Self::DashScopeVideo | Self::DashScopeOmni) {
            return Err(crate::Error::InvalidInput(
                "this Chat Completions deployment does not support native video input".into(),
            ));
        }

        video.source.validate(&video.mime_type, "video")?;
        Ok(types::ChatMessageContentPart::VideoUrl {
            video_url: types::ChatMessageVideoUrl {
                url: media_url(video.source, &video.mime_type),
            },
        })
    }
}

fn media_url(source: UserMediaSource, mime_type: &str) -> String {
    match source {
        UserMediaSource::Bytes(bytes) => {
            format!("data:{mime_type};base64,{}", STANDARD.encode(bytes))
        }
        UserMediaSource::Url(url) => url,
    }
}

// ============================================================================
// SDK Tool -> ChatCompletions Tool
// ============================================================================

impl From<SdkTool> for types::Tool {
    fn from(tool: SdkTool) -> Self {
        let mut params = tool.input_schema.to_value();

        // Remove schema metadata fields that may conflict with strict mode
        if let serde_json::Value::Object(ref mut obj) = params {
            obj.remove("$schema");
            obj.remove("title");
        }

        // Ensure required fields for OpenAI Chat Completions
        params["type"] = serde_json::Value::String("object".to_string());
        params["additionalProperties"] = serde_json::Value::Bool(false);

        if !params
            .get("properties")
            .map(|p| p.is_object())
            .unwrap_or(false)
        {
            params["properties"] = serde_json::Value::Object(serde_json::Map::new());
        }

        types::Tool {
            type_: "function".to_string(),
            function: types::FunctionDefinition {
                name: tool.name,
                description: Some(tool.description),
                parameters: params,
                strict: Some(true),
            },
        }
    }
}

// ============================================================================
// ChatCompletions Usage -> SDK Usage
// ============================================================================

impl From<types::Usage> for Usage {
    fn from(usage: types::Usage) -> Self {
        Self {
            input_tokens: Some(usage.prompt_tokens as usize),
            output_tokens: Some(usage.completion_tokens as usize),
            reasoning_tokens: usage
                .completion_tokens_details
                .map(|d| d.reasoning_tokens as usize),
            cached_tokens: usage
                .prompt_tokens_details
                .map(|d| d.cached_tokens as usize),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::language_model::ReasoningEffort;
    use crate::core::tools::{Tool, ToolExecute, ToolList};
    use schemars::{JsonSchema, schema_for};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct StructuredOutput {
        answer: String,
    }

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct SumInput {
        a: i32,
        b: i32,
    }

    #[test]
    fn test_message_conversion_system() {
        let msg = Message::System("You are helpful".to_string().into());
        let chat_msg: types::ChatMessage = msg.try_into().unwrap();

        assert_eq!(chat_msg.role, types::Role::System);
        assert_eq!(
            chat_msg.content,
            Some(types::ChatMessageContent::Text(
                "You are helpful".to_string()
            ))
        );
        assert!(chat_msg.tool_calls.is_none());
    }

    #[test]
    fn test_message_conversion_user() {
        let msg = Message::User("Hello".to_string().into());
        let chat_msg: types::ChatMessage = msg.try_into().unwrap();

        assert_eq!(chat_msg.role, types::Role::User);
        assert_eq!(
            chat_msg.content,
            Some(types::ChatMessageContent::Text("Hello".to_string()))
        );
    }

    #[test]
    fn test_message_conversion_multimodal_user() {
        let msg = Message::User(
            UserMessage::new("Describe this image").with_image_url("https://example.com/cat.png"),
        );
        let chat_msg: types::ChatMessage = msg.try_into().unwrap();

        assert_eq!(chat_msg.role, types::Role::User);
        assert_eq!(
            chat_msg.content,
            Some(types::ChatMessageContent::Parts(vec![
                types::ChatMessageContentPart::Text {
                    text: "Describe this image".to_string(),
                },
                types::ChatMessageContentPart::ImageUrl {
                    image_url: types::ChatMessageImageUrl {
                        url: "https://example.com/cat.png".to_string(),
                    },
                },
            ]))
        );
    }

    #[test]
    fn test_stop_sequences_single() {
        let options = LanguageModelOptions {
            stop_sequences: Some(vec!["STOP".to_string()]),
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.try_into().unwrap();
        assert!(matches!(
            completions_opts.stop,
            Some(types::StopSequences::Single(_))
        ));
    }

    #[test]
    fn test_stop_sequences_multiple_truncated() {
        let options = LanguageModelOptions {
            stop_sequences: Some(vec![
                "S1".to_string(),
                "S2".to_string(),
                "S3".to_string(),
                "S4".to_string(),
                "S5".to_string(),
            ]),
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.try_into().unwrap();
        if let Some(types::StopSequences::Multiple(seqs)) = completions_opts.stop {
            assert_eq!(seqs.len(), 4);
        }
    }

    #[test]
    fn test_scalar_request_options_map_to_chat_completions_body() {
        let options = LanguageModelOptions {
            system: Some("You are helpful".to_string()),
            messages: vec![Message::User("Hello".to_string().into()).into()],
            temperature: Some(70),
            top_p: Some(90),
            seed: Some(42),
            frequency_penalty: Some(0.5),
            stop_sequences: Some(vec!["END".to_string()]),
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.try_into().unwrap();

        assert!(
            completions_opts
                .temperature
                .is_some_and(|value| (value - 0.7).abs() < f32::EPSILON)
        );
        assert!(
            completions_opts
                .top_p
                .is_some_and(|value| (value - 0.9).abs() < f32::EPSILON)
        );
        assert_eq!(completions_opts.seed, Some(42));
        assert_eq!(completions_opts.frequency_penalty, Some(0.5));
        assert_eq!(completions_opts.messages.len(), 2);
        assert_eq!(completions_opts.messages[0].role, types::Role::System);
        assert!(matches!(
            completions_opts.messages[0].content.as_ref(),
            Some(types::ChatMessageContent::Text(text)) if text == "You are helpful"
        ));
        assert_eq!(completions_opts.messages[1].role, types::Role::User);
        assert!(matches!(
            completions_opts.messages[1].content.as_ref(),
            Some(types::ChatMessageContent::Text(text)) if text == "Hello"
        ));
        assert!(matches!(
            completions_opts.stop,
            Some(types::StopSequences::Single(sequence)) if sequence == "END"
        ));
    }

    #[test]
    fn test_multimodal_user_message_maps_to_openai_compatible_chat_body() {
        let options = LanguageModelOptions {
            messages: vec![
                Message::User(
                    UserMessage::new("Describe this image")
                        .with_image_url("https://example.com/cat.png"),
                )
                .into(),
            ],
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.try_into().unwrap();
        let request = serde_json::to_value(&completions_opts).expect("request should serialize");

        assert_eq!(request["messages"][0]["role"], json!("user"));
        assert_eq!(
            request["messages"][0]["content"][0],
            json!({
                "type": "text",
                "text": "Describe this image"
            })
        );
        assert_eq!(
            request["messages"][0]["content"][1],
            json!({
                "type": "image_url",
                "image_url": { "url": "https://example.com/cat.png" }
            })
        );
    }

    #[test]
    fn test_schema_and_reasoning_options_map_to_chat_completions_body() {
        let options = LanguageModelOptions {
            schema: Some(schema_for!(StructuredOutput)),
            reasoning_effort: Some(ReasoningEffort::High),
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.try_into().unwrap();

        let Some(types::ResponseFormat::JsonSchema { json_schema }) =
            completions_opts.response_format
        else {
            panic!("expected json schema response format");
        };

        assert_eq!(json_schema.name, "StructuredOutput");
        assert_eq!(json_schema.strict, Some(true));
        assert_eq!(json_schema.schema["additionalProperties"], json!(false));
        assert!(json_schema.schema["properties"].get("answer").is_some());
        assert_eq!(completions_opts.reasoning_effort.as_deref(), Some("high"));
    }

    #[test]
    fn test_tools_map_to_chat_completions_body() {
        let tool = Tool::builder()
            .name("sum")
            .description("Adds two numbers")
            .input_schema(schema_for!(SumInput))
            .execute(ToolExecute::from_sync(|_, _| Ok("3".to_string())))
            .build()
            .expect("tool should build");

        let options = LanguageModelOptions {
            tools: Some(ToolList::new(vec![tool])),
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.try_into().unwrap();
        let tools = completions_opts.tools.expect("tools should be present");
        let tool = &tools[0];

        assert_eq!(tools.len(), 1);
        assert_eq!(tool.function.name, "sum");
        assert_eq!(
            tool.function.description.as_deref(),
            Some("Adds two numbers")
        );
        assert_eq!(tool.function.strict, Some(true));
        assert_eq!(tool.function.parameters["type"], json!("object"));
        assert_eq!(
            tool.function.parameters["additionalProperties"],
            json!(false)
        );
        assert!(tool.function.parameters["properties"].get("a").is_some());
        assert!(tool.function.parameters["properties"].get("b").is_some());
        assert!(matches!(
            completions_opts.tool_choice,
            Some(types::ToolChoice::String(choice)) if choice == "auto"
        ));
        assert_eq!(completions_opts.parallel_tool_calls, Some(true));
    }

    #[test]
    fn test_usage_conversion() {
        let usage = types::Usage {
            prompt_tokens: 100,
            completion_tokens: 50,
            total_tokens: 150,
            prompt_tokens_details: Some(types::PromptTokensDetails {
                cached_tokens: 20,
                audio_tokens: None,
            }),
            completion_tokens_details: Some(types::CompletionTokensDetails {
                reasoning_tokens: 10,
                audio_tokens: None,
                accepted_prediction_tokens: None,
                rejected_prediction_tokens: None,
            }),
        };

        let sdk_usage: Usage = usage.into();
        assert_eq!(sdk_usage.input_tokens, Some(100));
        assert_eq!(sdk_usage.output_tokens, Some(50));
        assert_eq!(sdk_usage.cached_tokens, Some(20));
        assert_eq!(sdk_usage.reasoning_tokens, Some(10));
    }
}

#[cfg(test)]
#[path = "media_input_tests.rs"]
mod media_input_tests;
