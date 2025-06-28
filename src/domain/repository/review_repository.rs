use crate::domain::entity::review::Review;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait ReviewRepository {
    type Error: Error + Send + Sync;

    async fn save(&self, review: Review) -> Result<Review, Self::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Review>, Self::Error>;
    async fn find_by_product_id(&self, product_id: &str) -> Result<Vec<Review>, Self::Error>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Review>, Self::Error>;
    async fn update(&self, review: Review) -> Result<Review, Self::Error>;
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}
