// エンティティ（ドメインモデル）

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

/// ユーザーエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// ユーザーID
    pub id: String,
    /// ユーザー名
    pub username: String,
    /// メールアドレス
    pub email: String,
    /// 作成日時
    pub created_at: SystemTime,
    /// 更新日時
    pub updated_at: SystemTime,
}

impl User {
    /// 新しいユーザーを作成
    pub fn new(id: String, username: String, email: String) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            username,
            email,
            created_at: now,
            updated_at: now,
        }
    }

    /// ユーザー名を更新
    pub fn update_username(&mut self, username: String) {
        self.username = username;
        self.updated_at = SystemTime::now();
    }

    /// メールアドレスを更新
    pub fn update_email(&mut self, email: String) {
        self.email = email;
        self.updated_at = SystemTime::now();
    }
}

/// 商品エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// 商品ID
    pub id: String,
    /// 商品名
    pub name: String,
    /// 商品説明
    pub description: Option<String>,
    /// 価格（円）
    pub price: i32,
    /// 在庫数
    pub stock_quantity: i32,
    /// カテゴリ
    pub category: Option<String>,
    /// 作成日時
    pub created_at: DateTime<Utc>,
    /// 更新日時
    pub updated_at: DateTime<Utc>,
}

impl Product {
    /// 新しい商品を作成
    pub fn new(
        name: String,
        description: Option<String>,
        price: i32,
        stock_quantity: i32,
        category: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            price,
            stock_quantity,
            category,
            created_at: now,
            updated_at: now,
        }
    }

    /// 商品名を更新
    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    /// 価格を更新
    pub fn update_price(&mut self, price: i32) {
        self.price = price;
        self.updated_at = Utc::now();
    }

    /// 在庫数を更新
    pub fn update_stock_quantity(&mut self, stock_quantity: i32) {
        self.stock_quantity = stock_quantity;
        self.updated_at = Utc::now();
    }

    /// 在庫を減らす
    pub fn decrease_stock(&mut self, quantity: i32) -> Result<(), String> {
        if self.stock_quantity < quantity {
            return Err("在庫が不足しています".to_string());
        }
        self.stock_quantity -= quantity;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 在庫を増やす
    pub fn increase_stock(&mut self, quantity: i32) {
        self.stock_quantity += quantity;
        self.updated_at = Utc::now();
    }
}
