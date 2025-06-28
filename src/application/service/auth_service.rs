use async_trait::async_trait;

#[async_trait]
pub trait AuthService {
    type Error: std::error::Error + Send + Sync;

    async fn generate_token(&self, user_id: &str) -> Result<String, Self::Error>;
    async fn validate_token(&self, token: &str) -> Result<String, Self::Error>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<String, Self::Error>;
}
