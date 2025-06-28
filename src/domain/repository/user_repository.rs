use crate::domain::entity::user::User;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait UserRepository {
    type Error: Error + Send + Sync;

    async fn save(&self, user: User) -> Result<User, Self::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Self::Error>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Self::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Self::Error>;
    async fn find_all(&self) -> Result<Vec<User>, Self::Error>;
    async fn update(&self, user: User) -> Result<User, Self::Error>;
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}
