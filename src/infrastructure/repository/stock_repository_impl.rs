use crate::domain::entity::stock_movement::StockMovement;
use crate::domain::repository::stock_repository::StockRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct StockRepositoryImpl {
    pool: SqlitePool,
}

impl StockRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StockRepository for StockRepositoryImpl {
    type Error = sqlx::Error;

    async fn save_movement(&self, movement: StockMovement) -> Result<StockMovement, Self::Error> {
        todo!("Save stock movement implementation")
    }

    async fn find_movements_by_product_id(
        &self,
        product_id: &str,
    ) -> Result<Vec<StockMovement>, Self::Error> {
        todo!("Find stock movements by product id implementation")
    }

    async fn get_current_stock(&self, product_id: &str) -> Result<i32, Self::Error> {
        todo!("Get current stock implementation")
    }
}
