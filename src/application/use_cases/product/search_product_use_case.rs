use crate::application::dto::product_dto::ProductDto;
use crate::domain::repository::product_repository::ProductRepository;

pub struct SearchProductUseCase<R>
where
    R: ProductRepository,
{
    product_repository: R,
}

impl<R> SearchProductUseCase<R>
where
    R: ProductRepository,
{
    pub fn new(product_repository: R) -> Self {
        Self { product_repository }
    }

    pub async fn execute(&self, query: String) -> Result<Vec<ProductDto>, R::Error> {
        // 実装は後で追加
        todo!("Search product use case implementation")
    }
}
