//! OpenAI 图片模型实现。

use crate::core::capabilities::{ImageOutputSupport, ModelName};
use crate::core::client::{merge_body, merge_headers};
use crate::core::image_model::{
    ImageModel, ImageModelOptions, ImageModelResponse, ImageModelUsage, ImageModelWarning,
};
use crate::core::messages::UserImage;
use crate::core::utils::join_url;
use crate::error::{Error, Result};
use crate::providers::openai::OpenAI;
use async_trait::async_trait;
use base64::Engine as _;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Url, redirect};
use std::net::IpAddr;
use std::time::Duration;

const MAX_IMAGE_BYTES: usize = 32 * 1024 * 1024;
const MAX_RESPONSE_BYTES: usize = 128 * 1024 * 1024;
const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(30);
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
struct OpenAIImageGenerationRequest {
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAIImageResponse {
    data: Vec<OpenAIImageData>,
    usage: Option<OpenAIImageUsage>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAIImageData {
    b64_json: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAIImageUsage {
    input_tokens: Option<usize>,
    output_tokens: Option<usize>,
    total_tokens: Option<usize>,
}

#[async_trait]
impl<M> ImageModel for OpenAI<M>
where
    M: ModelName,
    OpenAI<M>: ImageOutputSupport,
{
    async fn generate_image(&self, input: ImageModelOptions) -> Result<ImageModelResponse> {
        let model_name = self.lm_options.model.clone();
        let warnings = openai_warnings(&input);

        if input.files.is_empty() && input.mask.is_some() {
            return Err(Error::InvalidInput(
                "an edit mask requires an input image".into(),
            ));
        }

        let response: OpenAIImageResponse = if input.files.is_empty() {
            let request = OpenAIImageGenerationRequest {
                model: model_name.clone(),
                prompt: input.prompt.clone(),
                n: input.n,
                size: input.size.clone(),
                response_format: (!has_default_response_format(&model_name))
                    .then_some("b64_json".to_string()),
            };

            let mut default_headers = reqwest::header::HeaderMap::new();
            default_headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
            default_headers.insert(
                "Authorization",
                format!("Bearer {}", self.settings.api_key).parse().unwrap(),
            );

            let headers = merge_headers(
                default_headers,
                self.settings.headers.as_ref(),
                input.headers.as_ref(),
            )?;
            let body = merge_body(&request, self.settings.body.as_ref(), input.body.as_ref())?;
            let url = join_url(
                &self.settings.base_url,
                self.settings
                    .path
                    .as_deref()
                    .unwrap_or("/v1/images/generations"),
            )?;

            send_json_request(url, headers, body).await?
        } else {
            let mut default_headers = reqwest::header::HeaderMap::new();
            default_headers.insert(
                "Authorization",
                format!("Bearer {}", self.settings.api_key).parse().unwrap(),
            );

            let headers = merge_headers(
                default_headers,
                self.settings.headers.as_ref(),
                input.headers.as_ref(),
            )?;

            let mut form = reqwest::multipart::Form::new().text("model", model_name);
            if let Some(prompt) = input.prompt.clone() {
                form = form.text("prompt", prompt);
            }
            if let Some(n) = input.n {
                form = form.text("n", n.to_string());
            }
            if let Some(size) = input.size.clone() {
                form = form.text("size", size);
            }

            for file in &input.files {
                form = form.part(
                    "image",
                    image_to_multipart_part(file, "image", &self.settings.base_url).await?,
                );
            }

            if let Some(mask) = &input.mask {
                form = form.part(
                    "mask",
                    image_to_multipart_part(mask, "mask", &self.settings.base_url).await?,
                );
            }

            for (key, value) in merge_extra_body(self.settings.body.as_ref(), input.body.as_ref()) {
                if value.is_null() {
                    continue;
                }
                form = form.text(key, form_value_to_string(value));
            }

            let url = join_url(
                &self.settings.base_url,
                self.settings.path.as_deref().unwrap_or("/v1/images/edits"),
            )?;

            send_multipart_request(url, headers, form).await?
        };

        if response.data.is_empty() {
            return Err(image_response_error("image response contains no results"));
        }
        let mut images = Vec::new();
        let mut encoded_bytes = 0;
        for item in response.data {
            let image = if let Some(data) = item.b64_json {
                decode_image(&data).map_err(image_response_error)?;
                data
            } else if let Some(url) = item.url {
                let (bytes, _) = download_image(&url, None, &self.settings.base_url).await?;
                base64::engine::general_purpose::STANDARD.encode(bytes)
            } else {
                return Err(image_response_error(
                    "image result contains neither b64_json nor url",
                ));
            };
            encoded_bytes += image.len();
            if encoded_bytes > MAX_RESPONSE_BYTES {
                return Err(image_response_error(
                    "encoded image results exceed the size limit",
                ));
            }
            images.push(image);
        }

        Ok(ImageModelResponse {
            images,
            usage: response.usage.map(|usage| ImageModelUsage {
                input_tokens: usage.input_tokens,
                output_tokens: usage.output_tokens,
                total_tokens: usage.total_tokens,
            }),
            warnings,
        })
    }
}

fn openai_warnings(input: &ImageModelOptions) -> Vec<ImageModelWarning> {
    let mut warnings = Vec::new();

    if input.aspect_ratio.is_some() {
        warnings.push(ImageModelWarning::Unsupported {
            feature: "aspect_ratio",
            details: Some("This model does not support aspect_ratio. Use size instead.".into()),
        });
    }

    if input.seed.is_some() {
        warnings.push(ImageModelWarning::Unsupported {
            feature: "seed",
            details: None,
        });
    }

    warnings
}

fn has_default_response_format(model_name: &str) -> bool {
    const PREFIXES: &[&str] = &[
        "chatgpt-image-",
        "gpt-image-1-mini",
        "gpt-image-1.5",
        "gpt-image-1",
        "gpt-image-2",
    ];

    PREFIXES.iter().any(|prefix| model_name.starts_with(prefix))
}

async fn image_to_multipart_part(
    image: &UserImage,
    fallback_name: &str,
    base_url: &str,
) -> Result<reqwest::multipart::Part> {
    let (bytes, media_type) = load_image_bytes(image, base_url).await?;
    let file_name = infer_file_name(image, fallback_name, &media_type);

    reqwest::multipart::Part::bytes(bytes)
        .file_name(file_name)
        .mime_str(&media_type)
        .map_err(|error| Error::InvalidInput(format!("invalid image media type: {error}")))
}

async fn load_image_bytes(image: &UserImage, base_url: &str) -> Result<(Vec<u8>, String)> {
    if let Some((media_type, data)) = image.inline_data() {
        let media_type = image_media_type(&media_type).ok_or_else(|| {
            Error::InvalidInput("input data URL requires an image MIME type".into())
        })?;
        let bytes = decode_image(&data).map_err(|details| Error::InvalidInput(details.into()))?;
        return Ok((bytes, media_type));
    }

    if image.image_url.starts_with("data:") {
        return Err(Error::InvalidInput("invalid image data URL".into()));
    }
    let media_type = image.resolved_media_type();
    if let Some(media_type) = image.media_type.as_deref()
        && image_media_type(media_type).is_none()
    {
        return Err(Error::InvalidInput(
            "input requires an image MIME type".into(),
        ));
    }
    download_image(&image.image_url, media_type.as_deref(), base_url).await
}

fn decode_image(data: &str) -> std::result::Result<Vec<u8>, &'static str> {
    if data.is_empty() || data.len() > MAX_IMAGE_BYTES.div_ceil(3) * 4 {
        return Err("base64 image is empty or exceeds the size limit");
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|_| "invalid base64 image")?;
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err("base64 image exceeds the size limit");
    }
    Ok(bytes)
}

fn image_media_type(value: &str) -> Option<String> {
    let mime = value.split(';').next()?.trim().to_ascii_lowercase();
    let subtype = mime.strip_prefix("image/")?;
    (!subtype.is_empty()
        && subtype
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"!#$&^_.+-".contains(&byte)))
    .then_some(mime)
}

fn image_response_error(details: &str) -> Error {
    Error::ApiError {
        status_code: None,
        details: details.into(),
    }
}

async fn download_image(
    url: &str,
    fallback_media_type: Option<&str>,
    base_url: &str,
) -> Result<(Vec<u8>, String)> {
    let url = Url::parse(url).map_err(|_| image_response_error("invalid image URL"))?;
    let host = url
        .host_str()
        .ok_or_else(|| image_response_error("invalid image URL host"))?;
    let ip = host.trim_matches(['[', ']']).parse::<IpAddr>().ok();
    // A locally configured provider may serve its own media on the same origin.
    let local_provider = ip.is_some_and(|ip| ip.to_canonical().is_loopback())
        && Url::parse(base_url).is_ok_and(|base| base.origin() == url.origin());

    if (url.scheme() != "https" && !(local_provider && url.scheme() == "http"))
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(image_response_error(
            "image URL requires HTTPS without credentials",
        ));
    }

    let mut client = reqwest::Client::builder()
        .no_proxy()
        .redirect(redirect::Policy::none())
        .timeout(DOWNLOAD_TIMEOUT);
    if let Some(ip) = ip {
        if !local_provider && !is_public_image_address(ip) {
            return Err(image_response_error(
                "image URL points to a non-public address",
            ));
        }
    } else {
        let port = url.port_or_known_default().unwrap_or(443);
        let addresses: Vec<_> =
            tokio::time::timeout(DOWNLOAD_TIMEOUT, tokio::net::lookup_host((host, port)))
                .await
                .ok()
                .and_then(std::result::Result::ok)
                .ok_or_else(|| image_response_error("failed to resolve image URL host"))?
                .collect();
        if addresses.is_empty()
            || addresses
                .iter()
                .any(|addr| !is_public_image_address(addr.ip()))
        {
            return Err(image_response_error(
                "image URL points to a non-public address",
            ));
        }
        // Use the validated DNS answers for the connection as well.
        client = client.resolve_to_addrs(host, &addresses);
    }

    let response = client
        .build()
        .map_err(|_| image_response_error("failed to prepare image download"))?
        .get(url)
        .send()
        .await
        .map_err(|_| image_response_error("failed to download image"))?;
    let status = response.status();
    if !status.is_success() {
        return Err(Error::ApiError {
            status_code: Some(status),
            details: "image download returned an unsuccessful status".into(),
        });
    }

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .map(|value| {
            value
                .to_str()
                .map_err(|_| image_response_error("invalid image download MIME type"))
        })
        .transpose()?;
    let media_type =
        match content_type {
            None => None,
            Some(value)
                if value.split(';').next().is_some_and(|mime| {
                    mime.trim().eq_ignore_ascii_case("application/octet-stream")
                }) =>
            {
                None
            }
            Some(value) => Some(image_media_type(value).ok_or_else(|| {
                image_response_error("image download requires an image MIME type")
            })?),
        };

    let bytes = read_limited_body(response, MAX_IMAGE_BYTES).await?;
    if bytes.is_empty() {
        return Err(image_response_error(
            "image download returned an empty body",
        ));
    }
    let media_type = media_type
        .or_else(|| infer_download_media_type(&bytes).map(str::to_string))
        .or_else(|| fallback_media_type.and_then(image_media_type))
        .ok_or_else(|| image_response_error("image download requires an image MIME type"))?;
    Ok((bytes, media_type))
}

fn infer_download_media_type(bytes: &[u8]) -> Option<&'static str> {
    match bytes {
        [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1a, b'\n', ..] => Some("image/png"),
        [0xff, 0xd8, 0xff, ..] => Some("image/jpeg"),
        [
            b'R',
            b'I',
            b'F',
            b'F',
            _,
            _,
            _,
            _,
            b'W',
            b'E',
            b'B',
            b'P',
            ..,
        ] => Some("image/webp"),
        [b'G', b'I', b'F', b'8', b'7' | b'9', b'a', ..] => Some("image/gif"),
        _ => None,
    }
}

fn is_public_image_address(ip: IpAddr) -> bool {
    match ip.to_canonical() {
        IpAddr::V4(ip) => {
            let [a, b, c, _] = ip.octets();
            !(ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || ip.is_documentation()
                || a == 0
                || a >= 224
                || (a == 100 && (64..=127).contains(&b))
                || (a == 192 && ((b == 0 && c == 0) || (b == 88 && c == 99)))
                || (a == 198 && (18..=19).contains(&b)))
        }
        IpAddr::V6(ip) => {
            let [a, b, ..] = ip.segments();
            // Global unicast only, excluding reserved protocol assignments and transition ranges.
            (a & 0xe000) == 0x2000
                && a != 0x2002
                && (a != 0x2001 || (b >= 0x0200 && b != 0x0db8))
                && (a & 0xfff0) != 0x3ff0
        }
    }
}

async fn read_limited_body(mut response: reqwest::Response, max_bytes: usize) -> Result<Vec<u8>> {
    if response
        .content_length()
        .is_some_and(|length| length > max_bytes as u64)
    {
        return Err(image_response_error(
            "image response exceeds the size limit",
        ));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| image_response_error("failed to read image response body"))?
    {
        if chunk.len() > max_bytes - bytes.len() {
            return Err(image_response_error(
                "image response exceeds the size limit",
            ));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

fn infer_file_name(image: &UserImage, fallback_name: &str, media_type: &str) -> String {
    let url = Url::parse(&image.image_url).ok();
    let candidate = url
        .as_ref()
        .and_then(Url::path_segments)
        .and_then(|mut segments| segments.next_back())
        .unwrap_or_default();

    if !candidate.is_empty() && candidate.contains('.') {
        return candidate.to_string();
    }

    let extension = match media_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        _ => "bin",
    };

    format!("{fallback_name}.{extension}")
}

fn merge_extra_body(
    provider_body: Option<&serde_json::Map<String, serde_json::Value>>,
    request_body: Option<&serde_json::Map<String, serde_json::Value>>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut merged = serde_json::Map::new();

    if let Some(provider_body) = provider_body {
        merged.extend(provider_body.clone());
    }
    if let Some(request_body) = request_body {
        merged.extend(request_body.clone());
    }

    merged
}

fn form_value_to_string(value: serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value,
        other => other.to_string(),
    }
}

async fn send_json_request<T>(
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
    body: reqwest::Body,
) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|error| Error::ApiError {
            status_code: error.status(),
            details: error.to_string(),
        })?;

    parse_json_response(response).await
}

async fn send_multipart_request<T>(
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
    form: reqwest::multipart::Form,
) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .multipart(form)
        .send()
        .await
        .map_err(|error| Error::ApiError {
            status_code: error.status(),
            details: error.to_string(),
        })?;

    parse_json_response(response).await
}

async fn parse_json_response<T>(response: reqwest::Response) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let status = response.status();
    let body = read_limited_body(response, MAX_RESPONSE_BYTES).await?;

    if !status.is_success() {
        return Err(Error::ApiError {
            status_code: Some(status),
            details: String::from_utf8_lossy(&body).into_owned(),
        });
    }

    serde_json::from_slice(&body).map_err(|error| Error::ApiError {
        status_code: Some(status),
        details: format!("failed to parse response: {error}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{DynamicModel, ImageModelRequest};
    use serde_json::json;
    use std::collections::HashMap;
    use wiremock::matchers::{
        body_partial_json, body_string_contains, header, header_regex, method, path,
    };
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_model(base_url: String) -> OpenAI<DynamicModel> {
        let mut model = OpenAI::model_name("gpt-image-1");
        model.settings.base_url = base_url;
        model.settings.api_key = "test-key".to_string();
        model
    }

    fn image_response() -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "data": [{ "b64_json": "Zm9v" }],
            "usage": {
                "input_tokens": 11,
                "output_tokens": 22,
                "total_tokens": 33
            }
        }))
    }

    #[tokio::test]
    async fn test_generate_image_sends_json_request() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/images/generations"))
            .and(header("authorization", "Bearer test-key"))
            .and(header("x-trace-id", "image-123"))
            .and(body_partial_json(json!({
                "model": "gpt-image-1",
                "prompt": "draw a lighthouse at sunrise",
                "n": 2,
                "size": "1024x1024",
                "quality": "high"
            })))
            .respond_with(image_response())
            .expect(1)
            .mount(&server)
            .await;

        let response = ImageModelRequest::builder()
            .model(test_model(server.uri()))
            .prompt("draw a lighthouse at sunrise")
            .n(2)
            .size("1024x1024")
            .headers(HashMap::from([(
                "x-trace-id".to_string(),
                "image-123".to_string(),
            )]))
            .body(json!({
                "quality": "high"
            }))
            .build()
            .generate_image()
            .await
            .expect("image generation should succeed");

        assert_eq!(response.images, vec!["Zm9v".to_string()]);
        assert_eq!(response.usage.unwrap().total_tokens, Some(33));
    }

    #[tokio::test]
    async fn test_edit_image_sends_multipart_request() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/images/edits"))
            .and(header("authorization", "Bearer test-key"))
            .and(header_regex(
                "content-type",
                "multipart/form-data; boundary=.*",
            ))
            .and(body_string_contains("name=\"model\""))
            .and(body_string_contains("name=\"prompt\""))
            .and(body_string_contains("name=\"image\""))
            .and(body_string_contains("name=\"mask\""))
            .and(body_string_contains("name=\"background\""))
            .respond_with(image_response())
            .expect(1)
            .mount(&server)
            .await;

        let response = ImageModelRequest::builder()
            .model(test_model(server.uri()))
            .prompt("replace the sky with aurora")
            .file(UserImage::new("data:image/png;base64,aGVsbG8="))
            .mask(UserImage::new("data:image/png;base64,d29ybGQ="))
            .body(json!({
                "background": "transparent"
            }))
            .build()
            .generate_image()
            .await
            .expect("image edit should succeed");

        assert_eq!(response.images, vec!["Zm9v".to_string()]);

        let requests = server
            .received_requests()
            .await
            .expect("server should expose received requests");
        let body = String::from_utf8_lossy(&requests[0].body);
        assert!(body.contains("gpt-image-1"));
        assert!(body.contains("replace the sky with aurora"));
        assert!(body.contains("transparent"));
    }

    #[tokio::test]
    async fn test_generate_image_returns_unsupported_warnings() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/images/generations"))
            .respond_with(image_response())
            .expect(1)
            .mount(&server)
            .await;

        let response = ImageModelRequest::builder()
            .model(test_model(server.uri()))
            .prompt("draw a fox")
            .aspect_ratio("16:9")
            .seed(7u32)
            .build()
            .generate_image()
            .await
            .expect("image generation should succeed");

        assert_eq!(response.warnings.len(), 2);
    }
}

#[cfg(all(test, feature = "image-model-request"))]
#[path = "image_model_media_tests.rs"]
mod media_tests;
