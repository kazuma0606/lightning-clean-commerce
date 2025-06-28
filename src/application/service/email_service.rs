use async_trait::async_trait;

#[async_trait]
pub trait EmailService {
    type Error: std::error::Error + Send + Sync;

    async fn send_welcome_email(&self, email: &str, username: &str) -> Result<(), Self::Error>;
    async fn send_password_reset_email(
        &self,
        email: &str,
        reset_token: &str,
    ) -> Result<(), Self::Error>;
    async fn send_order_confirmation_email(
        &self,
        email: &str,
        order_id: &str,
    ) -> Result<(), Self::Error>;
}
