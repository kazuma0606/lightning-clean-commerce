use crate::application::dto::order_dto::OrderDto;
use crate::domain::repository::order_repository::OrderRepository;

pub struct GetOrderDetailUseCase<R>
where
    R: OrderRepository,
{
    order_repository: R,
}

impl<R> GetOrderDetailUseCase<R>
where
    R: OrderRepository,
{
    pub fn new(order_repository: R) -> Self {
        Self { order_repository }
    }

    pub async fn execute(&self, order_id: String) -> Result<OrderDto, R::Error> {
        // 実装は後で追加
        todo!("Get order detail use case implementation")
    }
}
