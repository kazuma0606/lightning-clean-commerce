use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponseDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub message: String,
}
