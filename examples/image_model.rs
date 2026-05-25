#[cfg(any(feature = "openai", feature = "google"))]
use aisdk::core::{ImageModelRequest, UserImage};
#[cfg(feature = "google")]
use aisdk::providers::Google;
#[cfg(feature = "openai")]
use aisdk::providers::OpenAI;
#[cfg(feature = "openai")]
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "openai-generate".to_string());

    match mode.as_str() {
        "openai-generate" => {
            #[cfg(feature = "openai")]
            {
                return run_openai_generate().await;
            }
            #[cfg(not(feature = "openai"))]
            {
                return Err("openai feature is not enabled".into());
            }
        }
        "openai-edit" => {
            #[cfg(feature = "openai")]
            {
                let image_url = std::env::args().nth(2).unwrap_or_else(|| {
                    "https://upload.wikimedia.org/wikipedia/commons/3/3f/Fronalpstock_big.jpg"
                        .to_string()
                });
                return run_openai_edit(image_url).await;
            }
            #[cfg(not(feature = "openai"))]
            {
                return Err("openai feature is not enabled".into());
            }
        }
        "google-edit" => {
            #[cfg(feature = "google")]
            {
                let image_url = std::env::args().nth(2).unwrap_or_else(|| {
                    "https://upload.wikimedia.org/wikipedia/commons/3/3f/Fronalpstock_big.jpg"
                        .to_string()
                });
                return run_google_edit(image_url).await;
            }
            #[cfg(not(feature = "google"))]
            {
                return Err("google feature is not enabled".into());
            }
        }
        _ => Err("mode must be one of: openai-generate | openai-edit | google-edit".into()),
    }
}

#[cfg(feature = "openai")]
async fn run_openai_generate() -> Result<(), Box<dyn std::error::Error>> {
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
    if let Some(image) = response.image() {
        let preview_len = image.len().min(80);
        println!("first image base64 prefix: {}", &image[..preview_len]);
    }
    Ok(())
}

#[cfg(feature = "openai")]
async fn run_openai_edit(image_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = ImageModelRequest::builder()
        .model(OpenAI::model_name("gpt-image-1"))
        .prompt("Turn this photo into a cinematic travel poster")
        .file(UserImage::new(image_url))
        .size("1024x1024")
        .build()
        .generate_image()
        .await?;

    println!("edited {} image(s)", response.images().len());
    Ok(())
}

#[cfg(feature = "google")]
async fn run_google_edit(image_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = ImageModelRequest::builder()
        .model(Google::gemini_2_5_flash_image())
        .prompt("Turn this photo into a polished product poster")
        .file(UserImage::new(image_url))
        .aspect_ratio("1:1")
        .build()
        .generate_image()
        .await?;

    println!("edited {} image(s)", response.images().len());
    Ok(())
}
