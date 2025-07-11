# ⚡ Lightning Clean Commerce

🚀 **AI駆動開発で構築されたECサイトバックエンド（クリーンアーキテクチャ）**

> **Claude + Cursor による電光石火の開発体験**  
> 従来なら数週間かかる機能を数時間で実装 - これがAI時代の開発速度

## 📋 概要

このプロジェクトは、**Claude**と**Cursor**を活用してRustで構築されたモダンなECサイトのバックエンドAPIです。クリーンアーキテクチャを採用し、TiDBを使用して高パフォーマンスと拡張性を実現しています。

### 🌟 特徴

- ⚡ **Lightning Fast Development**: AI駆動による爆速開発
- 🏗️ **Clean Architecture**: 保守性と拡張性を重視した設計
- 🤖 **AI-Assisted**: Claude & Cursor による開発支援
- 🦀 **Rust Powered**: 高パフォーマンス・メモリセーフ
- 🗄️ **TiDB Integration**: 分散データベースによる拡張性

### 🎯 実装済み機能

- ✅ **商品管理**: CRUD操作・在庫管理・検索機能
- ⚡ **API エンドポイント**: RESTful API設計
- 🛡️ **エラーハンドリング**: 統一されたエラー管理
- 📊 **JSON レスポンス**: 一貫したレスポンス形式
- 🔄 **在庫操作**: 増減・更新機能

### 🚧 計画中の機能

- 🔐 **ユーザー管理**: 登録・認証・プロフィール管理
- 🛒 **ショッピングカート**: カート操作・セッション管理
- 📋 **注文管理**: 注文処理・ステータス管理・履歴
- ⭐ **レビューシステム**: 商品レビュー・評価機能
- 🔧 **管理者機能**: 商品・注文・ユーザー管理

## 🏗️ アーキテクチャ

### Lightning Clean Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Adapter Layer                            │
│  (Web API, Controllers, Routes, Middleware)                │
├─────────────────────────────────────────────────────────────┤
│                 Application Layer                           │
│        (Use Cases, DTOs, Services)                         │
├─────────────────────────────────────────────────────────────┤
│                   Domain Layer                              │
│    (Entities, Repository Traits, Business Logic)           │
├─────────────────────────────────────────────────────────────┤
│                Infrastructure Layer                         │
│ (Database, External Services, Repository Implementations)  │
└─────────────────────────────────────────────────────────────┘
```

### ディレクトリ構造

```
src/
├── adapter/           # Web API層（Controllers, Routes, Middleware）
├── application/       # アプリケーション層（Use Cases, DTOs）
├── domain/           # ドメイン層（Entities, Repository Traits）
├── infrastructure/   # インフラ層（Database, External Services）
├── api/             # API スキーマ・レスポンス
└── state/           # アプリケーション状態管理
```

## 🛠️ 技術スタック

### **Backend**
- **Language**: Rust 2024 Edition
- **Web Framework**: Axum 0.7
- **Database**: TiDB (MySQL互換)
- **ORM**: SQLx 0.7
- **Authentication**: JWT + bcrypt (予定)
- **Async Runtime**: Tokio

### **Infrastructure**
- **Database**: TiDB via Docker
- **Cache**: Redis (予定)
- **Storage**: MinIO (S3互換・予定)
- **Container**: Docker & Docker Compose

### **Development Tools**
- **AI Assistant**: Claude Sonnet 4 & Cursor
- **DB Management**: phpMyAdmin
- **Logging**: tracing + tracing-subscriber
- **Testing**: tokio-test
- **Validation**: validator

## 🚀 セットアップ

### 前提条件

- [Rust](https://rustup.rs/) (1.70以降)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Git](https://git-scm.com/)

### 1. プロジェクトクローン

```bash
git clone https://github.com/kazuma0606/lightning-clean-commerce.git
cd lightning-clean-commerce
```

### 2. 環境設定

```bash
# .envファイル作成
cp .env.example .env

# 必要に応じて.envファイルを編集
notepad .env  # Windows
```

### 3. Docker環境起動

```bash
# Windows
scripts\start.bat

# または直接実行
docker-compose up -d
```

### 4. 依存関係インストール

```bash
cargo build
```

### 5. サーバー起動

```bash
cargo run
```

## 🌐 利用可能なサービス

| サービス | URL | 用途 |
|---------|-----|------|
| **API Server** | http://localhost:3000 | メインAPI |
| **TiDB** | localhost:4000 | データベース |
| **phpMyAdmin** | http://localhost:8080 | DB管理 |

## 📚 API ドキュメント

### 商品管理 API

#### 商品作成
```bash
curl -X POST http://localhost:3000/api/products \
  -H "Content-Type: application/json" \
  -d '{"name":"Sample Product","description":"Sample description","price":1000,"stock_quantity":5,"category":"Food"}'
```

#### 商品一覧取得
```bash
curl -X GET http://localhost:3000/api/products
```

#### 商品更新
```bash
curl -X PUT http://localhost:3000/api/products/{id} \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Product","price":1200}'
```

#### 在庫管理
```bash
# 在庫更新
curl -X PUT http://localhost:3000/api/products/{id}/stock \
  -H "Content-Type: application/json" \
  -d '{"quantity":20}'

# 在庫増加
curl -X PUT http://localhost:3000/api/products/{id}/stock/increase \
  -H "Content-Type: application/json" \
  -d '{"quantity":3}'

# 在庫減少
curl -X PUT http://localhost:3000/api/products/{id}/stock/decrease \
  -H "Content-Type: application/json" \
  -d '{"quantity":2}'
```

### レスポンス形式

#### 成功レスポンス
```json
{
  "success": true,
  "data": {
    "id": "uuid-here",
    "name": "Sample Product",
    "price": 1000,
    "stock_quantity": 5,
    "created_at": "2025-06-28T12:12:07.681777600Z"
  },
  "message": "商品が正常に作成されました"
}
```

#### エラーレスポンス
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "入力データが無効です",
    "details": {}
  }
}
```

## 🤖 AI開発の軌跡

### 開発フロー

1. **設計相談**: Claude が要件を整理・アーキテクチャを提案
2. **コード生成**: Cursor が実装を支援
3. **エラー解決**: Claude & Cursor で問題を即座に解決
4. **機能拡張**: 段階的に機能を追加

### AI活用のポイント

- 📋 **明確な要求**: 具体的な機能要件を提示
- 🔄 **段階的開発**: 小さな単位で確実に進行
- 🐛 **エラー共有**: コンパイルエラーを即座に相談
- 🧪 **動作確認**: 各段階でテスト・検証

### 開発速度の比較

| 従来の開発 | AI駆動開発 | 効率化 |
|-----------|-----------|--------|
| 設計: 1-2日 | 設計: 1-2時間 | 🚀 **12-24倍** |
| 実装: 3-5日 | 実装: 2-4時間 | 🚀 **18-30倍** |
| デバッグ: 1-2日 | デバッグ: 10-30分 | 🚀 **24-48倍** |

## 🧪 テスト

```bash
# 全テスト実行
cargo test

# 統合テスト
cargo test --test integration

# 商品API手動テスト
scripts\test-api.bat
```

## 📊 開発ツール

### データベース操作

```bash
# TiDB接続
scripts\connect-tidb.bat

# phpMyAdmin
start http://localhost:8080
```

### 環境管理

```bash
# 環境起動
scripts\start.bat

# 環境停止
scripts\stop.bat

# 完全クリーンアップ
scripts\cleanup.bat
```

## 🚀 デプロイ

```bash
# リリースビルド
cargo build --release

# Docker本番環境
docker-compose -f docker-compose.prod.yml up -d
```

## 🔧 開発ガイド

### AI開発のベストプラクティス

1. **段階的アプローチ**: 大きな機能を小さく分割
2. **エラー即相談**: コンパイルエラーをAIと即座に解決
3. **動作確認**: 各段階で必ずテスト実行
4. **コード理解**: AIが生成したコードの理解を怠らない

### コード規約

- **命名規則**: snake_case（Rust標準）
- **エラーハンドリング**: `Result<T, E>`を使用
- **非同期処理**: `async/await`を使用
- **コメント**: 複雑なロジックには説明を追加

## 🎉 成果物

このプロジェクトは、**Claude + Cursor**による AI駆動開発の実証実験として：

✅ **クリーンアーキテクチャ**の商品管理APIを数時間で実装  
✅ **CRUD操作 + 在庫管理**の完全動作確認  
✅ **適切なエラーハンドリング**と統一レスポンス形式  
✅ **Docker環境**での本格的な開発環境構築  

を達成しました。

## 🤝 コントリビューション

AI駆動開発に興味のある方、ぜひご参加ください！

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/ai-enhanced-feature`)
3. AI支援で実装・テスト
4. プルリクエストを作成

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています。

## 👥 作成者

- **Yoshimura Hisanori** - *AI駆動開発実証* - [GitHub](https://github.com/kazuma0606/lightning-clean-commerce)

## 🙏 謝辞

- **[Claude](https://claude.ai/)** - 設計支援・問題解決
- **[Cursor](https://cursor.sh/)** - AI駆動コード生成
- **[TiDB](https://github.com/pingcap/tidb)** - 高性能分散データベース
- **[Axum](https://github.com/tokio-rs/axum)** - モダンなWebフレームワーク

---

## 🔄 更新履歴

### [0.1.0] - 2025-06-28
- 🚀 初期プロジェクト設定（AI駆動）
- 🏗️ クリーンアーキテクチャ基盤構築
- 🗄️ TiDB + Docker環境構築
- 📦 商品管理API実装完了
- ⚡ 在庫管理機能追加

---

**⚡ Lightning Fast Development with AI! 🤖🦀**

> *"コードが書けないエンジニアの時代は終わった。  
> 今度は、AIと協働できないエンジニアの時代が終わる。"*