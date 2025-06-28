use crate::application::dto::order_dto::OrderDto;
use crate::domain::repository::order_repository::OrderRepository;

pub struct OrderManagementUseCase<R>
where
    R: OrderRepository,
{
    order_repository: R,
}

impl<R> OrderManagementUseCase<R>
where
    R: OrderRepository,
{
    pub fn new(order_repository: R) -> Self {
        Self { order_repository }
    }

    pub async fn get_all_orders(&self) -> Result<Vec<OrderDto>, R::Error> {
        // 実装は後で追加
        todo!("Get all orders implementation")
    }

    pub async fn update_order_status(
        &self,
        order_id: String,
        status: String,
    ) -> Result<OrderDto, R::Error> {
        // 実装は後で追加
        todo!("Update order status implementation")
    }
}
