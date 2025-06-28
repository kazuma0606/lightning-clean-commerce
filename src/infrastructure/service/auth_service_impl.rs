use crate::application::service::auth_service::AuthService;
use async_trait::async_trait;
use std::error::Error;

#[derive(Debug)]
pub struct AuthServiceImpl;

impl AuthServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct AuthError(String);

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AuthError {}

#[async_trait]
impl AuthService for AuthServiceImpl {
    type Error = AuthError;

    async fn generate_token(&self, user_id: &str) -> Result<String, Self::Error> {
        todo!("Generate token implementation")
    }

    async fn validate_token(&self, token: &str) -> Result<String, Self::Error> {
        todo!("Validate token implementation")
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<String, Self::Error> {
        todo!("Refresh token implementation")
    }
}
