use crate::domain::entity::order::Order;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait OrderRepository {
    type Error: Error + Send + Sync;

    async fn save(&self, order: Order) -> Result<Order, Self::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Order>, Self::Error>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Order>, Self::Error>;
    async fn find_all(&self) -> Result<Vec<Order>, Self::Error>;
    async fn update(&self, order: Order) -> Result<Order, Self::Error>;
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}
