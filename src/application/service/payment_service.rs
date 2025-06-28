use async_trait::async_trait;

#[async_trait]
pub trait PaymentService {
    type Error: std::error::Error + Send + Sync;

    async fn process_payment(
        &self,
        amount: f64,
        payment_method: &str,
    ) -> Result<String, Self::Error>;
    async fn refund_payment(&self, payment_id: &str, amount: f64) -> Result<(), Self::Error>;
}
