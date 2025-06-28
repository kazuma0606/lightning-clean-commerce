use crate::application::use_cases::user::create_user_usecase::create_user_usecase::CreateUserUseCase;
use crate::domain::repository::user_repository::UserRepository;

pub struct CreateUserUseCaseImpl<R>
where
    R: UserRepository,
{
    inner: CreateUserUseCase<R>,
}

impl<R> CreateUserUseCaseImpl<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self {
            inner: CreateUserUseCase::new(user_repository),
        }
    }
}
