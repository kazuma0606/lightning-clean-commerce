use crate::application::dto::auth_dto::{LoginRequest, LoginResponse};
use crate::domain::entity::user::User;
use crate::domain::repository::user_repository::UserRepository;

pub struct LoginUseCase<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> LoginUseCase<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, request: LoginRequest) -> Result<LoginResponse, R::Error> {
        // 実装は後で追加
        todo!("Login use case implementation")
    }
}
