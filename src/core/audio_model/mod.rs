//! Speech synthesis and file transcription contracts shared by provider adapters.

use async_trait::async_trait;
use reqwest::{StatusCode, Url};
use serde::Deserialize;
use serde_json::Value;
use std::fmt;

/// Encodings available for synthesized speech, subject to deployment support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    /// MPEG Layer III.
    Mp3,
    /// Opus.
    Opus,
    /// Advanced Audio Coding.
    Aac,
    /// Free Lossless Audio Codec.
    Flac,
    /// RIFF Waveform Audio.
    Wav,
    /// Headerless pulse-code modulation.
    Pcm,
}

/// Input for a single speech synthesis request.
pub struct SpeechOptions {
    /// Text to synthesize.
    pub text: String,
    /// Provider voice or voice ID.
    pub voice: String,
    /// Requested encoding.
    pub format: AudioFormat,
    /// Speaking rate relative to the provider's normal speed.
    pub speed: Option<f64>,
    /// Maximum number of audio bytes to accept from an inline response.
    pub max_bytes: usize,
}

/// Audio returned inline or made available as a separate remote asset.
pub enum SpeechAudio {
    /// Encoded audio from the deployment's response.
    Bytes(Vec<u8>),
    /// A provider-issued HTTP(S) reference that may have a short lifetime.
    ///
    /// Before fetching it, callers must apply destination and redirect policy,
    /// impose their own download limit, and omit deployment credentials.
    Url(Url),
}

impl fmt::Debug for SpeechAudio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(bytes) => formatter
                .debug_struct("Bytes")
                .field("byte_count", &bytes.len())
                .finish(),
            Self::Url(_) => formatter.write_str("Url"),
        }
    }
}

/// Speech synthesis result.
#[derive(Debug)]
pub struct SpeechResponse {
    /// Inline audio or a remote asset reference.
    pub audio: SpeechAudio,
    /// The output encoding.
    pub format: AudioFormat,
    /// Canonical audio media type for the encoding.
    pub media_type: String,
    /// Audio duration in seconds, when reported or derivable.
    pub duration_seconds: Option<f64>,
}

/// Caller-provided audio file, already loaded into memory.
pub struct AudioFile {
    /// Basename sent to the upstream service in the multipart request.
    pub file_name: String,
    /// File media type, such as `audio/wav`.
    pub media_type: String,
    /// Encoded bytes of the file.
    pub bytes: Vec<u8>,
}

/// Response detail requested from a file transcription endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptionFormat {
    /// Standard JSON with transcript text.
    Json,
    /// JSON with additional metadata, when supported by the deployment.
    VerboseJson,
}

/// Input for a single file transcription request.
pub struct TranscriptionOptions {
    /// Audio file to transcribe.
    pub file: AudioFile,
    /// Optional input language hint.
    pub language: Option<String>,
    /// Optional context or vocabulary hint.
    pub prompt: Option<String>,
    /// Requested JSON response detail.
    pub format: TranscriptionFormat,
    /// Request word timestamps in addition to segments in verbose JSON.
    pub word_timestamps: bool,
}

/// File transcription result and metadata supplied by the deployment.
#[derive(Deserialize)]
pub struct TranscriptionResponse {
    /// Transcribed text. An empty transcript can represent silence.
    pub text: String,
    /// Detected or reported language.
    pub language: Option<String>,
    /// Reported language breakdown, when the deployment returns multiple languages.
    pub languages: Option<Vec<Value>>,
    /// Audio duration in seconds, when reported.
    #[serde(rename = "duration")]
    pub duration_seconds: Option<f64>,
    /// Provider task label, when reported.
    pub task: Option<String>,
    /// Transcript segments with timestamps and any provider-specific fields.
    pub segments: Option<Vec<Value>>,
    /// Transcript words with timestamps and any provider-specific fields.
    pub words: Option<Vec<Value>>,
    /// Token log probabilities and associated provider metadata, when reported.
    pub logprobs: Option<Vec<Value>>,
    /// Provider usage metadata, when reported.
    pub usage: Option<Value>,
}

impl fmt::Debug for TranscriptionResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TranscriptionResponse")
            .field("text_bytes", &self.text.len())
            .field("duration_seconds", &self.duration_seconds)
            .field("segments", &self.segments.as_ref().map(Vec::len))
            .field("words", &self.words.as_ref().map(Vec::len))
            .finish_non_exhaustive()
    }
}

/// Failure from an operation on an audio deployment.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AudioModelError {
    /// An input or deployment setting was invalid.
    #[error("Invalid audio input: {0}")]
    InvalidInput(&'static str),
    /// The deployment does not support the operation or option.
    #[error("Unsupported audio operation or option: {0}")]
    Unsupported(&'static str),
    /// The deployment rejected the request in a successful HTTP response.
    #[error("Audio provider rejected the request")]
    ProviderRejected,
    /// The service returned an unsuccessful HTTP status.
    #[error("Audio endpoint returned HTTP {0}")]
    Http(StatusCode),
    /// The request reached its deadline.
    #[error("Audio request timed out")]
    Timeout,
    /// The request failed at the transport layer.
    #[error("Audio transport failed")]
    Transport,
    /// The service returned an invalid response.
    #[error("Invalid audio response: {0}")]
    InvalidResponse(&'static str),
    /// The response body exceeded its byte limit.
    #[error("Audio response exceeds the {limit}-byte limit")]
    ContentTooLarge {
        /// Maximum accepted response length in bytes.
        limit: usize,
    },
}

/// A deployment supporting speech synthesis, file transcription, or both.
///
/// Dropping the operation future stops local request processing. The upstream
/// service may continue processing a request it already received.
// async_trait adds #[must_use] to a future that is already must-use.
#[allow(clippy::double_must_use)]
#[async_trait]
pub trait AudioModel: Send + Sync {
    /// Submit one speech synthesis request.
    async fn synthesize_speech(
        &self,
        input: SpeechOptions,
    ) -> Result<SpeechResponse, AudioModelError>;

    /// Submit one audio file for transcription.
    async fn transcribe_audio(
        &self,
        input: TranscriptionOptions,
    ) -> Result<TranscriptionResponse, AudioModelError>;
}
