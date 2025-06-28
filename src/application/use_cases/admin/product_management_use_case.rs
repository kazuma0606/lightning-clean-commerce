use crate::application::dto::product_dto::{CreateProductRequest, ProductDto};
use crate::domain::repository::product_repository::ProductRepository;

pub struct ProductManagementUseCase<R>
where
    R: ProductRepository,
{
    product_repository: R,
}

impl<R> ProductManagementUseCase<R>
where
    R: ProductRepository,
{
    pub fn new(product_repository: R) -> Self {
        Self { product_repository }
    }

    pub async fn create_product(
        &self,
        request: CreateProductRequest,
    ) -> Result<ProductDto, R::Error> {
        // 実装は後で追加
        todo!("Create product implementation")
    }

    pub async fn update_product(
        &self,
        product_id: String,
        product_data: ProductDto,
    ) -> Result<ProductDto, R::Error> {
        // 実装は後で追加
        todo!("Update product implementation")
    }

    pub async fn delete_product(&self, product_id: String) -> Result<bool, R::Error> {
        // 実装は後で追加
        todo!("Delete product implementation")
    }
}
