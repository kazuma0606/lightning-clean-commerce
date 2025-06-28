use crate::application::dto::cart_dto::CartDto;
use crate::domain::repository::cart_repository::CartRepository;

pub struct GetCartUseCase<R>
where
    R: CartRepository,
{
    cart_repository: R,
}

impl<R> GetCartUseCase<R>
where
    R: CartRepository,
{
    pub fn new(cart_repository: R) -> Self {
        Self { cart_repository }
    }

    pub async fn execute(&self, user_id: String) -> Result<CartDto, R::Error> {
        // 実装は後で追加
        todo!("Get cart use case implementation")
    }
}
