use std::path::Path;
use tokio::fs;

const PUBLIC_PATH: &str = "./public/";

pub async fn read(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let public_path = Path::new(PUBLIC_PATH).join(path);
    let text = match fs::read_to_string(public_path).await {
        Ok(text) => text,
        Err(e) => return Err(Box::new(e)),
    };
    Ok(text)
}
