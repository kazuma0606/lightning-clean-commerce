use crate::domain::entity::stock_movement::StockMovement;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait StockRepository {
    type Error: Error + Send + Sync;

    async fn save_movement(&self, movement: StockMovement) -> Result<StockMovement, Self::Error>;
    async fn find_movements_by_product_id(
        &self,
        product_id: &str,
    ) -> Result<Vec<StockMovement>, Self::Error>;
    async fn get_current_stock(&self, product_id: &str) -> Result<i32, Self::Error>;
}
