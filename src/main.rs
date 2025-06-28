// mod adapter;
// mod api;
// mod application;
// mod domain;
// mod infrastructure;
// mod state;

use axum::Router;
use axum::serve;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use rspc_ts_front::adapter::routes::product_routes::{AppState, create_product_router};
use rspc_ts_front::application::use_cases::product::manage_product_use_case::ProductUseCase;
use rspc_ts_front::infrastructure::db::Database;
use rspc_ts_front::infrastructure::repository_impl::SqliteProductRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ログの初期化
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting product management API server...");

    // データベース接続の初期化
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./data/products.db".to_string());

    let database = Database::new(&database_url).await?;
    database.initialize().await?;
    tracing::info!("Database initialized successfully");

    // リポジトリとユースケースの初期化
    let product_repository = SqliteProductRepository::new(database.pool().clone());
    let product_use_case = ProductUseCase::new(product_repository);

    // アプリケーション状態の作成
    let app_state = Arc::new(AppState { product_use_case });

    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ルーターの作成
    let app = Router::new()
        .nest("/api", create_product_router())
        .layer(cors)
        .with_state(app_state);

    // サーバーの起動
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}
