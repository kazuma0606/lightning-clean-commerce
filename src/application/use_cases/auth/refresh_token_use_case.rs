use crate::application::dto::auth_dto::LoginResponse;

pub struct RefreshTokenUseCase;

impl RefreshTokenUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, refresh_token: String) -> Result<LoginResponse, String> {
        // 実装は後で追加
        todo!("Refresh token use case implementation")
    }
}
