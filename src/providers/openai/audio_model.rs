//! Speech synthesis and file transcription for explicitly configured deployments.

mod bailian;

use super::OpenAI;
use crate::core::audio_model::{
    AudioFormat, AudioModel, AudioModelError, SpeechAudio, SpeechOptions, SpeechResponse,
    TranscriptionFormat, TranscriptionOptions, TranscriptionResponse,
};
use crate::core::capabilities::ModelName;
use crate::core::client::merge_headers;
use crate::core::utils::join_url;
use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HOST, HeaderMap, HeaderValue};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, RequestBuilder, Response, StatusCode, Url};
use serde_json::{Value, json};
use std::time::Duration;

const MAX_SPEECH_BYTES: usize = 32 * 1024 * 1024;
const MAX_SPEECH_TEXT_BYTES: usize = 16 * 1024;
const MAX_FILE_BYTES: usize = 25 * 1024 * 1024;
const MAX_TRANSCRIPT_BYTES: usize = 2 * 1024 * 1024;
const MAX_PROMPT_BYTES: usize = 4096;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Audio protocol and operations available on the selected deployment.
#[derive(Debug, Clone, Copy)]
pub enum OpenAIAudioProtocol {
    /// OpenAI-compatible HTTP audio endpoints.
    OpenAICompatible {
        /// The model accepts single-request speech synthesis.
        speech: bool,
        /// The model accepts file transcription. Set to false for realtime-only ASR.
        file_transcription: bool,
    },
    /// Bailian's non-streaming CosyVoice HTTP endpoint.
    BailianCosyVoice,
}

/// Audio adapter bound to a model and deployment.
#[derive(Clone)]
pub struct OpenAIAudioModel {
    client: Client,
    base: Url,
    headers: HeaderMap,
    model: String,
    protocol: OpenAIAudioProtocol,
}

impl<M: ModelName> OpenAI<M> {
    /// Bind this provider to an explicitly selected audio protocol.
    ///
    /// The protocol determines the endpoints relative to the base URL. A
    /// language-model path override does not affect audio routing. Extra JSON
    /// body fields are unsupported.
    pub fn audio_model(
        &self,
        protocol: OpenAIAudioProtocol,
    ) -> Result<OpenAIAudioModel, AudioModelError> {
        let base = http_url(&self.settings.base_url)
            .filter(|url| url.query().is_none())
            .ok_or(AudioModelError::InvalidInput("invalid deployment base URL"))?;
        if self.lm_options.model.trim().is_empty() || self.settings.api_key.trim().is_empty() {
            return Err(AudioModelError::InvalidInput(
                "model ID and API key are required",
            ));
        }
        if self
            .settings
            .body
            .as_ref()
            .is_some_and(|body| !body.is_empty())
        {
            return Err(AudioModelError::Unsupported("extra audio body fields"));
        }

        let mut headers = HeaderMap::new();
        let bearer = HeaderValue::from_str(&format!("Bearer {}", self.settings.api_key))
            .map_err(|_| AudioModelError::InvalidInput("invalid API key"))?;
        headers.insert(AUTHORIZATION, bearer);
        let mut headers = merge_headers(headers, self.settings.headers.as_ref(), None)
            .map_err(|_| AudioModelError::InvalidInput("invalid deployment headers"))?;
        if headers.contains_key(HOST) {
            return Err(AudioModelError::InvalidInput(
                "deployment headers contain Host",
            ));
        }
        if matches!(protocol, OpenAIAudioProtocol::BailianCosyVoice)
            && headers.contains_key("x-dashscope-sse")
        {
            return Err(AudioModelError::Unsupported("CosyVoice SSE header"));
        }
        headers.remove(CONTENT_TYPE);
        for value in headers.values_mut() {
            value.set_sensitive(true);
        }

        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(transport_error)?;
        Ok(OpenAIAudioModel {
            client,
            base,
            headers,
            model: self.lm_options.model.clone(),
            protocol,
        })
    }
}

#[async_trait]
impl AudioModel for OpenAIAudioModel {
    async fn synthesize_speech(
        &self,
        input: SpeechOptions,
    ) -> Result<SpeechResponse, AudioModelError> {
        if matches!(
            self.protocol,
            OpenAIAudioProtocol::OpenAICompatible { speech: false, .. }
        ) {
            return Err(AudioModelError::Unsupported("speech synthesis"));
        }
        if input.text.trim().is_empty() || input.text.len() > MAX_SPEECH_TEXT_BYTES {
            return Err(AudioModelError::InvalidInput(
                "speech text must be nonblank and at most 16 KiB",
            ));
        }
        if input.voice.trim().is_empty() || input.voice.len() > 256 {
            return Err(AudioModelError::InvalidInput("invalid speech voice"));
        }
        if !(1..=MAX_SPEECH_BYTES).contains(&input.max_bytes) {
            return Err(AudioModelError::InvalidInput(
                "speech byte limit must be between 1 and 32 MiB",
            ));
        }
        if matches!(self.protocol, OpenAIAudioProtocol::BailianCosyVoice) {
            return self.synthesize_cosyvoice(input).await;
        }
        if input.text.chars().count() > 4096 {
            return Err(AudioModelError::InvalidInput(
                "speech text exceeds 4096 characters",
            ));
        }
        if input
            .speed
            .is_some_and(|speed| !(0.25..=4.0).contains(&speed))
        {
            return Err(AudioModelError::InvalidInput(
                "speech speed must be between 0.25 and 4.0",
            ));
        }

        let mut body = json!({
            "model": self.model,
            "input": input.text,
            "voice": input.voice,
            "response_format": format_name(input.format),
        });
        if let Some(speed) = input.speed {
            body["speed"] = json!(speed);
        }
        let response = send(
            self.post("v1", "audio/speech")?.json(&body),
            "speech synthesis",
        )
        .await?;
        let mime = response_media_type(&response)?;
        if !speech_mime_matches(input.format, &mime) {
            return Err(AudioModelError::InvalidResponse(
                "unexpected speech content type",
            ));
        }
        let bytes = read_bounded(response, input.max_bytes).await?;
        if !matches_audio(input.format, &bytes) {
            return Err(AudioModelError::InvalidResponse(
                "body does not match speech format",
            ));
        }
        let duration_seconds = match input.format {
            AudioFormat::Wav => wav_duration(&bytes),
            AudioFormat::Pcm => Some(bytes.len() as f64 / 48_000.0),
            _ => None,
        };
        Ok(SpeechResponse {
            audio: SpeechAudio::Bytes(bytes),
            format: input.format,
            media_type: media_type(input.format).into(),
            duration_seconds,
        })
    }

    async fn transcribe_audio(
        &self,
        input: TranscriptionOptions,
    ) -> Result<TranscriptionResponse, AudioModelError> {
        if !matches!(
            self.protocol,
            OpenAIAudioProtocol::OpenAICompatible {
                file_transcription: true,
                ..
            }
        ) {
            return Err(AudioModelError::Unsupported("file transcription"));
        }
        if input.word_timestamps && input.format != TranscriptionFormat::VerboseJson {
            return Err(AudioModelError::Unsupported(
                "word timestamps require verbose_json",
            ));
        }
        if input.file.bytes.is_empty() || input.file.bytes.len() > MAX_FILE_BYTES {
            return Err(AudioModelError::InvalidInput(
                "audio file must be 1 byte to 25 MiB",
            ));
        }
        let name = input.file.file_name;
        if name.trim().is_empty()
            || name.len() > 255
            || matches!(name.as_str(), "." | "..")
            || name.contains(['/', '\\'])
            || name.chars().any(char::is_control)
        {
            return Err(AudioModelError::InvalidInput("invalid audio filename"));
        }
        let file_type = input.file.media_type.to_ascii_lowercase();
        if file_type.len() > 128
            || !(file_type.starts_with("audio/")
                || matches!(
                    file_type.as_str(),
                    "video/mp4"
                        | "video/mpeg"
                        | "video/webm"
                        | "application/ogg"
                        | "application/octet-stream"
                ))
        {
            return Err(AudioModelError::Unsupported(
                "transcription file media type",
            ));
        }
        let part = Part::bytes(input.file.bytes)
            .file_name(name)
            .mime_str(&file_type)
            .map_err(|_| AudioModelError::InvalidInput("invalid audio file media type"))?;
        let format = match input.format {
            TranscriptionFormat::Json => "json",
            TranscriptionFormat::VerboseJson => "verbose_json",
        };
        let mut form = Form::new()
            .text("model", self.model.clone())
            .part("file", part)
            .text("response_format", format);
        if let Some(language) = input.language {
            if language.trim().is_empty() || language.len() > 32 {
                return Err(AudioModelError::InvalidInput(
                    "invalid transcription language",
                ));
            }
            form = form.text("language", language);
        }
        if let Some(prompt) = input.prompt {
            if prompt.len() > MAX_PROMPT_BYTES {
                return Err(AudioModelError::InvalidInput(
                    "transcription prompt exceeds 4 KiB",
                ));
            }
            form = form.text("prompt", prompt);
        }
        if input.word_timestamps {
            form = form
                .text("timestamp_granularities[]", "segment")
                .text("timestamp_granularities[]", "word");
        }

        let response = send(
            self.post("v1", "audio/transcriptions")?.multipart(form),
            "file transcription",
        )
        .await?;
        let body = read_json(response, MAX_TRANSCRIPT_BYTES).await?;
        if body.get("error").is_some_and(|error| !error.is_null()) {
            return Err(AudioModelError::ProviderRejected);
        }
        let result: TranscriptionResponse = serde_json::from_value(body)
            .map_err(|_| AudioModelError::InvalidResponse("invalid transcription result"))?;
        if result
            .duration_seconds
            .is_some_and(|duration| duration < 0.0)
        {
            return Err(AudioModelError::InvalidResponse(
                "invalid transcription duration",
            ));
        }
        Ok(result)
    }
}

impl OpenAIAudioModel {
    fn post(&self, version: &str, path: &str) -> Result<RequestBuilder, AudioModelError> {
        let base_path = self.base.path().trim_end_matches('/');
        let versioned_path;
        let path = if base_path
            .strip_suffix(version)
            .is_some_and(|prefix| prefix.ends_with('/'))
        {
            path
        } else {
            versioned_path = format!("{version}/{path}");
            &versioned_path
        };
        let url = join_url(self.base.clone(), path)
            .map_err(|_| AudioModelError::InvalidInput("invalid audio endpoint URL"))?;
        Ok(self.client.post(url).headers(self.headers.clone()))
    }
}

fn format_name(format: AudioFormat) -> &'static str {
    match format {
        AudioFormat::Mp3 => "mp3",
        AudioFormat::Opus => "opus",
        AudioFormat::Aac => "aac",
        AudioFormat::Flac => "flac",
        AudioFormat::Wav => "wav",
        AudioFormat::Pcm => "pcm",
    }
}

fn media_type(format: AudioFormat) -> &'static str {
    match format {
        AudioFormat::Mp3 => "audio/mpeg",
        AudioFormat::Opus => "audio/ogg",
        AudioFormat::Aac => "audio/aac",
        AudioFormat::Flac => "audio/flac",
        AudioFormat::Wav => "audio/wav",
        AudioFormat::Pcm => "audio/pcm",
    }
}

fn response_media_type(response: &Response) -> Result<String, AudioModelError> {
    response.headers().get(CONTENT_TYPE).map_or_else(
        || Ok("application/octet-stream".into()),
        |value| {
            value
                .to_str()
                .map(|value| {
                    value
                        .split(';')
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_ascii_lowercase()
                })
                .map_err(|_| AudioModelError::InvalidResponse("invalid content type"))
        },
    )
}

fn speech_mime_matches(format: AudioFormat, mime: &str) -> bool {
    if mime == "application/octet-stream" || mime == media_type(format) {
        return true;
    }
    match format {
        AudioFormat::Mp3 => matches!(mime, "audio/mp3" | "audio/x-mp3"),
        AudioFormat::Opus => matches!(mime, "audio/opus" | "application/ogg"),
        AudioFormat::Aac => matches!(mime, "audio/x-aac" | "audio/aacp"),
        AudioFormat::Flac => mime == "audio/x-flac",
        AudioFormat::Wav => matches!(mime, "audio/x-wav" | "audio/wave" | "audio/vnd.wave"),
        AudioFormat::Pcm => mime == "audio/x-pcm",
    }
}

fn matches_audio(format: AudioFormat, bytes: &[u8]) -> bool {
    match format {
        AudioFormat::Mp3 => {
            bytes.len() >= 10 && bytes.starts_with(b"ID3")
                || bytes.len() >= 4 && bytes[0] == 0xff && bytes[1] & 0xe6 == 0xe2
        }
        AudioFormat::Opus => {
            bytes.starts_with(b"OggS")
                && bytes
                    .windows(8)
                    .take(256)
                    .any(|window| window == b"OpusHead")
        }
        AudioFormat::Aac => bytes.len() >= 7 && bytes[0] == 0xff && bytes[1] & 0xf6 == 0xf0,
        AudioFormat::Flac => bytes.len() > 4 && bytes.starts_with(b"fLaC"),
        AudioFormat::Wav => bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WAVE"),
        AudioFormat::Pcm => {
            // PCM has no magic bytes. Refuse text resembling an error body and odd sample lengths.
            !bytes.is_empty()
                && bytes.len() % 2 == 0
                && !std::str::from_utf8(bytes).is_ok_and(|text| {
                    text.chars()
                        .all(|character| !character.is_control() || character.is_whitespace())
                })
        }
    }
}

fn wav_duration(bytes: &[u8]) -> Option<f64> {
    let mut chunks = bytes.get(12..)?;
    let mut byte_rate = None;
    while chunks.len() >= 8 {
        let size = u32::from_le_bytes(chunks[4..8].try_into().ok()?) as usize;
        let data = chunks.get(8..)?.get(..size)?;
        match &chunks[..4] {
            b"fmt " => {
                byte_rate = Some(u32::from_le_bytes(data.get(8..12)?.try_into().ok()?))
                    .filter(|&rate| rate != 0);
            }
            b"data" => return byte_rate.map(|rate| size as f64 / f64::from(rate)),
            _ => {}
        }
        chunks = chunks.get(8..)?.get(size.checked_add(size % 2)?..)?;
    }
    None
}

fn http_url(raw: &str) -> Option<Url> {
    let url = Url::parse(raw).ok()?;
    (matches!(url.scheme(), "http" | "https")
        && url.has_host()
        && url.username().is_empty()
        && url.password().is_none()
        && url.fragment().is_none())
    .then_some(url)
}

async fn send(
    request: RequestBuilder,
    operation: &'static str,
) -> Result<Response, AudioModelError> {
    let response = request.send().await.map_err(transport_error)?;
    match response.status() {
        StatusCode::OK => Ok(response),
        StatusCode::METHOD_NOT_ALLOWED | StatusCode::NOT_IMPLEMENTED => {
            Err(AudioModelError::Unsupported(operation))
        }
        StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => Err(AudioModelError::Timeout),
        status if status.is_success() => Err(AudioModelError::InvalidResponse(
            "expected a complete response",
        )),
        status => Err(AudioModelError::Http(status)),
    }
}

fn transport_error(error: reqwest::Error) -> AudioModelError {
    if error.is_timeout() {
        AudioModelError::Timeout
    } else {
        AudioModelError::Transport
    }
}

async fn read_json(response: Response, limit: usize) -> Result<Value, AudioModelError> {
    let mime = response_media_type(&response)?;
    if !matches!(
        mime.as_str(),
        "application/json" | "application/octet-stream"
    ) && !mime.ends_with("+json")
    {
        return Err(AudioModelError::InvalidResponse(
            "unexpected audio metadata content type",
        ));
    }
    let bytes = read_bounded(response, limit).await?;
    serde_json::from_slice(&bytes)
        .map_err(|_| AudioModelError::InvalidResponse("invalid audio metadata JSON"))
}

async fn read_bounded(mut response: Response, limit: usize) -> Result<Vec<u8>, AudioModelError> {
    if response
        .content_length()
        .is_some_and(|length| length > limit as u64)
    {
        return Err(AudioModelError::ContentTooLarge { limit });
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(transport_error)? {
        if chunk.len() > limit - bytes.len() {
            return Err(AudioModelError::ContentTooLarge { limit });
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
