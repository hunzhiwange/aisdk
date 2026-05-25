#[cfg(feature = "anthropic")]
use aisdk::core::{LanguageModelRequest, Message, UserMessage};
#[cfg(feature = "anthropic")]
use aisdk::providers::Anthropic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    #[cfg(feature = "anthropic")]
    {
        let image_url = std::env::args().nth(1).unwrap_or_else(|| {
            "https://upload.wikimedia.org/wikipedia/commons/3/3f/Fronalpstock_big.jpg".to_string()
        });
        let question = std::env::args().nth(2).unwrap_or_else(|| {
            "What stands out in this image? Reply in a short bullet list.".to_string()
        });

        let response = LanguageModelRequest::builder()
            .model(Anthropic::claude_sonnet_4_5())
            .messages(vec![Message::User(
                UserMessage::new(question).with_image_url(image_url),
            )])
            .build()
            .generate_text()
            .await?;

        println!("{}", response.text().unwrap_or_default());
        return Ok(());
    }

    #[cfg(not(feature = "anthropic"))]
    {
        Err("anthropic feature is not enabled".into())
    }
}
