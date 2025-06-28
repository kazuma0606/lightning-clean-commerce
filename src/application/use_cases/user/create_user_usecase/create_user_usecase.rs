use crate::application::dto::user_dto::create_user_request_dto::CreateUserRequestDto;
use crate::application::dto::user_dto::create_user_response_dto::CreateUserResponseDto;
use crate::domain::repository::user_repository::UserRepository;

pub struct CreateUserUseCase<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> CreateUserUseCase<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        request: CreateUserRequestDto,
    ) -> Result<CreateUserResponseDto, R::Error> {
        // 実装は後で追加
        todo!("Create user use case implementation")
    }
}
