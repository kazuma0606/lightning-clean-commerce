use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AddressModel {
    pub id: String,
    pub user_id: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}
