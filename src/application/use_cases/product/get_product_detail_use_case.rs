use crate::application::dto::product_dto::ProductDto;
use crate::domain::repository::product_repository::ProductRepository;

pub struct GetProductDetailUseCase<R>
where
    R: ProductRepository,
{
    product_repository: R,
}

impl<R> GetProductDetailUseCase<R>
where
    R: ProductRepository,
{
    pub fn new(product_repository: R) -> Self {
        Self { product_repository }
    }

    pub async fn execute(&self, product_id: String) -> Result<ProductDto, R::Error> {
        // 実装は後で追加
        todo!("Get product detail use case implementation")
    }
}
