use std::path::Path;
use tokio::fs;

const PUBLIC_PATH: &str = "./public/";

pub async fn read(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let public_path = Path::new(PUBLIC_PATH).join(path);
    let bytes = match fs::read(public_path).await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading file: {e:?}");
            return Err(Box::new(e));
        }
    };
    Ok(bytes)
}
