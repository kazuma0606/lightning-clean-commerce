use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartModel {
    pub id: String,
    pub user_id: String,
    pub items: String, // JSON string
    pub created_at: String,
    pub updated_at: String,
}
