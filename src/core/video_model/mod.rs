//! Video generation job contracts shared by provider adapters.

use async_trait::async_trait;
use reqwest::{StatusCode, Url};

/// Input for a single video generation submission.
pub struct VideoModelOptions {
    /// Text describing the video to generate.
    pub prompt: String,
    /// Requested duration in seconds, if supported by the model.
    pub seconds: Option<u32>,
    /// Provider-defined output size, such as `1280x720`.
    pub size: Option<String>,
    /// Image data supplied by the caller for image-to-video generation.
    pub input_reference: Option<VideoReferenceImage>,
}

/// An input reference image already loaded by the caller.
pub struct VideoReferenceImage {
    /// Image media type, such as `image/png`.
    pub media_type: String,
    /// Encoded image bytes.
    pub bytes: Vec<u8>,
}

/// State reported by the upstream video service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoJobStatus {
    /// Waiting for generation to start.
    Queued,
    /// Generation is running.
    InProgress,
    /// Generation succeeded.
    Succeeded,
    /// Generation failed.
    Failed,
    /// Generation was cancelled.
    Cancelled,
    /// The upstream job reached its generation deadline.
    TimedOut,
    /// The service has yet to report a recognized state for this job.
    Unknown,
}

/// Snapshot of an upstream video generation job.
#[derive(Clone, PartialEq)]
pub struct VideoJob {
    /// The upstream task identifier.
    pub id: String,
    /// The provider's current job state.
    pub status: VideoJobStatus,
    /// Progress as reported by the provider, if present.
    pub progress: Option<f64>,
    /// An HTTP(S) result URL reported by the provider, if present and well-formed.
    ///
    /// Before downloading, callers must apply their own destination and redirect policy
    /// and use a client without the deployment's credentials.
    pub result_url: Option<Url>,
}

impl std::fmt::Debug for VideoJob {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VideoJob")
            .field("id", &self.id)
            .field("status", &self.status)
            .field("progress", &self.progress)
            .field("has_result_url", &self.result_url.is_some())
            .finish()
    }
}

/// Video content read through the deployment's authenticated content endpoint.
#[derive(PartialEq, Eq)]
pub struct VideoContent {
    /// Content media type.
    pub media_type: String,
    /// Encoded video bytes, bounded by the limit provided by the caller.
    pub bytes: Vec<u8>,
}

impl std::fmt::Debug for VideoContent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VideoContent")
            .field("media_type", &self.media_type)
            .field("byte_count", &self.bytes.len())
            .finish()
    }
}

/// Failure from an operation on a video deployment.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VideoModelError {
    /// An input or deployment setting was invalid.
    #[error("Invalid video input: {0}")]
    InvalidInput(&'static str),
    /// The deployment does not support the operation.
    #[error("Unsupported video operation: {0}")]
    Unsupported(&'static str),
    /// A submission may have reached the service, but returned no usable task ID.
    #[error("Video submission outcome is unknown")]
    SubmissionUnknown,
    /// An HTTP request or response read reached its deadline.
    #[error("Video request timed out")]
    Timeout,
    /// The service returned an unsuccessful HTTP status.
    #[error("Video endpoint returned HTTP {0}")]
    Http(StatusCode),
    /// The request could not be completed due to a transport failure.
    #[error("Video transport failed")]
    Transport,
    /// The service returned an invalid or unconfirmed response.
    #[error("Invalid video response: {0}")]
    InvalidResponse(&'static str),
    /// The result body exceeded the caller's limit.
    #[error("Video content exceeds the {limit}-byte limit")]
    ContentTooLarge {
        /// Maximum accepted response length in bytes.
        limit: usize,
    },
}

/// An explicitly configured video generation deployment.
// async_trait adds #[must_use] to a future that is already must-use.
#[allow(clippy::double_must_use)]
#[async_trait]
pub trait VideoModel: Send + Sync {
    /// Submit once. A submission with an unknown outcome may already have been accepted.
    async fn submit_video(&self, input: VideoModelOptions) -> Result<VideoJob, VideoModelError>;

    /// Fetch the latest state of an upstream job.
    async fn video_status(&self, id: &str) -> Result<VideoJob, VideoModelError>;

    /// Request cancellation and require an upstream acknowledgement.
    async fn cancel_video(&self, id: &str) -> Result<(), VideoModelError>;

    /// Fetch video bytes from the deployment's content endpoint with a size limit.
    async fn read_video_content(
        &self,
        id: &str,
        max_bytes: usize,
    ) -> Result<VideoContent, VideoModelError>;
}
