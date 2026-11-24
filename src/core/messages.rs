//! Message types for the `aisdk` library.

use crate::core::{
    language_model::{LanguageModelResponseContentType, Usage},
    tools::{ToolCallInfo, ToolResultInfo},
};

/// The detail level requested when a provider inspects an input image.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UserImageDetail {
    /// Let the provider pick the most appropriate detail level.
    #[default]
    Auto,
    /// Prefer a low-detail representation to reduce latency or cost.
    Low,
    /// Prefer a high-detail representation for richer visual analysis.
    High,
}

/// A user-supplied image referenced by URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserImage {
    /// The remote URL that points to the image content.
    pub image_url: String,
    /// Optional explicit media type for providers that require it.
    pub media_type: Option<String>,
    /// The requested provider-specific detail level.
    pub detail: UserImageDetail,
}

impl UserImage {
    /// Creates a new image input referenced by URL.
    pub fn new(image_url: impl Into<String>) -> Self {
        Self {
            image_url: image_url.into(),
            media_type: None,
            detail: UserImageDetail::Auto,
        }
    }

    /// Sets the explicit image media type.
    pub fn media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    /// Sets the preferred detail level for the image.
    pub fn detail(mut self, detail: UserImageDetail) -> Self {
        self.detail = detail;
        self
    }

    pub(crate) fn fallback_text(&self) -> String {
        format!("[image: {}]", self.image_url)
    }

    #[allow(dead_code)]
    pub(crate) fn inline_data(&self) -> Option<(String, String)> {
        let data_url = self.image_url.strip_prefix("data:")?;
        let (metadata, data) = data_url.split_once(',')?;
        let (media_type, encoding) = metadata.split_once(';')?;

        if media_type.is_empty() || !encoding.eq_ignore_ascii_case("base64") || data.is_empty() {
            return None;
        }

        Some((
            self.media_type
                .clone()
                .unwrap_or_else(|| media_type.to_string()),
            data.to_string(),
        ))
    }

    #[allow(dead_code)]
    pub(crate) fn resolved_media_type(&self) -> Option<String> {
        self.media_type
            .clone()
            .or_else(|| self.inline_data().map(|(media_type, _)| media_type))
            .or_else(|| infer_image_media_type_from_url(&self.image_url).map(str::to_string))
    }
}

#[allow(dead_code)]
fn infer_image_media_type_from_url(url: &str) -> Option<&'static str> {
    let path = url.split(['?', '#']).next().unwrap_or(url);
    let extension = path.rsplit('.').next()?.to_ascii_lowercase();

    match extension.as_str() {
        "jpg" | "jpeg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "gif" => Some("image/gif"),
        "webp" => Some("image/webp"),
        "bmp" => Some("image/bmp"),
        "tif" | "tiff" => Some("image/tiff"),
        "heic" => Some("image/heic"),
        "heif" => Some("image/heif"),
        "avif" => Some("image/avif"),
        "svg" => Some("image/svg+xml"),
        _ => None,
    }
}

/// Source of a native audio or video input.
#[derive(Clone, PartialEq, Eq)]
pub enum UserMediaSource {
    /// Encoded media bytes in the declared MIME type.
    Bytes(Vec<u8>),
    /// An HTTPS URL accessible to the provider.
    Url(String),
}

impl std::fmt::Debug for UserMediaSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bytes(bytes) => f.debug_struct("Bytes").field("len", &bytes.len()).finish(),
            Self::Url(_) => f.write_str("Url(..)"),
        }
    }
}

#[cfg(any(feature = "google", feature = "openaichatcompletions"))]
impl UserMediaSource {
    pub(crate) fn validate(&self, mime_type: &str, kind: &str) -> crate::Result<()> {
        let valid_mime = mime_type.split_once('/').is_some_and(|(ty, subtype)| {
            ty.eq_ignore_ascii_case(kind)
                && !subtype.is_empty()
                && subtype.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'+' | b'_')
                })
        });
        if !valid_mime {
            return Err(crate::Error::InvalidInput(format!(
                "expected a valid {kind} MIME type"
            )));
        }

        match self {
            Self::Bytes(bytes) if bytes.is_empty() => {
                Err(crate::Error::InvalidInput(format!("{kind} input is empty")))
            }
            Self::Url(url)
                if !reqwest::Url::parse(url).is_ok_and(|url| {
                    url.scheme() == "https"
                        && url.host().is_some()
                        && url.username().is_empty()
                        && url.password().is_none()
                }) =>
            {
                Err(crate::Error::InvalidInput(format!(
                    "{kind} input requires a valid HTTPS URL"
                )))
            }
            _ => Ok(()),
        }
    }
}

/// A native user audio input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAudio {
    /// MIME type of the encoded audio, such as `audio/wav`.
    pub mime_type: String,
    /// Audio bytes or an HTTPS reference.
    pub source: UserMediaSource,
}

impl UserAudio {
    /// Creates an audio input from encoded bytes and their MIME type.
    pub fn from_bytes(mime_type: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            mime_type: mime_type.into(),
            source: UserMediaSource::Bytes(bytes.into()),
        }
    }

    /// Creates an audio input from an HTTPS URL and its MIME type.
    pub fn from_url(mime_type: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            mime_type: mime_type.into(),
            source: UserMediaSource::Url(url.into()),
        }
    }
}

/// A native user video input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserVideo {
    /// MIME type of the encoded video, such as `video/mp4`.
    pub mime_type: String,
    /// Video bytes or an HTTPS reference.
    pub source: UserMediaSource,
}

impl UserVideo {
    /// Creates a video input from encoded bytes and their MIME type.
    pub fn from_bytes(mime_type: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            mime_type: mime_type.into(),
            source: UserMediaSource::Bytes(bytes.into()),
        }
    }

    /// Creates a video input from an HTTPS URL and its MIME type.
    pub fn from_url(mime_type: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            mime_type: mime_type.into(),
            source: UserMediaSource::Url(url.into()),
        }
    }
}

/// A single content part in a user message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserContentPart {
    /// Plain text content.
    Text(String),
    /// An image referenced by URL.
    Image(UserImage),
    /// A native audio input.
    Audio(UserAudio),
    /// A native video input.
    Video(UserVideo),
}

impl UserContentPart {
    /// Creates a text content part.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    /// Creates an image content part.
    pub fn image(image: UserImage) -> Self {
        Self::Image(image)
    }

    #[allow(dead_code)]
    pub(crate) fn fallback_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Image(image) => image.fallback_text(),
            Self::Audio(_) => "[audio]".to_string(),
            Self::Video(_) => "[video]".to_string(),
        }
    }
}

impl From<String> for UserContentPart {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for UserContentPart {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<UserImage> for UserContentPart {
    fn from(value: UserImage) -> Self {
        Self::Image(value)
    }
}

impl From<UserAudio> for UserContentPart {
    fn from(value: UserAudio) -> Self {
        Self::Audio(value)
    }
}

impl From<UserVideo> for UserContentPart {
    fn from(value: UserVideo) -> Self {
        Self::Video(value)
    }
}

/// The role of a participant in a conversation.
#[derive(Debug, Clone)]
pub enum Role {
    /// System-level instructions or context.
    System,
    /// Human user input.
    User,
    /// AI assistant response.
    Assistant,
}

/// A message in a conversation with a language model.
#[derive(Debug, Clone)]
pub enum Message {
    /// A system message providing context or instructions.
    System(SystemMessage),
    /// A user message containing input from the human.
    User(UserMessage),
    /// An assistant message containing the model's response.
    Assistant(AssistantMessage),
    /// A tool result message from executing a tool call.
    Tool(ToolResultInfo),
    /// A developer-specific message for advanced use cases.
    Developer(String),
    /// Dedicated native search output, never an ordinary function result.
    ToolSearchOutput(crate::core::tools::ToolSearchOutput),
}

/// A List of `Message`s
pub type Messages = Vec<Message>;

impl Message {
    /// Reject native history before conversion to a provider without native support.
    #[cfg(any(
        feature = "anthropic",
        feature = "google",
        feature = "openaichatcompletions"
    ))]
    pub(crate) fn reject_native_tool_protocol(&self, provider: &str) -> crate::Result<()> {
        let native = match self {
            Self::ToolSearchOutput(_) => true,
            Self::Assistant(a) => match &a.content {
                LanguageModelResponseContentType::ToolSearchCall(_)
                | LanguageModelResponseContentType::ToolSearchOutput(_) => true,
                LanguageModelResponseContentType::ToolCall(c) => c.tool.namespace.is_some(),
                _ => false,
            },
            Self::Tool(t) => t.tool.namespace.is_some(),
            _ => false,
        };
        if native {
            return Err(crate::Error::InvalidInput(format!(
                "{provider} does not support the native tool protocol"
            )));
        }
        Ok(())
    }

    /// Start a new conversation with an empty message list.
    ///
    /// Returns a `MessageBuilder<Conversation>`, allowing any number of
    /// `user`/`assistant` calls without forcing an initial system or user message.
    ///
    /// # Example
    /// ```
    /// use aisdk::core::Message;
    ///
    /// let mut msg = Message::conversation_builder();
    /// for _ in 0..10 {
    ///     msg = msg.user("hello");
    /// }
    /// let messages = msg.build();
    /// ```
    pub fn conversation_builder() -> MessageBuilder<Conversation> {
        MessageBuilder::conversation_builder()
    }

    /// Create a new message builder in the initial state.
    ///
    /// Returns a `MessageBuilder<Initial>` that enforces type-safe order:
    /// the first message **must** be either a system prompt or a user message.
    /// After that, the builder transitions to `Conversation` state and allows
    /// free mixing of user/assistant messages but not system prompts.
    ///
    /// # Example
    /// ```
    /// use aisdk::core::Message;
    ///
    /// let msgs = Message::builder()
    ///     .system("You are helpful.")
    ///     .user("Hello!")
    ///     .assistant("Hi there.")
    ///     .build();
    /// ```
    pub fn builder() -> MessageBuilder<Initial> {
        MessageBuilder::default()
    }
}

/// A system message that provides context or instructions to the model.
#[derive(Debug, Clone)]
pub struct SystemMessage {
    /// The text content of the system message.
    pub content: String,
}

impl SystemMessage {
    /// Creates a new system message with the given content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl From<String> for SystemMessage {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SystemMessage {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// A user message containing input from the human participant.
#[derive(Debug, Clone)]
pub struct UserMessage {
    /// The content parts of the user message.
    pub content: Vec<UserContentPart>,
}

impl UserMessage {
    /// Creates a new user message with a single text part.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: vec![UserContentPart::Text(content.into())],
        }
    }

    /// Creates a new user message from structured content parts.
    pub fn from_parts(content: impl Into<Vec<UserContentPart>>) -> Self {
        let mut content = content.into();
        if content.is_empty() {
            content.push(UserContentPart::Text(String::new()));
        }
        Self { content }
    }

    /// Appends an additional text part to the user message.
    pub fn with_text(mut self, content: impl Into<String>) -> Self {
        self.content.push(UserContentPart::Text(content.into()));
        self
    }

    /// Appends an image part to the user message.
    pub fn with_image(mut self, image: UserImage) -> Self {
        self.content.push(UserContentPart::Image(image));
        self
    }

    /// Appends an image URL part to the user message.
    pub fn with_image_url(self, image_url: impl Into<String>) -> Self {
        self.with_image(UserImage::new(image_url))
    }

    /// Appends a native audio part to the user message.
    pub fn with_audio(mut self, audio: UserAudio) -> Self {
        self.content.push(audio.into());
        self
    }

    /// Appends a native video part to the user message.
    pub fn with_video(mut self, video: UserVideo) -> Self {
        self.content.push(video.into());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn fallback_text(&self) -> String {
        self.content
            .iter()
            .map(UserContentPart::fallback_text)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<String> for UserMessage {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for UserMessage {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<Vec<UserContentPart>> for UserMessage {
    fn from(value: Vec<UserContentPart>) -> Self {
        Self::from_parts(value)
    }
}

/// A message generated by the language model assistant.
#[derive(Default, Debug, Clone)]
pub struct AssistantMessage {
    /// The content of the assistant's response.
    pub content: LanguageModelResponseContentType,
    /// Optional usage statistics for the response.
    pub usage: Option<Usage>,
}

impl From<String> for AssistantMessage {
    fn from(value: String) -> Self {
        Self {
            content: value.into(),
            usage: None,
        }
    }
}

impl AssistantMessage {
    /// Creates a new assistant message with the given content and usage.
    pub fn new(content: LanguageModelResponseContentType, usage: Option<Usage>) -> Self {
        Self { content, usage }
    }
}

/// Type-state marker for the initial message builder state.
///
/// In this state, the first message must be either a system prompt or a user message.
#[derive(Debug, Clone)]
pub struct Initial;

/// Type-state marker for the conversation message builder state.
///
/// In this state, only user and assistant messages can be added.
#[derive(Debug, Clone)]
pub struct Conversation;

/// A type-state builder for constructing message lists safely.
///
/// This builder ensures that messages are added in a valid order,
/// preventing invalid conversation structures.
#[derive(Debug, Clone)]
pub struct MessageBuilder<State = Initial> {
    messages: Messages,
    state: std::marker::PhantomData<State>,
}

impl MessageBuilder {
    /// Creates a new message builder starting in the conversation state.
    ///
    /// This allows building conversations without requiring an initial system or user message.
    pub fn conversation_builder() -> MessageBuilder<Conversation> {
        MessageBuilder {
            messages: Vec::new(),
            state: std::marker::PhantomData,
        }
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        MessageBuilder {
            messages: Vec::new(),
            state: std::marker::PhantomData,
        }
    }
}

impl<State> MessageBuilder<State> {
    /// Builds the message list.
    pub fn build(self) -> Messages {
        self.messages
    }
}

impl MessageBuilder<Initial> {
    /// Adds a system message and transitions to the conversation state.
    ///
    /// # Parameters
    ///
    /// * `content` - The system message content.
    ///
    /// # Returns
    ///
    /// The builder in the conversation state.
    pub fn system(mut self, content: impl Into<String>) -> MessageBuilder<Conversation> {
        self.messages.push(Message::System(content.into().into()));
        MessageBuilder {
            messages: self.messages,
            state: std::marker::PhantomData,
        }
    }

    /// Adds a user message and transitions to the conversation state.
    ///
    /// # Parameters
    ///
    /// * `content` - The user message content.
    ///
    /// # Returns
    ///
    /// The builder in the conversation state.
    pub fn user(mut self, content: impl Into<String>) -> MessageBuilder<Conversation> {
        self.messages.push(Message::User(content.into().into()));
        MessageBuilder {
            messages: self.messages,
            state: std::marker::PhantomData,
        }
    }

    /// Adds a structured user message and transitions to the conversation state.
    pub fn user_message(mut self, message: UserMessage) -> MessageBuilder<Conversation> {
        self.messages.push(Message::User(message));
        MessageBuilder {
            messages: self.messages,
            state: std::marker::PhantomData,
        }
    }
}

impl MessageBuilder<Conversation> {
    /// Adds a user message to the conversation.
    ///
    /// # Parameters
    ///
    /// * `content` - The user message content.
    ///
    /// # Returns
    ///
    /// The builder with the message added.
    pub fn user(mut self, content: impl Into<String>) -> MessageBuilder<Conversation> {
        self.messages.push(Message::User(content.into().into()));
        MessageBuilder {
            messages: self.messages,
            state: std::marker::PhantomData,
        }
    }

    /// Adds a structured user message to the conversation.
    pub fn user_message(mut self, message: UserMessage) -> MessageBuilder<Conversation> {
        self.messages.push(Message::User(message));
        MessageBuilder {
            messages: self.messages,
            state: std::marker::PhantomData,
        }
    }

    /// Adds an assistant message to the conversation.
    ///
    /// # Parameters
    ///
    /// * `content` - The assistant message content.
    ///
    /// # Returns
    ///
    /// The builder with the message added.
    pub fn assistant(mut self, content: impl Into<String>) -> MessageBuilder<Conversation> {
        self.messages
            .push(Message::Assistant(content.into().into()));
        MessageBuilder {
            messages: self.messages,
            state: std::marker::PhantomData,
        }
    }
}

/// A message tagged with its step id in a list of messages
/// used for tracking steps in a conversation
#[derive(Debug, Clone)]
pub(crate) struct TaggedMessage {
    pub step_id: usize,
    pub message: Message,
}

impl TaggedMessage {
    pub fn new(step_id: usize, message: Message) -> Self {
        Self { step_id, message }
    }

    pub fn initial_step_msg(message: Message) -> Self {
        Self {
            step_id: 0,
            message,
        }
    }
}

// conversions assume message is in initial step
impl From<Message> for TaggedMessage {
    fn from(value: Message) -> Self {
        Self::initial_step_msg(value)
    }
}

// conversions disregard tagging information
impl From<TaggedMessage> for Message {
    fn from(value: TaggedMessage) -> Self {
        value.message
    }
}

/// Helper trait for extracting messages from TaggedMessage collections
pub(crate) trait TaggedMessageHelpers {
    fn extract_tool_calls(&self) -> Option<Vec<ToolCallInfo>>;
    fn extract_tool_results(&self) -> Option<Vec<ToolResultInfo>>;
}

impl TaggedMessageHelpers for [TaggedMessage] {
    fn extract_tool_calls(&self) -> Option<Vec<ToolCallInfo>> {
        let calls: Vec<ToolCallInfo> = self
            .iter()
            .filter_map(|msg| match msg.message {
                Message::Assistant(AssistantMessage {
                    content: LanguageModelResponseContentType::ToolCall(ref tool_info),
                    ..
                }) => Some(tool_info.clone()),
                _ => None,
            })
            .collect();
        if calls.is_empty() { None } else { Some(calls) }
    }

    fn extract_tool_results(&self) -> Option<Vec<ToolResultInfo>> {
        let results: Vec<ToolResultInfo> = self
            .iter()
            .filter_map(|msg| match msg.message {
                Message::Tool(ref info) => Some(info.clone()),
                _ => None,
            })
            .collect();
        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
}
