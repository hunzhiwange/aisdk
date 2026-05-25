#[cfg(feature = "google")]
use aisdk::core::{LanguageModelRequest, Message, UserMessage};
#[cfg(feature = "google")]
use aisdk::providers::Google;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    #[cfg(feature = "google")]
    {
        let image_url = std::env::args().nth(1).unwrap_or_else(|| {
            "https://upload.wikimedia.org/wikipedia/commons/3/3f/Fronalpstock_big.jpg".to_string()
        });
        let question = std::env::args().nth(2).unwrap_or_else(|| {
            "Describe the scene and list any clearly visible objects.".to_string()
        });

        let response = LanguageModelRequest::builder()
            .model(Google::gemini_2_5_pro())
            .messages(vec![Message::User(
                UserMessage::new(question).with_image_url(image_url),
            )])
            .build()
            .generate_text()
            .await?;

        println!("{}", response.text().unwrap_or_default());
        return Ok(());
    }

    #[cfg(not(feature = "google"))]
    {
        Err("google feature is not enabled".into())
    }
}
