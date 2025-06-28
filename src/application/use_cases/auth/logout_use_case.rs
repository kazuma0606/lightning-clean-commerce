pub struct LogoutUseCase;

impl LogoutUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, user_id: String) -> Result<(), String> {
        // 実装は後で追加
        todo!("Logout use case implementation")
    }
}
