# Usage Guide: RSPC-TS-FRONT 商品管理API

このアプリケーションはRust + Axum + SQLiteで構築された商品管理APIサーバーです。

---

## 1. セットアップ

### 依存関係のインストール
```
cargo build
```

### データベースファイルの準備
- デフォルトでは `./products.db` がプロジェクト直下に作成されます。
- サブディレクトリを使う場合は、事前にディレクトリを作成してください。
  例: `mkdir data` → `DATABASE_URL=sqlite:./data/products.db`

---

## 2. サーバーの起動

```
cargo run
```

- デフォルトで `http://localhost:3000` でAPIサーバーが起動します。
- 環境変数 `DATABASE_URL` でSQLiteファイルのパスを変更できます。

---

## 3. 主なAPIエンドポイント

すべてのエンドポイントは `/api` プレフィックス付きです。

### 商品一覧取得
- `GET /api/products`

### 商品作成
- `POST /api/products`
- リクエストBody例:
```json
{
  "name": "サンプル商品",
  "description": "説明文",
  "price": 1000,
  "stock_quantity": 10,
  "category": "食品"
}
```

### 商品詳細取得
- `GET /api/products/{id}`

### 商品更新
- `PUT /api/products/{id}`
- リクエストBody例:
```json
{
  "name": "新しい商品名",
  "price": 1200
}
```

### 商品削除
- `DELETE /api/products/{id}`

### 商品検索
- `POST /api/products/search`
- リクエストBody例:
```json
{
  "name": "検索ワード"
}
```

### 在庫数の更新
- `PUT /api/products/{id}/stock`
- リクエストBody例:
```json
{
  "quantity": 20
}
```

### 在庫を増やす
- `PUT /api/products/{id}/stock/increase`
- リクエストBody例:
```json
{
  "quantity": 5
}
```

### 在庫を減らす
- `PUT /api/products/{id}/stock/decrease`
- リクエストBody例:
```json
{
  "quantity": 2
}
```

---

## 4. レスポンス例

すべてのAPIは以下のようなJSON形式でレスポンスします：
```json
{
  "success": true,
  "data": {...},
  "message": null
}
```

エラー時：
```json
{
  "success": false,
  "data": null,
  "message": "エラーメッセージ"
}
```

---

## 5. 注意事項
- CORSは全許可設定です。フロントエンドから直接APIを叩けます。
- データベースファイルのパスやサーバーポートは必要に応じて変更してください。

---

## 6. 開発・テスト
- Rustの標準的な方法でテスト・開発が可能です。
- PRやIssueはお気軽にどうぞ。 