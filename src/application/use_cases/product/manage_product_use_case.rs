// src/application/usecase/product_usecase.rs

use crate::domain::entity::product::Product;
use crate::domain::repository::product_repository::ProductRepository;
use crate::domain::utils::error::AppError;

/// 商品管理ユースケース
pub struct ProductUseCase<R>
where
    R: ProductRepository,
{
    product_repository: R,
}

impl<R> ProductUseCase<R>
where
    R: ProductRepository,
    R::Error: std::fmt::Display + Send + Sync + 'static,
{
    /// 新しい商品管理ユースケースを作成
    pub fn new(product_repository: R) -> Self {
        Self { product_repository }
    }

    /// 商品を作成
    pub async fn create_product(
        &self,
        name: String,
        description: Option<String>,
        price: i32,
        stock_quantity: i32,
        category: Option<String>,
    ) -> Result<Product, AppError> {
        // バリデーション
        if name.trim().is_empty() {
            return Err(AppError::validation("商品名は必須です"));
        }
        if price <= 0 {
            return Err(AppError::validation(
                "価格は0より大きい値である必要があります",
            ));
        }
        if stock_quantity < 0 {
            return Err(AppError::validation(
                "在庫数は0以上の値である必要があります",
            ));
        }

        let product = Product::new(name, description, price, stock_quantity, category);
        self.product_repository
            .save(product)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 商品を取得
    pub async fn get_product(&self, id: &str) -> Result<Option<Product>, AppError> {
        self.product_repository
            .find_by_id(id)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 全商品を取得
    pub async fn get_all_products(&self) -> Result<Vec<Product>, AppError> {
        self.product_repository
            .find_all()
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 商品名で検索
    pub async fn search_products_by_name(&self, name: &str) -> Result<Vec<Product>, AppError> {
        if name.trim().is_empty() {
            return Err(AppError::validation("検索キーワードは必須です"));
        }
        self.product_repository
            .find_by_name(name)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// カテゴリで検索
    pub async fn search_products_by_category(
        &self,
        category: &str,
    ) -> Result<Vec<Product>, AppError> {
        if category.trim().is_empty() {
            return Err(AppError::validation("カテゴリは必須です"));
        }
        self.product_repository
            .find_by_category(category)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 商品を更新
    pub async fn update_product(
        &self,
        id: &str,
        name: Option<String>,
        description: Option<String>,
        price: Option<i32>,
        stock_quantity: Option<i32>,
        category: Option<String>,
    ) -> Result<Product, AppError> {
        // 現在の商品を取得
        let mut product = self
            .product_repository
            .find_by_id(id)
            .await
            .map_err(|e| AppError::database(&e.to_string()))?
            .ok_or_else(|| AppError::not_found("商品が見つかりません"))?;

        // バリデーションと更新
        if let Some(name) = name {
            if name.trim().is_empty() {
                return Err(AppError::validation("商品名は必須です"));
            }
            product.update_name(name);
        }

        if let Some(price) = price {
            if price <= 0 {
                return Err(AppError::validation(
                    "価格は0より大きい値である必要があります",
                ));
            }
            product.update_price(price);
        }

        if let Some(stock_quantity) = stock_quantity {
            if stock_quantity < 0 {
                return Err(AppError::validation(
                    "在庫数は0以上の値である必要があります",
                ));
            }
            product.update_stock_quantity(stock_quantity);
        }

        if let Some(description) = description {
            product.description = Some(description);
            product.updated_at = chrono::Utc::now();
        }

        if let Some(category) = category {
            product.category = Some(category);
            product.updated_at = chrono::Utc::now();
        }

        self.product_repository
            .update(product)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 商品を削除
    pub async fn delete_product(&self, id: &str) -> Result<bool, AppError> {
        self.product_repository
            .delete(id)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 在庫を減らす
    pub async fn decrease_stock(&self, id: &str, quantity: i32) -> Result<Product, AppError> {
        if quantity <= 0 {
            return Err(AppError::validation(
                "減らす数量は0より大きい値である必要があります",
            ));
        }

        let mut product = self
            .product_repository
            .find_by_id(id)
            .await
            .map_err(|e| AppError::database(&e.to_string()))?
            .ok_or_else(|| AppError::not_found("商品が見つかりません"))?;

        product
            .decrease_stock(quantity)
            .map_err(|e| AppError::business_logic(&e.to_string()))?;

        self.product_repository
            .update(product)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 在庫を増やす
    pub async fn increase_stock(&self, id: &str, quantity: i32) -> Result<Product, AppError> {
        if quantity <= 0 {
            return Err(AppError::validation(
                "増やす数量は0より大きい値である必要があります",
            ));
        }

        let mut product = self
            .product_repository
            .find_by_id(id)
            .await
            .map_err(|e| AppError::database(&e.to_string()))?
            .ok_or_else(|| AppError::not_found("商品が見つかりません"))?;

        product.increase_stock(quantity);

        self.product_repository
            .update(product)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }

    /// 在庫を更新
    pub async fn update_stock(&self, id: &str, quantity: i32) -> Result<Product, AppError> {
        if quantity < 0 {
            return Err(AppError::validation(
                "在庫数は0以上の値である必要があります",
            ));
        }
        self.product_repository
            .update_stock(id, quantity)
            .await
            .map_err(|e| AppError::database(&e.to_string()))
    }
}
