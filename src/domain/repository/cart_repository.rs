use crate::domain::entity::cart::Cart;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait CartRepository {
    type Error: Error + Send + Sync;

    async fn save(&self, cart: Cart) -> Result<Cart, Self::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Cart>, Self::Error>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Cart>, Self::Error>;
    async fn update(&self, cart: Cart) -> Result<Cart, Self::Error>;
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}
