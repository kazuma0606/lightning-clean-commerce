use crate::application::dto::order_dto::OrderDto;
use crate::domain::repository::order_repository::OrderRepository;

pub struct GetOrderListUseCase<R>
where
    R: OrderRepository,
{
    order_repository: R,
}

impl<R> GetOrderListUseCase<R>
where
    R: OrderRepository,
{
    pub fn new(order_repository: R) -> Self {
        Self { order_repository }
    }

    pub async fn execute(&self, user_id: String) -> Result<Vec<OrderDto>, R::Error> {
        // 実装は後で追加
        todo!("Get order list use case implementation")
    }
}
