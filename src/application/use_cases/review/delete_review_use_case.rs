use crate::domain::repository::review_repository::ReviewRepository;

pub struct DeleteReviewUseCase<R>
where
    R: ReviewRepository,
{
    review_repository: R,
}

impl<R> DeleteReviewUseCase<R>
where
    R: ReviewRepository,
{
    pub fn new(review_repository: R) -> Self {
        Self { review_repository }
    }

    pub async fn execute(&self, review_id: String) -> Result<bool, R::Error> {
        // 実装は後で追加
        todo!("Delete review use case implementation")
    }
}
