use crate::application::dto::user_dto::user_dto::UserDto;
use crate::domain::repository::user_repository::UserRepository;

pub struct GetUserProfileUseCase<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> GetUserProfileUseCase<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: String) -> Result<UserDto, R::Error> {
        // 実装は後で追加
        todo!("Get user profile use case implementation")
    }
}
