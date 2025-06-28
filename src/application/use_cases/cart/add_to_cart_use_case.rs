use crate::application::dto::cart_dto::{AddToCartRequest, CartDto};
use crate::domain::repository::cart_repository::CartRepository;

pub struct AddToCartUseCase<R>
where
    R: CartRepository,
{
    cart_repository: R,
}

impl<R> AddToCartUseCase<R>
where
    R: CartRepository,
{
    pub fn new(cart_repository: R) -> Self {
        Self { cart_repository }
    }

    pub async fn execute(
        &self,
        user_id: String,
        request: AddToCartRequest,
    ) -> Result<CartDto, R::Error> {
        // 実装は後で追加
        todo!("Add to cart use case implementation")
    }
}
