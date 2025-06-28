use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderModel {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub total_amount: f64,
    pub items: String, // JSON string
    pub shipping_address: String,
    pub created_at: String,
    pub updated_at: String,
}
