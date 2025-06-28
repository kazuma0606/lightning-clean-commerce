use crate::domain::entity::address::Address;
use crate::domain::repository::address_repository::AddressRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct AddressRepositoryImpl {
    pool: SqlitePool,
}

impl AddressRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AddressRepository for AddressRepositoryImpl {
    type Error = sqlx::Error;

    async fn save(&self, address: Address) -> Result<Address, Self::Error> {
        todo!("Save address implementation")
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Address>, Self::Error> {
        todo!("Find address by id implementation")
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Address>, Self::Error> {
        todo!("Find addresses by user id implementation")
    }

    async fn update(&self, address: Address) -> Result<Address, Self::Error> {
        todo!("Update address implementation")
    }

    async fn delete(&self, id: &str) -> Result<bool, Self::Error> {
        todo!("Delete address implementation")
    }
}
