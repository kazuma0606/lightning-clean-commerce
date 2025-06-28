use crate::application::dto::auth_dto::RegisterRequest;
use crate::domain::repository::user_repository::UserRepository;

pub struct RegisterUseCase<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> RegisterUseCase<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, request: RegisterRequest) -> Result<(), R::Error> {
        // 実装は後で追加
        todo!("Register use case implementation")
    }
}
