use crate::application::service::upload_service::UploadService;
use async_trait::async_trait;

#[derive(Debug)]
pub struct UploadServiceImpl;

impl UploadServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct UploadError(String);

impl std::fmt::Display for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for UploadError {}

#[async_trait]
impl UploadService for UploadServiceImpl {
    type Error = UploadError;

    async fn upload_file(&self, file_data: Vec<u8>, filename: &str) -> Result<String, Self::Error> {
        todo!("Upload file implementation")
    }

    async fn delete_file(&self, file_url: &str) -> Result<(), Self::Error> {
        todo!("Delete file implementation")
    }
}
