//! Video jobs for deployments that implement the OpenAI-compatible video API.

use super::OpenAI;
use crate::core::capabilities::ModelName;
use crate::core::client::merge_headers;
use crate::core::utils::join_url;
use crate::core::video_model::{
    VideoContent, VideoJob, VideoJobStatus, VideoModel, VideoModelError, VideoModelOptions,
};
use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HOST, HeaderMap, HeaderValue};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response, StatusCode, Url};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::time::Duration;

const MAX_REFERENCE_BYTES: usize = 20 * 1024 * 1024;
const MAX_METADATA_BYTES: usize = 64 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Optional video operations explicitly supported by a deployment.
#[derive(Debug, Clone, Copy, Default)]
pub struct OpenAIVideoCapabilities {
    /// Accepts a multipart image reference when submitting a job.
    pub image_reference: bool,
    /// Supports `DELETE` on active jobs as a cancellation operation.
    pub cancel: bool,
    /// Serves video bytes through the job's `/content` endpoint.
    pub content: bool,
}

/// OpenAI-compatible video adapter bound to a model and deployment.
#[derive(Clone)]
pub struct OpenAIVideoModel {
    client: Client,
    collection: Url,
    headers: HeaderMap,
    model: String,
    capabilities: OpenAIVideoCapabilities,
}

impl<M: ModelName> OpenAI<M> {
    /// Bind this provider to a deployment known to support the video job protocol.
    ///
    /// The provider's path, when set, is the video collection path relative to its
    /// base URL. Otherwise the collection path defaults to `/v1/videos`, reusing
    /// an existing `/v1` suffix on the base URL. All job actions use this collection.
    pub fn video_model(
        &self,
        capabilities: OpenAIVideoCapabilities,
    ) -> Result<OpenAIVideoModel, VideoModelError> {
        let base = http_url(&self.settings.base_url)
            .filter(|url| url.query().is_none())
            .ok_or(VideoModelError::InvalidInput("invalid deployment base URL"))?;
        let collection_path = self.settings.path.as_deref().unwrap_or_else(|| {
            if base.path().trim_end_matches('/').ends_with("/v1") {
                "videos"
            } else {
                "v1/videos"
            }
        });
        if collection_path.trim_matches('/').is_empty()
            || collection_path.starts_with("//")
            || collection_path.contains(['?', '#', '\\', ':'])
            || collection_path.chars().any(char::is_control)
        {
            return Err(VideoModelError::InvalidInput(
                "invalid video collection path",
            ));
        }
        let collection = join_url(base, collection_path.trim_end_matches('/'))
            .map_err(|_| VideoModelError::InvalidInput("invalid video collection path"))?;

        if self.lm_options.model.trim().is_empty() || self.settings.api_key.trim().is_empty() {
            return Err(VideoModelError::InvalidInput(
                "model ID and API key are required",
            ));
        }
        let mut headers = HeaderMap::new();
        let bearer = HeaderValue::from_str(&format!("Bearer {}", self.settings.api_key))
            .map_err(|_| VideoModelError::InvalidInput("invalid API key"))?;
        headers.insert(AUTHORIZATION, bearer);
        let mut headers = merge_headers(headers, self.settings.headers.as_ref(), None)
            .map_err(|_| VideoModelError::InvalidInput("invalid deployment headers"))?;
        if headers.contains_key(HOST) {
            return Err(VideoModelError::InvalidInput(
                "deployment headers contain Host",
            ));
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

        Ok(OpenAIVideoModel {
            client,
            collection,
            headers,
            model: self.lm_options.model.clone(),
            capabilities,
        })
    }
}

#[derive(Serialize)]
struct SubmitRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<&'a str>,
}

#[derive(Deserialize)]
struct JobResponse {
    id: Option<String>,
    task_id: Option<String>,
    status: Option<String>,
    progress: Option<f64>,
    url: Option<String>,
    video_url: Option<String>,
}

impl JobResponse {
    fn into_job(self, requested_id: Option<&str>) -> Result<VideoJob, VideoModelError> {
        let id = self
            .id
            .filter(|id| !id.is_empty())
            .or(self.task_id)
            .or_else(|| requested_id.map(str::to_owned))
            .filter(|id| valid_job_id(id))
            .ok_or(VideoModelError::InvalidResponse(
                "missing or invalid task ID",
            ))?;
        if requested_id.is_some_and(|requested| requested != id) {
            return Err(VideoModelError::InvalidResponse("task ID mismatch"));
        }

        let status = match self
            .status
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "queued" | "pending" => VideoJobStatus::Queued,
            "in_progress" | "processing" | "running" => VideoJobStatus::InProgress,
            "completed" | "succeeded" => VideoJobStatus::Succeeded,
            "failed" | "error" => VideoJobStatus::Failed,
            "canceled" | "cancelled" => VideoJobStatus::Cancelled,
            "timeout" | "timed_out" | "expired" => VideoJobStatus::TimedOut,
            _ => VideoJobStatus::Unknown,
        };
        let result_url = [self.url, self.video_url]
            .into_iter()
            .flatten()
            .find_map(|raw| http_url(&raw));
        Ok(VideoJob {
            id,
            status,
            progress: self.progress,
            result_url,
        })
    }
}

#[derive(Deserialize)]
struct CancellationResponse {
    id: Option<String>,
    task_id: Option<String>,
    deleted: Option<bool>,
    status: Option<String>,
}

#[async_trait]
impl VideoModel for OpenAIVideoModel {
    async fn submit_video(&self, input: VideoModelOptions) -> Result<VideoJob, VideoModelError> {
        if input.prompt.trim().is_empty() {
            return Err(VideoModelError::InvalidInput("video prompt is required"));
        }
        if input.seconds == Some(0) {
            return Err(VideoModelError::InvalidInput("duration must be positive"));
        }
        let body = SubmitRequest {
            model: &self.model,
            prompt: &input.prompt,
            seconds: input.seconds.map(|seconds| seconds.to_string()),
            size: input.size.as_deref(),
        };
        let request = self
            .client
            .post(self.collection.clone())
            .headers(self.headers.clone());
        let request = if let Some(reference) = input.input_reference {
            if !self.capabilities.image_reference {
                return Err(VideoModelError::Unsupported("image_reference"));
            }
            if reference.bytes.is_empty() || reference.bytes.len() > MAX_REFERENCE_BYTES {
                return Err(VideoModelError::InvalidInput(
                    "reference image must be 1 byte to 20 MiB",
                ));
            }
            let name = match reference.media_type.as_str() {
                "image/jpeg" => "reference.jpg",
                "image/png" => "reference.png",
                "image/webp" => "reference.webp",
                _ => return Err(VideoModelError::Unsupported("reference image media type")),
            };
            let part = Part::bytes(reference.bytes)
                .file_name(name)
                .mime_str(&reference.media_type)
                .map_err(|_| VideoModelError::InvalidInput("invalid reference image media type"))?;
            let mut form = Form::new()
                .text("model", body.model.to_owned())
                .text("prompt", body.prompt.to_owned())
                .part("input_reference", part);
            if let Some(seconds) = body.seconds {
                form = form.text("seconds", seconds);
            }
            if let Some(size) = body.size {
                form = form.text("size", size.to_owned());
            }
            request.multipart(form)
        } else {
            request.json(&body)
        };
        let request = request
            .build()
            .map_err(|_| VideoModelError::InvalidInput("invalid video submit request"))?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|_| VideoModelError::SubmissionUnknown)?;
        let status = response.status();
        if !status.is_success() {
            return Err(match status {
                StatusCode::METHOD_NOT_ALLOWED | StatusCode::NOT_IMPLEMENTED => {
                    VideoModelError::Unsupported("submit")
                }
                status if status.is_client_error() && status != StatusCode::REQUEST_TIMEOUT => {
                    VideoModelError::Http(status)
                }
                _ => VideoModelError::SubmissionUnknown,
            });
        }
        read_metadata::<JobResponse>(response)
            .await
            .and_then(|job| job.into_job(None))
            .map_err(|_| VideoModelError::SubmissionUnknown)
    }

    async fn video_status(&self, id: &str) -> Result<VideoJob, VideoModelError> {
        let response = self
            .client
            .get(self.job_url(id, false)?)
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(transport_error)?;
        check_status(response.status(), "status")?;
        read_metadata::<JobResponse>(response)
            .await?
            .into_job(Some(id))
    }

    async fn cancel_video(&self, id: &str) -> Result<(), VideoModelError> {
        if !self.capabilities.cancel {
            return Err(VideoModelError::Unsupported("cancel"));
        }
        let response = self
            .client
            .delete(self.job_url(id, false)?)
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(transport_error)?;
        check_status(response.status(), "cancel")?;
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(());
        }

        let ack = read_metadata::<CancellationResponse>(response).await?;
        if ack
            .id
            .or(ack.task_id)
            .as_deref()
            .is_some_and(|ack_id| ack_id != id)
        {
            return Err(VideoModelError::InvalidResponse(
                "cancellation task ID mismatch",
            ));
        }
        let cancelled = ack.status.as_deref().is_some_and(|status| {
            status.eq_ignore_ascii_case("cancelled") || status.eq_ignore_ascii_case("canceled")
        });
        if ack.deleted == Some(true) || cancelled {
            return Ok(());
        }
        Err(VideoModelError::InvalidResponse(
            "cancellation was not confirmed",
        ))
    }

    async fn read_video_content(
        &self,
        id: &str,
        max_bytes: usize,
    ) -> Result<VideoContent, VideoModelError> {
        if !self.capabilities.content {
            return Err(VideoModelError::Unsupported("content"));
        }
        if max_bytes == 0 {
            return Err(VideoModelError::InvalidInput(
                "content limit must be positive",
            ));
        }

        let response = self
            .client
            .get(self.job_url(id, true)?)
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(transport_error)?;
        check_status(response.status(), "content")?;
        if response.status() != StatusCode::OK {
            return Err(VideoModelError::InvalidResponse(
                "expected a complete video body",
            ));
        }
        let media_type = match response.headers().get(CONTENT_TYPE) {
            Some(value) => value
                .to_str()
                .map_err(|_| VideoModelError::InvalidResponse("invalid content type"))?
                .split(';')
                .next()
                .unwrap_or_default()
                .trim()
                .to_ascii_lowercase(),
            None => "application/octet-stream".into(),
        };
        if !media_type.starts_with("video/") && media_type != "application/octet-stream" {
            return Err(VideoModelError::InvalidResponse(
                "unexpected video content type",
            ));
        }
        let bytes = read_bounded(response, max_bytes).await?;
        if bytes.is_empty() {
            return Err(VideoModelError::InvalidResponse("empty video body"));
        }
        Ok(VideoContent { media_type, bytes })
    }
}

impl OpenAIVideoModel {
    fn job_url(&self, id: &str, content: bool) -> Result<Url, VideoModelError> {
        if !valid_job_id(id) {
            return Err(VideoModelError::InvalidInput("invalid video task ID"));
        }
        let mut url = self.collection.clone();
        let mut path = url
            .path_segments_mut()
            .map_err(|_| VideoModelError::InvalidInput("invalid video collection URL"))?;
        path.pop_if_empty().push(id);
        if content {
            path.push("content");
        }
        drop(path);
        Ok(url)
    }
}

fn valid_job_id(id: &str) -> bool {
    !id.is_empty()
        && id.trim() == id
        && !matches!(id, "." | "..")
        && !id.chars().any(char::is_control)
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

fn transport_error(error: reqwest::Error) -> VideoModelError {
    if error.is_timeout() {
        VideoModelError::Timeout
    } else {
        VideoModelError::Transport
    }
}

fn check_status(status: StatusCode, operation: &'static str) -> Result<(), VideoModelError> {
    match status {
        status if status.is_success() => Ok(()),
        StatusCode::METHOD_NOT_ALLOWED | StatusCode::NOT_IMPLEMENTED => {
            Err(VideoModelError::Unsupported(operation))
        }
        StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => Err(VideoModelError::Timeout),
        status => Err(VideoModelError::Http(status)),
    }
}

async fn read_metadata<T: DeserializeOwned>(response: Response) -> Result<T, VideoModelError> {
    let body = read_bounded(response, MAX_METADATA_BYTES)
        .await
        .map_err(|error| match error {
            VideoModelError::ContentTooLarge { .. } => {
                VideoModelError::InvalidResponse("job metadata is too large")
            }
            other => other,
        })?;
    serde_json::from_slice(&body)
        .map_err(|_| VideoModelError::InvalidResponse("invalid job metadata"))
}

async fn read_bounded(mut response: Response, limit: usize) -> Result<Vec<u8>, VideoModelError> {
    if response
        .content_length()
        .is_some_and(|length| length > limit as u64)
    {
        return Err(VideoModelError::ContentTooLarge { limit });
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(transport_error)? {
        if chunk.len() > limit - bytes.len() {
            return Err(VideoModelError::ContentTooLarge { limit });
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
