use crate::domain::entity::order::Order;
use crate::domain::repository::order_repository::OrderRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct OrderRepositoryImpl {
    pool: SqlitePool,
}

impl OrderRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepository for OrderRepositoryImpl {
    type Error = sqlx::Error;

    async fn save(&self, order: Order) -> Result<Order, Self::Error> {
        todo!("Save order implementation")
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Order>, Self::Error> {
        todo!("Find order by id implementation")
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Order>, Self::Error> {
        todo!("Find orders by user id implementation")
    }

    async fn find_all(&self) -> Result<Vec<Order>, Self::Error> {
        todo!("Find all orders implementation")
    }

    async fn update(&self, order: Order) -> Result<Order, Self::Error> {
        todo!("Update order implementation")
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        todo!("Delete order implementation")
    }
}
