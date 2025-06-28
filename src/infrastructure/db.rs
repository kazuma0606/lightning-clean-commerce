// DB接続やクエリ実装


use sqlx::{sqlite::SqlitePool, Row};
use std::error::Error;
use std::fmt;

/// データベースエラー
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    MigrationError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DatabaseError::MigrationError(msg) => write!(f, "Migration error: {}", msg),
        }
    }
}

impl Error for DatabaseError {}

/// データベース接続を管理する構造体
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// 新しいデータベース接続を作成
    pub async fn new(database_url: &str) -> Result<Self, DatabaseError> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Self { pool })
    }

    /// データベースの初期化（テーブル作成）
    pub async fn initialize(&self) -> Result<(), DatabaseError> {
        // ユーザーテーブルの作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // 商品テーブルの作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                price INTEGER NOT NULL,
                stock_quantity INTEGER NOT NULL DEFAULT 0,
                category TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        Ok(())
    }

    /// プールへの参照を取得
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
} 