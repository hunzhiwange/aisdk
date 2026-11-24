use super::OpenAI;
use super::audio_model::{OpenAIAudioModel, OpenAIAudioProtocol};
use crate::core::{
    AudioFile, AudioFormat, AudioModel, AudioModelError, SpeechAudio, SpeechOptions,
    TranscriptionFormat, TranscriptionOptions,
};
use reqwest::StatusCode;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;
use wiremock::matchers::{body_json, header, header_regex, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const COMPATIBLE: OpenAIAudioProtocol = OpenAIAudioProtocol::OpenAICompatible {
    speech: true,
    file_transcription: true,
};
const SPEECH_PATH: &str = "/deployment/v1/audio/speech";
const TRANSCRIPTION_PATH: &str = "/deployment/v1/audio/transcriptions";
const COSYVOICE_PATH: &str = "/deployment/api/v1/services/audio/tts/SpeechSynthesizer";
const WAV: &[u8] = b"RIFF\x28\0\0\0WAVEfmt \x10\0\0\0\x01\0\x01\0\x80\xbb\0\0\0\x77\x01\0\x02\0\x10\0data\x04\0\0\0\0\0\0\0";

fn model(base_url: String, name: &str, protocol: OpenAIAudioProtocol) -> OpenAIAudioModel {
    OpenAI::builder()
        .model_name(name)
        .base_url(base_url)
        .api_key("audio-key")
        .build()
        .unwrap()
        .audio_model(protocol)
        .unwrap()
}

fn compatible(server: &MockServer) -> OpenAIAudioModel {
    model(
        format!("{}/deployment/v1/", server.uri()),
        "audio-test",
        COMPATIBLE,
    )
}

fn speech() -> SpeechOptions {
    SpeechOptions {
        text: "A short line.".into(),
        voice: "alloy".into(),
        format: AudioFormat::Wav,
        speed: Some(1.2),
        max_bytes: 1024,
    }
}

fn transcription() -> TranscriptionOptions {
    TranscriptionOptions {
        file: AudioFile {
            file_name: "sample.wav".into(),
            media_type: "audio/wav".into(),
            bytes: WAV.to_vec(),
        },
        language: Some("en".into()),
        prompt: Some("A short line.".into()),
        format: TranscriptionFormat::VerboseJson,
        word_timestamps: true,
    }
}

#[tokio::test]
async fn media_audio_wire_speech_bytes_and_format() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(SPEECH_PATH))
        .and(header("authorization", "Bearer audio-key"))
        .and(body_json(json!({
            "model": "audio-test", "input": "A short line.", "voice": "alloy",
            "response_format": "wav", "speed": 1.2
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(WAV.to_vec(), "Audio/X-Wav; charset=binary"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let result = compatible(&server)
        .synthesize_speech(speech())
        .await
        .unwrap();
    assert_eq!(result.format, AudioFormat::Wav);
    assert_eq!(result.media_type, "audio/wav");
    assert_eq!(result.duration_seconds, Some(4.0 / 96_000.0));
    assert!(matches!(result.audio, SpeechAudio::Bytes(bytes) if bytes.as_slice() == WAV));
}

#[tokio::test]
async fn media_audio_wire_file_transcription_metadata() {
    let server = MockServer::start().await;
    let segments = json!([{"id": 0, "start": 0.0, "end": 1.25, "text": "A short line."}]);
    let words = json!([{"word": "short", "start": 0.2, "end": 0.6}]);
    let usage = json!({"type": "duration", "seconds": 2});
    Mock::given(method("POST"))
        .and(path(TRANSCRIPTION_PATH))
        .and(header("authorization", "Bearer audio-key"))
        .and(header_regex(
            "content-type",
            "multipart/form-data; boundary=.*",
        ))
        .and(|request: &wiremock::Request| {
            let body = String::from_utf8_lossy(&request.body);
            let fields = [
                "name=\"file\"; filename=\"sample.wav\"",
                "Content-Type: audio/wav",
                "name=\"model\"\r\n\r\naudio-test\r\n",
                "name=\"language\"\r\n\r\nen\r\n",
                "name=\"prompt\"\r\n\r\nA short line.\r\n",
                "name=\"response_format\"\r\n\r\nverbose_json\r\n",
                "name=\"timestamp_granularities[]\"\r\n\r\nword\r\n",
                "name=\"timestamp_granularities[]\"\r\n\r\nsegment\r\n",
            ];
            fields.iter().all(|field| body.contains(*field))
                && request.body.windows(WAV.len()).any(|window| window == WAV)
        })
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "text": "A short line.", "language": "english", "duration": 1.25,
            "task": "transcribe", "segments": segments, "words": words, "usage": usage
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = compatible(&server)
        .transcribe_audio(transcription())
        .await
        .unwrap();
    assert_eq!(result.text, "A short line.");
    assert_eq!(result.language.as_deref(), Some("english"));
    assert_eq!(result.duration_seconds, Some(1.25));
    assert_eq!(result.task.as_deref(), Some("transcribe"));
    assert_eq!(json!(result.segments), segments);
    assert_eq!(json!(result.words), words);
    assert_eq!(result.usage, Some(usage));
}

#[tokio::test]
async fn media_audio_wire_bailian_reference_and_unsupported_transcription() {
    let server = MockServer::start().await;
    let assets = MockServer::start().await;
    let url = format!("{}/sample.wav?signature=fixture", assets.uri());
    Mock::given(method("POST"))
        .and(path(COSYVOICE_PATH))
        .and(header("authorization", "Bearer audio-key"))
        .and(body_json(json!({
            "model": "cosyvoice-v2",
            "input": {
                "text": "A short line.", "voice": "longxiaochun_v2", "format": "wav", "rate": 1.2
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "request_id": "request-fixture",
            "output": {"finish_reason": "stop", "audio": {"url": url}}
        })))
        .expect(1)
        .mount(&server)
        .await;
    let model = model(
        format!("{}/deployment/api/v1/", server.uri()),
        "cosyvoice-v2",
        OpenAIAudioProtocol::BailianCosyVoice,
    );

    let result = model
        .synthesize_speech(SpeechOptions {
            voice: "longxiaochun_v2".into(),
            ..speech()
        })
        .await
        .unwrap();
    assert_eq!(result.format, AudioFormat::Wav);
    assert_eq!(result.media_type, "audio/wav");
    assert!(!format!("{result:?}").contains("signature"));
    assert!(matches!(result.audio, SpeechAudio::Url(reference) if reference.as_str() == url));
    assert!(assets.received_requests().await.unwrap().is_empty());
    assert_eq!(
        model.transcribe_audio(transcription()).await.unwrap_err(),
        AudioModelError::Unsupported("file transcription")
    );
    assert_eq!(
        model
            .synthesize_speech(SpeechOptions {
                format: AudioFormat::Flac,
                ..speech()
            })
            .await
            .unwrap_err(),
        AudioModelError::Unsupported("CosyVoice audio format")
    );
}

#[tokio::test]
async fn media_audio_wire_bailian_rejects_invalid_references_and_provider_errors() {
    let server = MockServer::start().await;
    let model = model(
        format!("{}/deployment", server.uri()),
        "cosyvoice-v2",
        OpenAIAudioProtocol::BailianCosyVoice,
    );
    for response in [
        json!({"output": {"audio": {"url": "file:///private/sample.wav"}}}),
        json!({"output": {"audio": {"url": "https://user:password@example.com/sample.wav"}}}),
        json!({"output": {"audio": {"url": format!("https://example.com/{}", "a".repeat(9000))}}}),
    ] {
        Mock::given(method("POST"))
            .and(path(COSYVOICE_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .expect(1)
            .mount(&server)
            .await;
        assert_eq!(
            model.synthesize_speech(speech()).await.unwrap_err(),
            AudioModelError::InvalidResponse("missing or invalid CosyVoice audio URL")
        );
        server.verify().await;
        server.reset().await;
    }
    Mock::given(method("POST"))
        .and(path(COSYVOICE_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": "InvalidParameter", "message": "private provider details"
        })))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        model.synthesize_speech(speech()).await.unwrap_err(),
        AudioModelError::ProviderRejected
    );
}

#[tokio::test]
async fn media_audio_wire_rejects_error_bodies_and_bounds_speech() {
    let server = MockServer::start().await;
    let model = model(
        format!("{}/deployment", server.uri()),
        "audio-test",
        COMPATIBLE,
    );
    let failures = [
        (
            ResponseTemplate::new(429),
            AudioModelError::Http(StatusCode::TOO_MANY_REQUESTS),
        ),
        (ResponseTemplate::new(504), AudioModelError::Timeout),
        (
            ResponseTemplate::new(200).set_body_raw(WAV.to_vec(), "audio/mpeg"),
            AudioModelError::InvalidResponse("unexpected speech content type"),
        ),
        (
            ResponseTemplate::new(200).set_body_raw(br#"{"error":"failed"}"#.to_vec(), "audio/wav"),
            AudioModelError::InvalidResponse("body does not match speech format"),
        ),
        (
            ResponseTemplate::new(200).set_body_raw(vec![0; 1026], "application/octet-stream"),
            AudioModelError::ContentTooLarge { limit: 1024 },
        ),
    ];
    for (response, expected) in failures {
        Mock::given(method("POST"))
            .and(path(SPEECH_PATH))
            .respond_with(response)
            .expect(1)
            .mount(&server)
            .await;
        assert_eq!(
            model.synthesize_speech(speech()).await.unwrap_err(),
            expected
        );
        server.verify().await;
        server.reset().await;
    }
}

#[tokio::test]
async fn media_audio_wire_rejects_unsupported_parameters_before_posting() {
    let server = MockServer::start().await;
    let realtime = model(
        server.uri(),
        "realtime-asr",
        OpenAIAudioProtocol::OpenAICompatible {
            speech: false,
            file_transcription: false,
        },
    );
    assert_eq!(
        realtime
            .transcribe_audio(transcription())
            .await
            .unwrap_err(),
        AudioModelError::Unsupported("file transcription")
    );
    assert_eq!(
        compatible(&server)
            .transcribe_audio(TranscriptionOptions {
                format: TranscriptionFormat::Json,
                ..transcription()
            })
            .await
            .unwrap_err(),
        AudioModelError::Unsupported("word timestamps require verbose_json")
    );
    assert!(matches!(
        compatible(&server)
            .synthesize_speech(SpeechOptions {
                text: "a".repeat(4097),
                ..speech()
            })
            .await
            .unwrap_err(),
        AudioModelError::InvalidInput(_)
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn media_audio_wire_can_cancel_in_flight_request_without_retrying() {
    let server = MockServer::start().await;
    let received = Arc::new(Notify::new());
    let signal = received.clone();
    Mock::given(method("POST"))
        .and(path(SPEECH_PATH))
        .respond_with(move |_: &wiremock::Request| {
            signal.notify_one();
            ResponseTemplate::new(200)
                .set_body_raw(WAV.to_vec(), "audio/wav")
                .set_delay(Duration::from_secs(30))
        })
        .expect(1)
        .mount(&server)
        .await;
    let model = compatible(&server);
    let task = tokio::spawn(async move { model.synthesize_speech(speech()).await });
    tokio::time::timeout(Duration::from_secs(5), received.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn media_audio_wire_transcription_json_accepts_silence_and_rejects_error_envelopes() {
    let server = MockServer::start().await;
    let model = compatible(&server);
    let input = || TranscriptionOptions {
        format: TranscriptionFormat::Json,
        word_timestamps: false,
        ..transcription()
    };

    Mock::given(method("POST"))
        .and(path(TRANSCRIPTION_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"text": ""})))
        .expect(1)
        .mount(&server)
        .await;
    let transcript = model.transcribe_audio(input()).await.unwrap();
    assert!(transcript.text.is_empty());
    assert!(transcript.duration_seconds.is_none());
    server.verify().await;
    server.reset().await;

    Mock::given(method("POST"))
        .and(path(TRANSCRIPTION_PATH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "error": {"message": "provider details"}
        })))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(
        model.transcribe_audio(input()).await.unwrap_err(),
        AudioModelError::ProviderRejected
    );
}
