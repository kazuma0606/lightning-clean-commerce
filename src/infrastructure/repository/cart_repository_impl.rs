use crate::domain::entity::cart::Cart;
use crate::domain::repository::cart_repository::CartRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct CartRepositoryImpl {
    pool: SqlitePool,
}

impl CartRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CartRepository for CartRepositoryImpl {
    type Error = sqlx::Error;

    async fn save(&self, cart: Cart) -> Result<Cart, Self::Error> {
        todo!("Save cart implementation")
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Cart>, Self::Error> {
        todo!("Find cart by id implementation")
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<Cart>, Self::Error> {
        todo!("Find cart by user id implementation")
    }

    async fn update(&self, cart: Cart) -> Result<Cart, Self::Error> {
        todo!("Update cart implementation")
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        todo!("Delete cart implementation")
    }
}
