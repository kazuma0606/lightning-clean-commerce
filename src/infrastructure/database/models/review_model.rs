use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReviewModel {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
