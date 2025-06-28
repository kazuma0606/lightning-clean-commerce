// リポジトリ実装

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Mutex;

use crate::domain::entity::product::Product;
use crate::domain::entity::user::User;
use crate::domain::repository::product_repository::ProductRepository;
use crate::domain::repository::user_repository::UserRepository;

/// インメモリユーザーリポジトリのエラー
#[derive(Debug)]
pub enum InMemoryUserRepositoryError {
    UserNotFound,
    UserAlreadyExists,
    DatabaseError(String),
}

impl fmt::Display for InMemoryUserRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InMemoryUserRepositoryError::UserNotFound => write!(f, "User not found"),
            InMemoryUserRepositoryError::UserAlreadyExists => write!(f, "User already exists"),
            InMemoryUserRepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for InMemoryUserRepositoryError {}

/// インメモリユーザーリポジトリ
pub struct InMemoryUserRepository {
    users: Mutex<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    /// 新しいインメモリユーザーリポジトリを作成
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    type Error = InMemoryUserRepositoryError;

    async fn save(&self, user: User) -> Result<User, Self::Error> {
        let mut users = self.users.lock().map_err(|e| {
            InMemoryUserRepositoryError::DatabaseError(format!("Failed to acquire lock: {}", e))
        })?;

        if users.contains_key(&user.id) {
            return Err(InMemoryUserRepositoryError::UserAlreadyExists);
        }

        let user_clone = user.clone();
        users.insert(user.id.clone(), user);
        Ok(user_clone)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Self::Error> {
        let users = self.users.lock().map_err(|e| {
            InMemoryUserRepositoryError::DatabaseError(format!("Failed to acquire lock: {}", e))
        })?;

        Ok(users.get(id).cloned())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Self::Error> {
        let users = self.users.lock().map_err(|e| {
            InMemoryUserRepositoryError::DatabaseError(format!("Failed to acquire lock: {}", e))
        })?;

        Ok(users
            .values()
            .find(|user| user.username == username)
            .cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Self::Error> {
        let users = self.users.lock().map_err(|e| {
            InMemoryUserRepositoryError::DatabaseError(format!("Failed to acquire lock: {}", e))
        })?;

        Ok(users.values().find(|user| user.email == email).cloned())
    }

    async fn find_all(&self) -> Result<Vec<User>, Self::Error> {
        let users = self.users.lock().map_err(|e| {
            InMemoryUserRepositoryError::DatabaseError(format!("Failed to acquire lock: {}", e))
        })?;

        Ok(users.values().cloned().collect())
    }

    async fn update(&self, user: User) -> Result<User, Self::Error> {
        let mut users = self.users.lock().map_err(|e| {
            InMemoryUserRepositoryError::DatabaseError(format!("Failed to acquire lock: {}", e))
        })?;

        if !users.contains_key(&user.id) {
            return Err(InMemoryUserRepositoryError::UserNotFound);
        }

        let user_clone = user.clone();
        users.insert(user.id.clone(), user);
        Ok(user_clone)
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        let mut users = self.users.lock().map_err(|e| {
            InMemoryUserRepositoryError::DatabaseError(format!("Failed to acquire lock: {}", e))
        })?;

        Ok(users.remove(id).is_some())
    }
}

/// SQLite商品リポジトリのエラー
#[derive(Debug)]
pub enum SqliteProductRepositoryError {
    ProductNotFound,
    ProductAlreadyExists,
    DatabaseError(String),
    InvalidData(String),
}

impl fmt::Display for SqliteProductRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqliteProductRepositoryError::ProductNotFound => write!(f, "Product not found"),
            SqliteProductRepositoryError::ProductAlreadyExists => {
                write!(f, "Product already exists")
            }
            SqliteProductRepositoryError::DatabaseError(msg) => {
                write!(f, "Database error: {}", msg)
            }
            SqliteProductRepositoryError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl Error for SqliteProductRepositoryError {}

/// SQLite商品リポジトリ
pub struct SqliteProductRepository {
    pool: SqlitePool,
}

impl SqliteProductRepository {
    /// 新しいSQLite商品リポジトリを作成
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 行からProductエンティティを作成
    fn row_to_product(
        &self,
        row: &sqlx::sqlite::SqliteRow,
    ) -> Result<Product, SqliteProductRepositoryError> {
        let id: String = row
            .try_get("id")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let name: String = row
            .try_get("name")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let description: Option<String> = row
            .try_get("description")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let price: i32 = row
            .try_get("price")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let stock_quantity: i32 = row
            .try_get("stock_quantity")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let category: Option<String> = row
            .try_get("category")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let created_at_str: String = row
            .try_get("created_at")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let updated_at_str: String = row
            .try_get("updated_at")
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| {
                SqliteProductRepositoryError::InvalidData(format!("Invalid created_at: {}", e))
            })?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|e| {
                SqliteProductRepositoryError::InvalidData(format!("Invalid updated_at: {}", e))
            })?
            .with_timezone(&Utc);

        Ok(Product {
            id,
            name,
            description,
            price,
            stock_quantity,
            category,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    type Error = SqliteProductRepositoryError;

    async fn save(&self, product: Product) -> Result<Product, Self::Error> {
        // 既存の商品をチェック
        let existing = sqlx::query("SELECT id FROM products WHERE id = ?")
            .bind(&product.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        if existing.is_some() {
            return Err(SqliteProductRepositoryError::ProductAlreadyExists);
        }

        // 商品を保存
        sqlx::query(
            r#"
            INSERT INTO products (id, name, description, price, stock_quantity, category, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&product.id)
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price)
        .bind(product.stock_quantity)
        .bind(&product.category)
        .bind(product.created_at.to_rfc3339())
        .bind(product.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        Ok(product)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, Self::Error> {
        let row = sqlx::query("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let product = self.row_to_product(&row)?;
                Ok(Some(product))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Product>, Self::Error> {
        let rows = sqlx::query("SELECT * FROM products WHERE name LIKE ?")
            .bind(format!("%{}%", name))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        for row in rows {
            let product = self.row_to_product(&row)?;
            products.push(product);
        }

        Ok(products)
    }

    async fn find_by_category(&self, category: &str) -> Result<Vec<Product>, Self::Error> {
        let rows = sqlx::query("SELECT * FROM products WHERE category = ?")
            .bind(category)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        for row in rows {
            let product = self.row_to_product(&row)?;
            products.push(product);
        }

        Ok(products)
    }

    async fn find_all(&self) -> Result<Vec<Product>, Self::Error> {
        let rows = sqlx::query("SELECT * FROM products ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        for row in rows {
            let product = self.row_to_product(&row)?;
            products.push(product);
        }

        Ok(products)
    }

    async fn update(&self, product: Product) -> Result<Product, Self::Error> {
        // 既存の商品をチェック
        let existing = sqlx::query("SELECT id FROM products WHERE id = ?")
            .bind(&product.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        if existing.is_none() {
            return Err(SqliteProductRepositoryError::ProductNotFound);
        }

        // 商品を更新
        sqlx::query(
            r#"
            UPDATE products 
            SET name = ?, description = ?, price = ?, stock_quantity = ?, category = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price)
        .bind(product.stock_quantity)
        .bind(&product.category)
        .bind(product.updated_at.to_rfc3339())
        .bind(&product.id)
        .execute(&self.pool)
        .await
        .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        Ok(product)
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| SqliteProductRepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_stock(&self, id: &str, quantity: i32) -> Result<Product, Self::Error> {
        // 現在の商品を取得
        let mut product = self
            .find_by_id(id)
            .await?
            .ok_or(SqliteProductRepositoryError::ProductNotFound)?;

        // 在庫を更新
        product.update_stock_quantity(quantity);

        // データベースを更新
        self.update(product.clone()).await?;

        Ok(product)
    }
}
