use crate::domain::entity::address::Address;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait AddressRepository {
    type Error: Error + Send + Sync;

    async fn save(&self, address: Address) -> Result<Address, Self::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Address>, Self::Error>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Address>, Self::Error>;
    async fn update(&self, address: Address) -> Result<Address, Self::Error>;
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}
