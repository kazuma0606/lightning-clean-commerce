use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CartDto {
    pub id: String,
    pub user_id: String,
    pub items: Vec<CartItemDto>,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItemDto {
    pub product_id: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: String,
    pub quantity: i32,
}
