use crate::domain::entity::stock_movement::StockMovement;
use crate::domain::repository::stock_repository::StockRepository;

pub struct StockManagementUseCase<R>
where
    R: StockRepository,
{
    stock_repository: R,
}

impl<R> StockManagementUseCase<R>
where
    R: StockRepository,
{
    pub fn new(stock_repository: R) -> Self {
        Self { stock_repository }
    }

    pub async fn get_stock_level(&self, product_id: String) -> Result<i32, R::Error> {
        // 実装は後で追加
        todo!("Get stock level implementation")
    }

    pub async fn add_stock(
        &self,
        product_id: String,
        quantity: i32,
        reason: String,
    ) -> Result<StockMovement, R::Error> {
        // 実装は後で追加
        todo!("Add stock implementation")
    }
}
