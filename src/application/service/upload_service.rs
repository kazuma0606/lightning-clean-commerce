use async_trait::async_trait;

#[async_trait]
pub trait UploadService {
    type Error: std::error::Error + Send + Sync;

    async fn upload_file(&self, file_data: Vec<u8>, filename: &str) -> Result<String, Self::Error>;
    async fn delete_file(&self, file_url: &str) -> Result<(), Self::Error>;
}
