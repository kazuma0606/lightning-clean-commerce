use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}

impl ErrorResponse {
    pub fn new(error: String, message: String, status_code: u16) -> Self {
        Self {
            error,
            message,
            status_code,
        }
    }
}
