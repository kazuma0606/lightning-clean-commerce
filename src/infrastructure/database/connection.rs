use sqlx::SqlitePool;

pub struct DatabaseConnection {
    pool: SqlitePool,
}

impl DatabaseConnection {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
