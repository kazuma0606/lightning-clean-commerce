use crate::domain::entity::user::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::infrastructure::database::models::user_model::UserModel;
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::error::Error;

#[derive(Debug)]
pub struct UserRepositoryImpl {
    pool: SqlitePool,
}

impl UserRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    type Error = sqlx::Error;

    async fn save(&self, user: User) -> Result<User, Self::Error> {
        // 実装は後で追加
        todo!("Save user implementation")
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Self::Error> {
        // 実装は後で追加
        todo!("Find user by id implementation")
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Self::Error> {
        // 実装は後で追加
        todo!("Find user by username implementation")
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Self::Error> {
        // 実装は後で追加
        todo!("Find user by email implementation")
    }

    async fn find_all(&self) -> Result<Vec<User>, Self::Error> {
        // 実装は後で追加
        todo!("Find all users implementation")
    }

    async fn update(&self, user: User) -> Result<User, Self::Error> {
        // 実装は後で追加
        todo!("Update user implementation")
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        // 実装は後で追加
        todo!("Delete user implementation")
    }
}
