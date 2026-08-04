use aisdk::providers::{OpenAICompatibleResolveOptions, resolve_openai_compatible_model};

fn main() -> aisdk::Result<()> {
    let _model = resolve_openai_compatible_model(
        "alibaba-cn/qwen3.7-plus-2026-05-26",
        OpenAICompatibleResolveOptions {
            api_key: Some("your-api-key".to_owned()),
            ..Default::default()
        },
    )?;

    Ok(())
}
