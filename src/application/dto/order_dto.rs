use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDto {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub total_amount: f64,
    pub items: Vec<OrderItemDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemDto {
    pub product_id: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<OrderItemDto>,
    pub shipping_address: String,
}
