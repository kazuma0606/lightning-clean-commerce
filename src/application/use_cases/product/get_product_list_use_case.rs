use crate::application::dto::product_dto::ProductDto;
use crate::domain::repository::product_repository::ProductRepository;

pub struct GetProductListUseCase<R>
where
    R: ProductRepository,
{
    product_repository: R,
}

impl<R> GetProductListUseCase<R>
where
    R: ProductRepository,
{
    pub fn new(product_repository: R) -> Self {
        Self { product_repository }
    }

    pub async fn execute(&self, page: u32, per_page: u32) -> Result<Vec<ProductDto>, R::Error> {
        // 実装は後で追加
        todo!("Get product list use case implementation")
    }
}
