use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Mutex;
// use thiserror::Error;

use rspc_ts_front::application::use_cases::product::manage_product_use_case::ProductUseCase;
use rspc_ts_front::domain::entity::product::Product;
use rspc_ts_front::domain::repository::product_repository::ProductRepository;

#[derive(Debug)]
pub enum MockError {
    NotFound,
    AlreadyExists,
    Other(String),
}

impl std::fmt::Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MockError::NotFound => write!(f, "Not found"),
            MockError::AlreadyExists => write!(f, "Already exists"),
            MockError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for MockError {}

#[derive(Debug, Default)]
struct MockProductRepository {
    products: Mutex<HashMap<String, Product>>,
}

#[async_trait]
impl ProductRepository for MockProductRepository {
    type Error = MockError;

    async fn save(&self, product: Product) -> Result<Product, Self::Error> {
        let mut map = self.products.lock().unwrap();
        if map.contains_key(&product.id) {
            return Err(MockError::AlreadyExists);
        }
        map.insert(product.id.clone(), product.clone());
        Ok(product)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, Self::Error> {
        let map = self.products.lock().unwrap();
        Ok(map.get(id).cloned())
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Product>, Self::Error> {
        let map = self.products.lock().unwrap();
        Ok(map.values().filter(|p| p.name == name).cloned().collect())
    }

    async fn find_by_category(&self, category: &str) -> Result<Vec<Product>, Self::Error> {
        let map = self.products.lock().unwrap();
        Ok(map
            .values()
            .filter(|p| p.category.as_deref() == Some(category))
            .cloned()
            .collect())
    }

    async fn find_all(&self) -> Result<Vec<Product>, Self::Error> {
        let map = self.products.lock().unwrap();
        Ok(map.values().cloned().collect())
    }

    async fn update(&self, product: Product) -> Result<Product, Self::Error> {
        let mut map = self.products.lock().unwrap();
        if !map.contains_key(&product.id) {
            return Err(MockError::NotFound);
        }
        map.insert(product.id.clone(), product.clone());
        Ok(product)
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        let mut map = self.products.lock().unwrap();
        Ok(map.remove(id).is_some())
    }

    async fn update_stock(&self, id: &str, quantity: i32) -> Result<Product, Self::Error> {
        let mut map = self.products.lock().unwrap();
        let product = map.get_mut(id).ok_or(MockError::NotFound)?;
        product.stock_quantity = quantity;
        product.updated_at = Utc::now();
        Ok(product.clone())
    }
}

#[tokio::test]
async fn test_product_crud() {
    let repo = MockProductRepository::default();
    let usecase = ProductUseCase::new(repo);

    // Create
    let product = usecase
        .create_product(
            "Test Product".to_string(),
            Some("desc".to_string()),
            1000,
            10,
            Some("cat".to_string()),
        )
        .await
        .expect("create");
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.price, 1000);
    assert_eq!(product.stock_quantity, 10);

    // Read
    let found = usecase
        .get_product(&product.id)
        .await
        .expect("get")
        .unwrap();
    assert_eq!(found.id, product.id);

    // Update
    let updated = usecase
        .update_product(
            &product.id,
            Some("Updated Name".to_string()),
            Some("new desc".to_string()),
            Some(2000),
            Some(5),
            Some("cat2".to_string()),
        )
        .await
        .expect("update");
    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.price, 2000);
    assert_eq!(updated.stock_quantity, 5);
    assert_eq!(updated.category.as_deref(), Some("cat2"));

    // Delete
    let deleted = usecase.delete_product(&product.id).await.expect("delete");
    assert!(deleted);
    let not_found = usecase
        .get_product(&product.id)
        .await
        .expect("get after delete");
    assert!(not_found.is_none());
}
