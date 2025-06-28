use sqlx::SqlitePool;

pub struct DatabaseMigration;

impl DatabaseMigration {
    pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // マイグレーション実行（後で実装）
        Ok(())
    }
}
