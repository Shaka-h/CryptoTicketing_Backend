use rocket::data::{ByteUnit, Data};
use rocket::tokio::fs::File;
use std::path::{Path, PathBuf};

pub async fn upload_image(file: Data<'_>) -> Result<String, String> {
    let path: PathBuf = Path::new("media").join("uploaded_image.jpg");

    let mut f = File::create(path.clone()).await.map_err(|e| e.to_string())?;
    file.open(ByteUnit::default())
        .stream_to(&mut f)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("File uploaded to: {:?}", path))
}