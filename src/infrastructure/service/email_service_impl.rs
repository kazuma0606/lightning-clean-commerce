use crate::application::service::email_service::EmailService;
use async_trait::async_trait;

#[derive(Debug)]
pub struct EmailServiceImpl;

impl EmailServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct EmailError(String);

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for EmailError {}

#[async_trait]
impl EmailService for EmailServiceImpl {
    type Error = EmailError;

    async fn send_welcome_email(&self, email: &str, username: &str) -> Result<(), Self::Error> {
        todo!("Send welcome email implementation")
    }

    async fn send_password_reset_email(
        &self,
        email: &str,
        reset_token: &str,
    ) -> Result<(), Self::Error> {
        todo!("Send password reset email implementation")
    }

    async fn send_order_confirmation_email(
        &self,
        email: &str,
        order_id: &str,
    ) -> Result<(), Self::Error> {
        todo!("Send order confirmation email implementation")
    }
}
