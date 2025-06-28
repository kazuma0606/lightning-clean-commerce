use crate::application::dto::user_dto::user_dto::UserDto;
use crate::domain::repository::user_repository::UserRepository;

pub struct UserManagementUseCase<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> UserManagementUseCase<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserDto>, R::Error> {
        // 実装は後で追加
        todo!("Get all users implementation")
    }

    pub async fn deactivate_user(&self, user_id: String) -> Result<UserDto, R::Error> {
        // 実装は後で追加
        todo!("Deactivate user implementation")
    }
}
