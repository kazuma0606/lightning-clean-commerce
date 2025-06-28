use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewDto {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    pub product_id: String,
    pub rating: i32,
    pub comment: Option<String>,
}
