use crate::domain::entity::category::Category;
use crate::domain::repository::category_repository::CategoryRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct CategoryRepositoryImpl {
    pool: SqlitePool,
}

impl CategoryRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for CategoryRepositoryImpl {
    type Error = sqlx::Error;

    async fn save(&self, category: Category) -> Result<Category, Self::Error> {
        todo!("Save category implementation")
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Category>, Self::Error> {
        todo!("Find category by id implementation")
    }

    async fn find_all(&self) -> Result<Vec<Category>, Self::Error> {
        todo!("Find all categories implementation")
    }

    async fn find_by_parent_id(&self, parent_id: &str) -> Result<Vec<Category>, Self::Error> {
        todo!("Find categories by parent id implementation")
    }

    async fn update(&self, category: Category) -> Result<Category, Self::Error> {
        todo!("Update category implementation")
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        todo!("Delete category implementation")
    }
}
