// ドメイン層（Entities, Repository Traits）
pub mod entities;
pub mod error;
pub mod repository;
pub use entities::{Product, User};
pub use repository::{ProductRepository, UserRepository};
