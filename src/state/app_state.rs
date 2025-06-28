use crate::application::use_cases::product::manage_product_use_case::ProductUseCase;
use crate::infrastructure::repository_impl::SqliteProductRepository;
use std::sync::Arc;

pub struct AppState {
    pub product_use_case: ProductUseCase<SqliteProductRepository>,
    // 後でリポジトリやサービスを追加
}

impl AppState {
    pub fn new(product_use_case: ProductUseCase<SqliteProductRepository>) -> Self {
        Self { product_use_case }
    }
}
