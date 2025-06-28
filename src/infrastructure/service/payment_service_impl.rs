use crate::application::service::payment_service::PaymentService;
use async_trait::async_trait;

#[derive(Debug)]
pub struct PaymentServiceImpl;

impl PaymentServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct PaymentError(String);

impl std::fmt::Display for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for PaymentError {}

#[async_trait]
impl PaymentService for PaymentServiceImpl {
    type Error = PaymentError;

    async fn process_payment(
        &self,
        amount: f64,
        payment_method: &str,
    ) -> Result<String, Self::Error> {
        todo!("Process payment implementation")
    }

    async fn refund_payment(&self, payment_id: &str, amount: f64) -> Result<(), Self::Error> {
        todo!("Refund payment implementation")
    }
}
