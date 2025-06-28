use crate::domain::entity::category::Category;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait CategoryRepository {
    type Error: Error + Send + Sync;

    async fn save(&self, category: Category) -> Result<Category, Self::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Category>, Self::Error>;
    async fn find_all(&self) -> Result<Vec<Category>, Self::Error>;
    async fn find_by_parent_id(&self, parent_id: &str) -> Result<Vec<Category>, Self::Error>;
    async fn update(&self, category: Category) -> Result<Category, Self::Error>;
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}
