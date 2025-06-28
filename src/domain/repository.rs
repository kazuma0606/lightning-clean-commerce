use crate::domain::entities::{Product, User};
use async_trait::async_trait;
use std::error::Error;

/// ユーザーリポジトリのトレイト
#[async_trait]
pub trait UserRepository {
    /// エラー型
    type Error: Error + Send + Sync;

    /// ユーザーを保存
    async fn save(&self, user: User) -> Result<User, Self::Error>;

    /// IDでユーザーを取得
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Self::Error>;

    /// ユーザー名でユーザーを取得
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Self::Error>;

    /// メールアドレスでユーザーを取得
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Self::Error>;

    /// 全ユーザーを取得
    async fn find_all(&self) -> Result<Vec<User>, Self::Error>;

    /// ユーザーを更新
    async fn update(&self, user: User) -> Result<User, Self::Error>;

    /// ユーザーを削除
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}

/// 商品リポジトリのトレイト
#[async_trait]
pub trait ProductRepository {
    /// エラー型
    type Error: Error + Send + Sync + 'static;

    /// 商品を保存
    async fn save(&self, product: Product) -> Result<Product, Self::Error>;

    /// IDで商品を取得
    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, Self::Error>;

    /// 商品名で商品を検索
    async fn find_by_name(&self, name: &str) -> Result<Vec<Product>, Self::Error>;

    /// カテゴリで商品を検索
    async fn find_by_category(&self, category: &str) -> Result<Vec<Product>, Self::Error>;

    /// 全商品を取得
    async fn find_all(&self) -> Result<Vec<Product>, Self::Error>;

    /// 商品を更新
    async fn update(&self, product: Product) -> Result<Product, Self::Error>;

    /// 商品を削除
    async fn delete(&self, id: &str) -> Result<bool, Self::Error>;

    /// 在庫数を更新
    async fn update_stock(&self, id: &str, quantity: i32) -> Result<Product, Self::Error>;
}
