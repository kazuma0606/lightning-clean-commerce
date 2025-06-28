use axum::Router;

pub async fn run() {
    let app: Router = Router::new();

    println!("Server starting on http://localhost:3000");
    // 実際のサーバー起動処理は後で実装
}
