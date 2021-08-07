# [RESTful API in Sync & Async Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md)


**Table of Contents**

- [Intro](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#intro)
- General
  - [Project Setup](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#project-setup)
  - [Loading Environment Variables w/dotenv](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#loading-environment-variables-wdotenv)
  - [Handling Dates & Times w/chrono](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#handling-dates--times-wchrono)
  - [Logging w/fern](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#logging-wfern)
  - [JSON Serialization w/serde](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#json-serialization-wserde)
  - [Domain Modeling](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#domain-modeling)
- Sync Implementation
  - [SQL Schema Migrations w/diesel-cli](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#sql-schema-migrations-wdiesel-cli)
  - Executing SQL Queries w/Diesel
    - [Mapping DB Enums to Rust Enums](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#mapping-db-enums-to-rust-enums)
    - [Fetching Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#fetching-data)
    - [Inserting Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#inserting-data)
    - [Updating Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#updating-data)
    - [Deleting Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#deleting-data)
    - [Using a Connection Pool w/r2d2](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#using-a-connection-pool-wr2d2)
    - [Refactoring DB Operations Into a Module](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#refactoring-db-operations-into-a-module)
  - HTTP Routing w/Rocket
    - [Routing Basics](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#routing-basics)
    - [GET Requests](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#get-requests)
    - [POST & PATCH Requests](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#post--patch-requests)
    - [DELETE Requests](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#delete-requests)
    - [Refactoring API Routes Into a Module](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#refactoring-api-routes-into-a-module)
    - [Authentication](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#authentication)
- Async Implementation
  - [SQL Schema Migrations w/sqlx-cli](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#sql-schema-migrations-wsqlx-cli)
  - Executing SQL Queries w/sqlx
    - [Fetching Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#fetching-data-1)
    - [Inserting Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#inserting-data-1)
    - [Updating Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#updating-data-1)
    - [Deleting Data](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#deleting-data-1)
    - [Compile-Time Verification of SQL Queries](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#compile-time-verification-of-sql-queries)
    - [Using a Connection Pool w/sqlx](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#using-a-connection-pool-wsqlx)
    - [Refactoring DB Operations Into a Module](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#refactoring-db-operations-into-a-module-1)
  - HTTP Routing w/actix-web
    - [Routing Basics](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#routing-basics-1)
    - [GET Requests](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#get-requests-1)
    - [POST & PATCH Requests](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#post--patch-requests-1)
    - [DELETE Requests](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#delete-requests-1)
    - [Refactoring API Routes Into a Module](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#refactoring-api-routes-into-a-module-1)
    - [Authentication](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#authentication-1)
- Benchmarks
  - [Servers](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#servers)
  - [Methodology](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#methodology)
  - [Measuring Resource Usage](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#measuring-resource-usage)
  - Results
    - [Read-Only Workload](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#read-only-workload)
    - [Reads + Writes Workload](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#reads--writes-workload)
- Concluding Thoughts
  - [Diesel vs sqlx](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#diesel-vs-sqlx)
  - [Rocket vs actix-web](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#rocket-vs-actix-web)
  - [Sync Rust vs Async Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#sync-rust-vs-async-rust)
  - [Rust vs JS](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#rust-vs-js)
  - [In Summary](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#in-summary)
- [Discuss](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#discuss)
- [Notifications](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#notifications)
- [Further Reading](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md#further-reading)

---

### Intro

架空のカンバン方式のプロジェクト管理アプリのために、RustでRESTfulなAPIサーバーを実装してみましょう。このようなアプリの実例としては、Trelloが有名です。

かんばんは、表面的にはとてもシンプルで、ボードとカードがあります。ボードはプロジェクトを表しています。カードはタスクを表します。ボード上のカードの位置は、タスクの状態と進捗を表しています。最もシンプルなボードには、タスクのための3つの列があり、それらは、キュー（やるべきこと）、進行中（やっていること）、完了（やったこと）となっています。

表面上はシンプルですが、Kanbanやあらゆる種類のプロジェクト管理ソフトウェア全般は、文字通り底なしの複雑さです。実装できることは100万個もあり、最初の100万個を終えたら、さらに100万個のものがあるだろう。しかし、私は1つの記事を書こうとしているのであって、本シリーズ全体を書こうとしているわけではないので、機能の範囲は小さくしておきましょう。

サーバーは以下の機能をサポートする必要があります。

- ボードの作成
- ボードには名前がある
- すべてのボードのリストを取得する
- ボードの削除
- カードの作成
- ボードにカードを関連付けることができます。
- カードには説明とステータスがある
- ボード上のすべてのカードのリストを取得する
- ボードサマリーの取得：ボード上のすべてのカードの数をステータスごとにまとめたもの。
- カードの更新
- カードの削除

以上で完成です。このプロジェクトをもう少し面白くするために、サーバーのすべてのエンドポイントにトークンベースの認証を導入してみましょう。ただし、リクエストに有効なトークンが含まれている限り、すべてのボードとカードにアクセスできるというシンプルなものにしておきます。

さらに、私自身の好奇心を満たし、この記事の教育効果を最大化するために、2つの実装を一緒に書くことにします。1つはsync Rustを使用し、もう1つはasync Rustを使用します。1つ目の実装では、r2d2、Diesel、Rocketを使用します。2つ目の実装では、sqlxとactix-webを使用します。このプロジェクトで使用するクレートを簡単に紹介します。

- 一般的なクレート
    - dotenv (環境変数の読み込み)
    - log + fern (ロギング)
    - chrono (日付と時間の処理)
    - serde + serde_json (JSON デ/シリアル化)
- Sync クレート
    - diesel-cli (DBスキーマの移行)
    - diesel + diesel-derive-enum (ORM / SQLクエリの構築)
    - r2d2 (DBコネクションプール)
    - rocket + rocket_contrib (HTTPルーティング)
- 非同期クレート
    - sqlx-cli (DBスキーマのマイグレーション)
    - sqlx (SQLクエリの実行とDBコネクションプール)
    - actix-web (HTTPルーティング)
    - futures (一般的な未来関連のユーティリティ)

同期と非同期の両方の実装が終わったら、ベンチマークを実施してどちらのパフォーマンスが優れているかを確認します。

---

### General

#### Project Setup

Dockerのインストールやローカルでの実行など、このプロジェクトをセットアップするためのつまらない説明はすべて、[コンパニオンコードのリポジトリ](https://github.com/pretzelhammer/kanban)にあります。この記事では、楽しい部分に焦点を当ててみましょう

初期設定が終わると、空のCargo.tomlファイルができあがります。

```toml
[package]
name = "kanban"
version = "0.1.0"
edition = "2018"
```

そして、この空のmain.rs。

```rust
fn main() {
    println!("Hello, world!");
}
```
