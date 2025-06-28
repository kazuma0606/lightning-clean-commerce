use crate::application::dto::review_dto::{CreateReviewRequest, ReviewDto};
use crate::domain::repository::review_repository::ReviewRepository;

pub struct CreateReviewUseCase<R>
where
    R: ReviewRepository,
{
    review_repository: R,
}

impl<R> CreateReviewUseCase<R>
where
    R: ReviewRepository,
{
    pub fn new(review_repository: R) -> Self {
        Self { review_repository }
    }

    pub async fn execute(
        &self,
        user_id: String,
        request: CreateReviewRequest,
    ) -> Result<ReviewDto, R::Error> {
        // 実装は後で追加
        todo!("Create review use case implementation")
    }
}
