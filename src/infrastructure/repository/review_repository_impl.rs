use crate::domain::entity::review::Review;
use crate::domain::repository::review_repository::ReviewRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct ReviewRepositoryImpl {
    pool: SqlitePool,
}

impl ReviewRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewRepository for ReviewRepositoryImpl {
    type Error = sqlx::Error;

    async fn save(&self, review: Review) -> Result<Review, Self::Error> {
        todo!("Save review implementation")
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Review>, Self::Error> {
        todo!("Find review by id implementation")
    }

    async fn find_by_product_id(&self, product_id: &str) -> Result<Vec<Review>, Self::Error> {
        todo!("Find reviews by product id implementation")
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Review>, Self::Error> {
        todo!("Find reviews by user id implementation")
    }

    async fn update(&self, review: Review) -> Result<Review, Self::Error> {
        todo!("Update review implementation")
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        todo!("Delete review implementation")
    }
}
