//! Image generation through a deployment's OpenAI-compatible chat endpoint.

use super::{
    OpenAIImageResponse, OpenAIImageUsage, image_prompt, image_references, image_results,
    image_sources, merge_extra_body, multimodal_request, send_multimodal_request,
};
use crate::core::capabilities::ModelName;
use crate::core::image_model::{ImageModelOptions, ImageModelResponse, ImageModelWarning};
use crate::error::Result;
use crate::providers::openai::OpenAI;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    usage: Option<ChatUsage>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: Option<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatMessage {
    images: Option<Vec<Value>>,
    content: Option<Value>,
}

#[derive(Deserialize)]
struct ChatUsage {
    prompt_tokens: Option<usize>,
    completion_tokens: Option<usize>,
    total_tokens: Option<usize>,
}

pub(super) async fn generate<M: ModelName>(
    model: &OpenAI<M>,
    path: &str,
    input: ImageModelOptions,
) -> Result<ImageModelResponse> {
    let prompt = image_prompt(&input)?;
    let mut content = vec![json!({"type": "text", "text": prompt})];
    for url in image_references(&input.files)? {
        content.push(json!({"type": "image_url", "image_url": {"url": url}}));
    }

    let mut body = merge_extra_body(model.settings.body.as_ref(), input.body.as_ref());
    body.entry("model")
        .or_insert_with(|| json!(model.lm_options.model));
    body.entry("modalities")
        .or_insert_with(|| json!(["image", "text"]));
    body.entry("messages")
        .or_insert_with(|| json!([{"role": "user", "content": content}]));
    body.insert("stream".into(), Value::Bool(false));
    let (client, url, headers) = multimodal_request(model, path, &input)?;
    let response: ChatResponse = send_multimodal_request(
        &client,
        client.post(url).headers(headers).json(&body),
        "Gemini chat image submission",
    )
    .await?;

    let mut sources = Vec::new();
    for message in response
        .choices
        .iter()
        .filter_map(|choice| choice.message.as_ref())
    {
        if let Some(images) = &message.images {
            sources.extend(images.iter().filter_map(part_image_url));
        }
        match &message.content {
            Some(Value::Array(parts)) => sources.extend(parts.iter().filter_map(part_image_url)),
            Some(Value::String(text)) => sources.extend(markdown_image_urls(text)),
            _ => {}
        }
    }

    let warnings = [
        ("n", input.n.is_some_and(|count| count != 1)),
        ("size", input.size.is_some()),
        ("aspect_ratio", input.aspect_ratio.is_some()),
        ("seed", input.seed.is_some()),
    ]
    .into_iter()
    .filter(|(_, requested)| *requested)
    .map(|(feature, _)| ImageModelWarning::Unsupported {
        feature,
        details: None,
    })
    .collect();
    let result = OpenAIImageResponse {
        data: image_sources(sources)?,
        usage: response.usage.map(|usage| OpenAIImageUsage {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
        }),
    };
    image_results(result, warnings, &model.settings.base_url).await
}

fn part_image_url(part: &Value) -> Option<&str> {
    let image = part.get("image_url")?;
    image.as_str().or_else(|| image.get("url")?.as_str())
}

fn markdown_image_urls(text: &str) -> impl Iterator<Item = &str> {
    text.split("![").skip(1).filter_map(|image| {
        let (label, tail) = image.split_once("](")?;
        if label.contains(']') {
            return None;
        }
        let (url, _) = tail.split_once(')')?;
        (url.starts_with("https://")
            || url.starts_with("http://")
            || url.starts_with("data:image/"))
        .then_some(url)
    })
}
