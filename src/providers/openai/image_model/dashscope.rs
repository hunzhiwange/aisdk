//! Image generation through the native DashScope Wan asynchronous endpoint.

use super::{
    IMAGE_GENERATION_TIMEOUT, OpenAIImageResponse, OpenAIImageUsage, image_prompt,
    image_references, image_response_error, image_results, image_sources, merge_extra_body,
    multimodal_request, send_multimodal_request, valid_provider_id,
};
use crate::core::capabilities::ModelName;
use crate::core::image_model::{ImageModelOptions, ImageModelResponse, ImageModelWarning};
use crate::error::{Error, Result};
use crate::providers::openai::OpenAI;
use reqwest::Url;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderValue, TRANSFER_ENCODING};
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;
use tokio::time::{Instant, sleep, timeout_at};

pub(super) const SUBMIT_PATH: &str = "api/v1/services/aigc/image-generation/generation";
const ASYNC_HEADER: &str = "x-dashscope-async";
const POLL_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Deserialize)]
struct TaskResponse {
    output: Option<TaskOutput>,
    request_id: Option<String>,
    code: Option<String>,
    usage: Option<OpenAIImageUsage>,
}

#[derive(Deserialize)]
struct TaskOutput {
    task_id: Option<String>,
    task_status: Option<String>,
    code: Option<String>,
    choices: Option<Vec<Value>>,
    results: Option<Vec<Value>>,
}

impl TaskOutput {
    fn image_urls(&self) -> Vec<&str> {
        let choices = self
            .choices
            .iter()
            .flatten()
            .filter_map(|choice| choice.pointer("/message/content")?.as_array());
        choices
            .flatten()
            .filter_map(|part| part.get("image")?.as_str())
            .chain(
                self.results
                    .iter()
                    .flatten()
                    .filter_map(|result| result.get("url")?.as_str()),
            )
            .collect()
    }
}

impl TaskResponse {
    fn output(&self) -> Result<&TaskOutput> {
        if let Some(code) = self.code.as_deref().filter(|code| !code.is_empty()) {
            let code = if valid_provider_id(code) {
                code
            } else {
                "unknown"
            };
            return Err(self.error(&format!("provider error {code}")));
        }
        self.output
            .as_ref()
            .ok_or_else(|| self.error("response has no task output"))
    }

    fn error(&self, details: &str) -> Error {
        let trace = self
            .request_id
            .as_deref()
            .filter(|id| valid_provider_id(id))
            .map_or_else(String::new, |id| format!(" (request {id})"));
        image_response_error(&format!("DashScope image {details}{trace}"))
    }
}

pub(super) async fn generate<M: ModelName>(
    model: &OpenAI<M>,
    path: &str,
    input: ImageModelOptions,
) -> Result<ImageModelResponse> {
    let prompt = image_prompt(&input)?;
    if input.n == Some(0) {
        return Err(Error::InvalidInput("image count must be positive".into()));
    }
    let mut content: Vec<_> = image_references(&input.files)?
        .into_iter()
        .map(|image| json!({ "image": image }))
        .collect();
    content.push(json!({ "text": prompt }));

    let mut parameters = serde_json::Map::new();
    parameters.insert("n".into(), json!(input.n.unwrap_or(1)));
    let mut warnings = Vec::new();
    if let Some(size) = input
        .size
        .as_deref()
        .map(str::trim)
        .filter(|size| !size.is_empty())
    {
        let normalized = size.split_once(['x', 'X']).filter(|(width, height)| {
            [width, height].iter().all(|dimension| {
                !dimension.is_empty() && dimension.bytes().all(|byte| byte.is_ascii_digit())
            })
        });
        let size = normalized.map_or_else(|| size.to_owned(), |(w, h)| format!("{w}*{h}"));
        parameters.insert("size".into(), json!(size));
    } else if let Some(ratio) = input.aspect_ratio.as_deref() {
        if let Some(size) = aspect_size(ratio.trim()) {
            parameters.insert("size".into(), json!(size));
        } else {
            warnings.push(ImageModelWarning::Unsupported {
                feature: "aspect_ratio",
                details: Some("Use size to specify an explicit image resolution.".into()),
            });
        }
    }
    if let Some(seed) = input.seed {
        parameters.insert("seed".into(), json!(seed));
    }

    let mut body = merge_extra_body(model.settings.body.as_ref(), input.body.as_ref());
    if let Some(overrides) = body.remove("parameters") {
        let Value::Object(overrides) = overrides else {
            return Err(Error::InvalidInput(
                "DashScope parameters must be an object".into(),
            ));
        };
        parameters.extend(overrides);
    }
    parameters.remove("stream");
    body.entry("model")
        .or_insert_with(|| json!(model.lm_options.model));
    body.entry("input").or_insert_with(|| {
        json!({
            "messages": [{"role": "user", "content": content}]
        })
    });
    body.insert("parameters".into(), Value::Object(parameters));

    let (client, submit_url, mut headers) = multimodal_request(model, path, &input)?;
    let tasks_url = task_collection_url(&submit_url)?;
    let deadline = Instant::now() + IMAGE_GENERATION_TIMEOUT;
    headers.insert(ASYNC_HEADER, HeaderValue::from_static("enable"));
    let mut response: TaskResponse = send_multimodal_request(
        &client,
        client.post(submit_url).headers(headers.clone()).json(&body),
        "DashScope image submission",
    )
    .await?;

    headers.remove(ASYNC_HEADER);
    headers.remove(CONTENT_TYPE);
    headers.remove(CONTENT_LENGTH);
    headers.remove(TRANSFER_ENCODING);
    let mut task_id: Option<String> = None;
    let result = loop {
        let output = response
            .output()
            .map_err(|error| with_task_id(error, task_id.as_deref()))?;
        if let Some(id) = &task_id {
            if output
                .task_id
                .as_deref()
                .is_some_and(|returned| returned != id)
            {
                return Err(with_task_id(response.error("task ID mismatch"), Some(id)));
            }
        } else {
            task_id = output
                .task_id
                .as_deref()
                .filter(|id| valid_provider_id(id))
                .map(str::to_owned);
        }

        let status = output.task_status.as_deref().map(str::to_ascii_uppercase);
        if matches!(status.as_deref(), Some("SUCCEEDED") | None) {
            let urls = output.image_urls();
            if status.is_some() || !urls.is_empty() {
                let data =
                    image_sources(urls).map_err(|error| with_task_id(error, task_id.as_deref()))?;
                let usage = response.usage.filter(|usage| {
                    usage.input_tokens.is_some()
                        || usage.output_tokens.is_some()
                        || usage.total_tokens.is_some()
                });
                break OpenAIImageResponse { data, usage };
            }
        }
        match status.as_deref() {
            Some("PENDING" | "RUNNING") | None => {}
            Some(status) => {
                let status = if valid_provider_id(status) {
                    status
                } else {
                    "UNKNOWN"
                };
                let code = output
                    .code
                    .as_deref()
                    .filter(|code| valid_provider_id(code));
                let code = code.map_or_else(String::new, |code| format!(" with code {code}"));
                let error = response.error(&format!("task ended with status {status}{code}"));
                return Err(with_task_id(error, task_id.as_deref()));
            }
        }

        let id = task_id
            .as_deref()
            .ok_or_else(|| response.error("submission returned no valid task ID"))?;
        let task_url = tasks_url
            .join(id)
            .map_err(|_| image_response_error("invalid DashScope image task URL"))?;
        response = timeout_at(deadline, async {
            sleep(POLL_INTERVAL).await;
            send_multimodal_request(
                &client,
                client.get(task_url).headers(headers.clone()),
                "DashScope image status request",
            )
            .await
        })
        .await
        .map_err(|_| timeout_error(Some(id)))?
        .map_err(|error| with_task_id(error, Some(id)))?;
    };
    timeout_at(
        deadline,
        image_results(result, warnings, &model.settings.base_url),
    )
    .await
    .map_err(|_| timeout_error(task_id.as_deref()))?
    .map_err(|error| with_task_id(error, task_id.as_deref()))
}

fn task_collection_url(submit: &Url) -> Result<Url> {
    let prefix = submit
        .path()
        .trim_end_matches('/')
        .strip_suffix(SUBMIT_PATH)
        .filter(|prefix| prefix.ends_with('/'))
        .ok_or_else(|| Error::InvalidInput("invalid DashScope image submission path".into()))?;
    let mut tasks = submit.clone();
    tasks.set_path(&format!("{prefix}api/v1/tasks/"));
    tasks.set_query(None);
    tasks.set_fragment(None);
    Ok(tasks)
}

fn timeout_error(task_id: Option<&str>) -> Error {
    with_task_id(
        image_response_error("DashScope image generation timed out"),
        task_id,
    )
}

fn with_task_id(error: Error, task_id: Option<&str>) -> Error {
    match (error, task_id) {
        (
            Error::ApiError {
                status_code,
                details,
            },
            Some(id),
        ) => Error::ApiError {
            status_code,
            details: format!("{details} (DashScope task {id})"),
        },
        (error, _) => error,
    }
}

fn aspect_size(ratio: &str) -> Option<&'static str> {
    match ratio {
        "1:1" => Some("1024*1024"),
        "16:9" => Some("1280*720"),
        "9:16" => Some("720*1280"),
        "4:3" => Some("1024*768"),
        "3:4" => Some("768*1024"),
        "3:2" => Some("1248*832"),
        "2:3" => Some("832*1248"),
        _ => None,
    }
}
