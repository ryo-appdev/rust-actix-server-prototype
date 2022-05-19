# rust-actix-server-prototype

本プロジェクトは、「Rust/Actix Example」をベースに自家製公開サーバーの土台を作成するためのプロジェクトです。

### Rust/Actix Example

[![Build Status](https://travis-ci.com/ddimaria/rust-actix-example.svg?branch=master)](https://travis-ci.com/ddimaria/rust-actix-example)

### 選定理由

欲しい機能がコンパクトにまとめられていることが理由です。
パフォーマスと安全性を兼ねる Rust によるバックエンド実装に興味があり、
当初は Web サーバーとして容易に稼働し、DB 連携ができれば良いと考えていましたが、
以下に挙げるように、ユーザー認証やパスワード暗号化機能など、
実用途にも使用できそうな機能がまとめられています。

### 主な機能

- Actix Web HTTP Server
- Multi-Database Support (Postgres, MySQL, Sqlite)
- JWT Support
- Async Caching Layer with a Simple API
- Custom Errors and HTTP Payload/Json Validation
- Secure Argon2i Password Hashing
- CORS Support
- Unit and Integration Tests
- Test Coverage Reports

など。詳細は Rust/Actix Example のリンクを参照

## 作成ログ

### 〜2022/05
- Docker Compose 化
```
最近の 'DevOps' 流行りの理由の一端を実感した。これは便利。
HTTPS で若干つまずくもこれも実現。
マルチステージビルドによりイメージサイズを削減。
シングルバイナリ化も試行したがマルチ DB の影響か断念。今後の課題
```
- HTTPS 化
```
実運用では必須となるため HTTPS 化の修正を追加。
実装自体は本家の修正例もあり容易だった。mkcert 便利。
```
- 依存クレートの最新化
```
依存クレート内での干渉なのか完全に現時点での最新化まではできていない。
バージョンアップによりクレート内で構成が大きく変わるものもあったが、
ビルド時の修正提案ログが非常に参考になった。やはりここは他の言語とは大きく異なる。
クレートによりマイグレーションテキストがあることや、
テストコードが含まれていることなどもコード修正の参考になった。
```
- 環境構築
```
- rustfmt、Postman 導入
- Docker コンテナへの移行
- REDIS サーバーの実行、Postgres の実行とコネクション調整 
```
