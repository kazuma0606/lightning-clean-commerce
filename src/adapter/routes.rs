// ルーティング設定

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
};
use serde_json::json;
use std::sync::Arc;

use crate::api::{
    ApiResponse, CreateProductRequest, SearchProductsRequest, UpdateProductRequest,
    UpdateStockRequest,
};
use crate::application::usecase::ProductUseCase;
use crate::domain::repository::ProductRepository;

/// アプリケーション状態
pub struct AppState<R>
where
    R: ProductRepository + Send + Sync + 'static,
{
    pub product_use_case: ProductUseCase<R>,
}

/// 商品管理のAxumルーターを作成
pub fn create_product_router<R>() -> Router<Arc<AppState<R>>>
where
    R: ProductRepository + Send + Sync + 'static,
{
    Router::new()
        .route("/products", get(get_all_products))
        .route("/products", post(create_product))
        .route("/products/:id", get(get_product_by_id))
        .route("/products/:id", put(update_product))
        .route("/products/:id", delete(delete_product))
        .route("/products/search", post(search_products))
        .route("/products/:id/stock", put(update_stock))
        .route("/products/:id/stock/increase", put(increase_stock))
        .route("/products/:id/stock/decrease", put(decrease_stock))
}

/// 全商品を取得
async fn get_all_products<R>(
    State(state): State<Arc<AppState<R>>>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state.product_use_case.get_all_products().await {
        Ok(products) => Ok(Json(json!({
            "success": true,
            "data": products,
            "message": null
        }))),
        Err(e) => {
            tracing::error!("Failed to get all products: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// 商品を作成
async fn create_product<R>(
    State(state): State<Arc<AppState<R>>>,
    Json(request): Json<CreateProductRequest>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state
        .product_use_case
        .create_product(
            request.name,
            request.description,
            request.price,
            request.stock_quantity,
            request.category,
        )
        .await
    {
        Ok(product) => Ok(Json(json!({
            "success": true,
            "data": product,
            "message": "商品が正常に作成されました"
        }))),
        Err(e) => {
            tracing::error!("Failed to create product: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// IDで商品を取得
async fn get_product_by_id<R>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state.product_use_case.get_product(&id).await {
        Ok(Some(product)) => Ok(Json(json!({
            "success": true,
            "data": product,
            "message": null
        }))),
        Ok(None) => Ok(Json(json!({
            "success": false,
            "data": null,
            "message": "商品が見つかりません"
        }))),
        Err(e) => {
            tracing::error!("Failed to get product by id: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// 商品を更新
async fn update_product<R>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state
        .product_use_case
        .update_product(
            &id,
            request.name,
            request.description,
            request.price,
            request.stock_quantity,
            request.category,
        )
        .await
    {
        Ok(product) => Ok(Json(json!({
            "success": true,
            "data": product,
            "message": "商品が正常に更新されました"
        }))),
        Err(e) => {
            tracing::error!("Failed to update product: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// 商品を削除
async fn delete_product<R>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state.product_use_case.delete_product(&id).await {
        Ok(true) => Ok(Json(json!({
            "success": true,
            "data": null,
            "message": "商品が正常に削除されました"
        }))),
        Ok(false) => Ok(Json(json!({
            "success": false,
            "data": null,
            "message": "商品が見つかりません"
        }))),
        Err(e) => {
            tracing::error!("Failed to delete product: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// 商品を検索
async fn search_products<R>(
    State(state): State<Arc<AppState<R>>>,
    Json(request): Json<SearchProductsRequest>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    let result = if let Some(name) = request.name {
        state.product_use_case.search_products_by_name(&name).await
    } else if let Some(category) = request.category {
        state
            .product_use_case
            .search_products_by_category(&category)
            .await
    } else {
        state.product_use_case.get_all_products().await
    };

    match result {
        Ok(products) => Ok(Json(json!({
            "success": true,
            "data": products,
            "message": null
        }))),
        Err(e) => {
            tracing::error!("Failed to search products: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// 在庫を更新
async fn update_stock<R>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateStockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state
        .product_use_case
        .update_stock(&id, request.quantity)
        .await
    {
        Ok(product) => Ok(Json(json!({
            "success": true,
            "data": product,
            "message": "在庫が正常に更新されました"
        }))),
        Err(e) => {
            tracing::error!("Failed to update stock: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// 在庫を増やす
async fn increase_stock<R>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateStockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state
        .product_use_case
        .increase_stock(&id, request.quantity)
        .await
    {
        Ok(product) => Ok(Json(json!({
            "success": true,
            "data": product,
            "message": "在庫が正常に増加されました"
        }))),
        Err(e) => {
            tracing::error!("Failed to increase stock: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}

/// 在庫を減らす
async fn decrease_stock<R>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateStockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode>
where
    R: ProductRepository + Send + Sync + 'static,
{
    match state
        .product_use_case
        .decrease_stock(&id, request.quantity)
        .await
    {
        Ok(product) => Ok(Json(json!({
            "success": true,
            "data": product,
            "message": "在庫が正常に減少されました"
        }))),
        Err(e) => {
            tracing::error!("Failed to decrease stock: {}", e);
            Ok(Json(json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            })))
        }
    }
}
