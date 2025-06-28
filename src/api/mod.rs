// API スキーマ・レスポンス

use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

/// 商品作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub stock_quantity: i32,
    pub category: Option<String>,
}

/// 商品更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub stock_quantity: Option<i32>,
    pub category: Option<String>,
}

/// 在庫更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateStockRequest {
    pub quantity: i32,
}

/// 商品検索リクエスト
#[derive(Debug, Deserialize)]
pub struct SearchProductsRequest {
    pub name: Option<String>,
    pub category: Option<String>,
}
