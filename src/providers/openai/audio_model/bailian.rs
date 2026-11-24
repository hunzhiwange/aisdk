//! Non-streaming CosyVoice protocol.

use super::{OpenAIAudioModel, format_name, http_url, media_type, read_json, send};
use crate::core::audio_model::{
    AudioFormat, AudioModelError, SpeechAudio, SpeechOptions, SpeechResponse,
};
use serde::Deserialize;
use serde_json::json;

const MAX_METADATA_BYTES: usize = 64 * 1024;
const MAX_REFERENCE_BYTES: usize = 8 * 1024;

#[derive(Deserialize)]
struct CosyVoiceOutput {
    finish_reason: Option<String>,
    audio: Option<CosyVoiceAudio>,
}

#[derive(Deserialize)]
struct CosyVoiceAudio {
    url: Option<String>,
    duration: Option<f64>,
}

impl OpenAIAudioModel {
    pub(super) async fn synthesize_cosyvoice(
        &self,
        input: SpeechOptions,
    ) -> Result<SpeechResponse, AudioModelError> {
        if !matches!(
            input.format,
            AudioFormat::Mp3 | AudioFormat::Wav | AudioFormat::Pcm | AudioFormat::Opus
        ) {
            return Err(AudioModelError::Unsupported("CosyVoice audio format"));
        }
        if input
            .speed
            .is_some_and(|speed| !(0.5..=2.0).contains(&speed))
        {
            return Err(AudioModelError::InvalidInput(
                "CosyVoice rate must be between 0.5 and 2.0",
            ));
        }
        let mut body = json!({
            "model": self.model,
            "input": {
                "text": input.text,
                "voice": input.voice,
                "format": format_name(input.format),
            }
        });
        if let Some(speed) = input.speed {
            body["input"]["rate"] = json!(speed);
        }

        let response = send(
            self.post("api/v1", "services/audio/tts/SpeechSynthesizer")?
                .json(&body),
            "speech synthesis",
        )
        .await?;
        let metadata = read_json(response, MAX_METADATA_BYTES).await?;
        if metadata.get("code").is_some_and(|code| !code.is_null()) {
            return Err(AudioModelError::ProviderRejected);
        }
        let output: CosyVoiceOutput = serde_json::from_value(
            metadata.get("output").cloned().unwrap_or_default(),
        )
        .map_err(|_| AudioModelError::InvalidResponse("missing or invalid CosyVoice output"))?;
        if output
            .finish_reason
            .as_deref()
            .is_some_and(|reason| reason != "stop")
        {
            return Err(AudioModelError::InvalidResponse(
                "incomplete CosyVoice synthesis",
            ));
        }
        let audio = output.audio.ok_or(AudioModelError::InvalidResponse(
            "missing or invalid CosyVoice audio URL",
        ))?;
        let url = audio
            .url
            .as_deref()
            .filter(|url| url.len() <= MAX_REFERENCE_BYTES)
            .and_then(http_url)
            .filter(|url| url.as_str().len() <= MAX_REFERENCE_BYTES)
            .ok_or(AudioModelError::InvalidResponse(
                "missing or invalid CosyVoice audio URL",
            ))?;
        Ok(SpeechResponse {
            audio: SpeechAudio::Url(url),
            format: input.format,
            media_type: media_type(input.format).into(),
            duration_seconds: audio.duration.filter(|duration| *duration >= 0.0),
        })
    }
}
