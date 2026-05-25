# vibewindow-aisdk

[![Docs](https://img.shields.io/badge/docs-latest-blue)](https://vibewindow-aisdk.rs)
[![Build Status](https://github.com/hunzhiwange/aisdk/actions/workflows/ci.yml/badge.svg)](https://github.com/hunzhiwange/aisdk/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Issues](https://img.shields.io/github/issues/hunzhiwange/aisdk)](https://github.com/hunzhiwange/aisdk/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/hunzhiwange/aisdk/pulls)


An open-source, **provider-agnostic** Rust library for building AI-powered applications, inspired by the Vercel AI SDK. Type-safe, framework-friendly, and ready to connect with **70+ AI providers**.

To learn more about how to use the AI SDK, check out our [Documentation](https://vibewindow-aisdk.rs) and [API Reference](https://docs.rs/vibewindow-aisdk/latest).

## Features
* Agents & Tool Execution
* Prompt Templating
* Text Generation & Streaming
* Multimodal Image Input for OpenAI, Google, and Anthropic
* Image Generation & Editing with `ImageModelRequest`
* Structured Output (JSON Schema)
* Embedding Model Support
* Compatible with [Vercel AI SDK UI](https://ai-sdk.dev/docs/ai-sdk-ui/overview) (React, Solid, Vue, Svelte, …)
* Supports 73+ providers, including Anthropic, Google, OpenAI, OpenRouter, xAI

## Installation

```bash
cargo add vibewindow-aisdk
```

## Usage

Enable Providers of your choice such as OpenAI, Anthropic, Google, and [more](https://vibewindow-aisdk.rs/docs#model-providers)

### Example with OpenAI provider

```bash
cargo add vibewindow-aisdk --features openai
```

### Basic Text Generation

```rust
use aisdk::core::LanguageModelRequest;
use aisdk::providers::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let openai = OpenAI::gpt_5();

    let result = LanguageModelRequest::builder()
        .model(openai)
        .prompt("What is the meaning of life?")
        .build()
        .generate_text() // or stream_text() for streaming
        .await?;

    println!("Response: {:?}", result.text());
    Ok(())
}
```

### Google 看图问答

```bash
cargo add vibewindow-aisdk --features google
```

```rust
use aisdk::core::{LanguageModelRequest, Message, UserMessage};
use aisdk::providers::Google;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = LanguageModelRequest::builder()
        .model(Google::gemini_2_5_pro())
        .messages(vec![Message::User(
            UserMessage::new("Describe the storefront and list any visible text.")
                .with_image_url("https://upload.wikimedia.org/wikipedia/commons/3/3f/Fronalpstock_big.jpg"),
        )])
        .build()
        .generate_text()
        .await?;

    println!("{}", response.text().unwrap_or_default());
    Ok(())
}
```

### Anthropic 看图问答

```bash
cargo add vibewindow-aisdk --features anthropic
```

```rust
use aisdk::core::{LanguageModelRequest, Message, UserMessage};
use aisdk::providers::Anthropic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = LanguageModelRequest::builder()
        .model(Anthropic::claude_sonnet_4_5())
        .messages(vec![Message::User(
            UserMessage::new("What objects stand out in this image? Reply with a short bullet list.")
                .with_image_url("https://upload.wikimedia.org/wikipedia/commons/3/3f/Fronalpstock_big.jpg"),
        )])
        .build()
        .generate_text()
        .await?;

    println!("{}", response.text().unwrap_or_default());
    Ok(())
}
```

### 文生图

```bash
cargo add vibewindow-aisdk --features openai
```

```rust
use aisdk::core::ImageModelRequest;
use aisdk::providers::OpenAI;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = ImageModelRequest::builder()
        .model(OpenAI::model_name("gpt-image-1"))
        .prompt("A bold risograph poster of a red fox crossing a snowy bridge")
        .size("1024x1024")
        .body(json!({
            "quality": "high",
            "background": "transparent"
        }))
        .build()
        .generate_image()
        .await?;

    println!("generated {} image(s)", response.images().len());
    Ok(())
}
```

### 图编辑

```rust
use aisdk::core::{ImageModelRequest, UserImage};
use aisdk::providers::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = ImageModelRequest::builder()
        .model(OpenAI::model_name("gpt-image-1"))
        .prompt("Turn this photo into a cinematic travel poster")
        .file(UserImage::new("https://upload.wikimedia.org/wikipedia/commons/3/3f/Fronalpstock_big.jpg"))
        .size("1024x1024")
        .build()
        .generate_image()
        .await?;

    println!("edited {} image(s)", response.images().len());
    Ok(())
}
```

## Agents

### Defining a Tool

Use the `#[tool]` macro to expose a Rust function as a callable tool.

```rust
use aisdk::core::Tool;
use aisdk::macros::tool;

#[tool]
/// Get the weather information given a location
pub fn get_weather(location: String) -> Tool {
    let weather = match location.as_str() {
        "New York" => 75,
        "Tokyo" => 80,
        _ => 70,
    };
    Ok(weather.to_string())
}
```

### Using Tools in an Agent

Register tools with an agent so the model can call them during its reasoning loop.

```rust
use aisdk::core::{LanguageModelRequest, utils::step_count_is};
use aisdk::providers::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let result = LanguageModelRequest::builder()
        .model(OpenAI::gpt_4o())
        .system("You are a helpful assistant.")
        .prompt("What is the weather in New York?")
        .with_tool(get_weather())
        .stop_when(step_count_is(3)) // Limit agent loop to 3 steps
        .build()
        .generate_text()
        .await?;

    println!("Response: {:?}", result.text());
    Ok(())
}
```

### Prompts

The vibewindow-aisdk prompt feature provides, file-based template system for managing AI prompts using the Tera template engine. It allows you to create reusable prompt templates with variable substitution, conditionals, loops, and template inclusion. See [Examples](https://vibewindow-aisdk.rs/docs/concepts/prompt) for more template examples. Enable with `cargo add vibewindow-aisdk --features prompt`

### Roadmap

- [ ] Voice Model Request Support
- [ ] Observability & OpenTelemetry (OTel) Support

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Licensed under the MIT License. See [LICENSE](./LICENSE) for details.
