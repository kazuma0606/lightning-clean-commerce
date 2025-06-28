use crate::application::dto::order_dto::{CreateOrderRequest, OrderDto};
use crate::domain::repository::order_repository::OrderRepository;

pub struct CreateOrderUseCase<R>
where
    R: OrderRepository,
{
    order_repository: R,
}

impl<R> CreateOrderUseCase<R>
where
    R: OrderRepository,
{
    pub fn new(order_repository: R) -> Self {
        Self { order_repository }
    }

    pub async fn execute(
        &self,
        user_id: String,
        request: CreateOrderRequest,
    ) -> Result<OrderDto, R::Error> {
        // 実装は後で追加
        todo!("Create order use case implementation")
    }
}
