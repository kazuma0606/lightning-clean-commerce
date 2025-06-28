use crate::application::dto::review_dto::ReviewDto;
use crate::domain::repository::review_repository::ReviewRepository;

pub struct UpdateReviewUseCase<R>
where
    R: ReviewRepository,
{
    review_repository: R,
}

impl<R> UpdateReviewUseCase<R>
where
    R: ReviewRepository,
{
    pub fn new(review_repository: R) -> Self {
        Self { review_repository }
    }

    pub async fn execute(
        &self,
        review_id: String,
        review_data: ReviewDto,
    ) -> Result<ReviewDto, R::Error> {
        // 実装は後で追加
        todo!("Update review use case implementation")
    }
}
