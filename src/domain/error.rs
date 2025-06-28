// src/domain/error.rs

use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    DatabaseError(String),
    BusinessLogicError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::BusinessLogicError(msg) => write!(f, "Business Logic Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// 便利なコンストラクタ
impl AppError {
    pub fn not_found(msg: &str) -> Self {
        AppError::NotFound(msg.to_string())
    }

    pub fn validation(msg: &str) -> Self {
        AppError::ValidationError(msg.to_string())
    }

    pub fn database(msg: &str) -> Self {
        AppError::DatabaseError(msg.to_string())
    }

    pub fn business_logic(msg: &str) -> Self {
        AppError::BusinessLogicError(msg.to_string())
    }
}

// 他のエラーからの変換
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}
