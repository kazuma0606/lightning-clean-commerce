use crate::domain::entity::address::Address;
use crate::domain::repository::address_repository::AddressRepository;

pub struct ManageAddressUseCase<R>
where
    R: AddressRepository,
{
    address_repository: R,
}

impl<R> ManageAddressUseCase<R>
where
    R: AddressRepository,
{
    pub fn new(address_repository: R) -> Self {
        Self { address_repository }
    }

    pub async fn add_address(&self, address: Address) -> Result<Address, R::Error> {
        // 実装は後で追加
        todo!("Add address implementation")
    }

    pub async fn update_address(&self, address: Address) -> Result<Address, R::Error> {
        // 実装は後で追加
        todo!("Update address implementation")
    }

    pub async fn delete_address(&self, address_id: String) -> Result<bool, R::Error> {
        // 実装は後で追加
        todo!("Delete address implementation")
    }
}
