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

そして、空のmain.rs。

```rust
fn main() {
    println!("Hello, world!");
}
```

#### Loading Environment Variables w/dotenv

crates

- dotenv

```toml
[package]
name = "kanban"
version = "0.1.0"
edition = "2018"

+[dependencies]
+dotenv = "0.15"
```

このクレートの役割は、カレントワーキングディレクトリにある`.env`から変数をロードして、プログラムの環境変数に追加するという、小さなシンプルなものです。ここでは、一般的な`.env`ファイルを使用します。

```
LOG_LEVEL=INFO
LOG_FILE=server.log
DATABASE_URL=postgres://postgres@172.24.244.57:5432/postgres 
```

`dotenv`を使用したmain.rsの更新

```rust
type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;

    // example
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}
```

#### Handling Dates & Times w/chrono

crates

- chrono

```toml
[package]
name = "kanban"
version = "0.1.0"
edition = "2018"

[dependencies]
dotenv = "0.15"
+chrono = "0.4"
```

Rustの日付や時間を扱うためのライブラリといえば`chrono`です。まだプロジェクトでは使用していませんが、いくつかの依存関係を追加した後、すぐに使用する予定です。

#### Logging w/fern

crates

- log
- fern

```toml
[package]
name = "kanban"
version = "0.1.0"
edition = "2018"

[dependencies]
dotenv = "0.15"
chrono = "0.4"
+log = "0.4"
+fern = "0.6"
```

LogはRustのロギング・ファサード・ライブラリです。ハイレベルなロギングAPIを提供していますが、実装を選ぶ必要があり、今回使用する実装はfernクレートです。`fern`を使うことで、ロギングのフォーマットを簡単にカスタマイズでき、また複数の出力を連鎖させることができるので、標準エラーとファイルにログを記録することができます。`log`と`fern`を追加した後、すべてのロギング設定と初期化を独自のモジュールにカプセル化してみましょう。

```rust
use std::env;
use std::fs;
use log::{debug, error, info, trace, warn};

pub fn init() -> Result<(), fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

        let log_file = env::var("LOG_FILE").ok();
            if let Some(log_file) = log_file {
            let log_file = fs::File::create(log_file)?;
            builder = builder.chain(log_file);
        }

        builder.apply()?;

        trace!("TRACE output enabled");
        debug!("DEBUG output enabled");
        info!("INFO output enabled");
        warn!("WARN output enabled");
        error!("ERROR output enabled");

    Ok(())
}
```

そして、そのモジュールをmain.rsに追加します。

INFOがデフォルトのログレベルなので、今このプログラムを実行すると、以下のようになります。

```
[00:06:13][disel_rocket::logger][INFO] INFO output enabled
[00:06:13][disel_rocket::logger][WARN] WARN output enabled
[00:06:13][disel_rocket::logger][ERROR] ERROR output enabled
```

#### JSON Serialization w/serde

crates

- serde
- serde_json

```toml
[dependencies]
dotenv = "0.15"
- chrono = "0.4"
+ chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
fern = "0.6"
+ serde = { version = "1.0", features = ["derive"] }
+ serde_json = "1.0"
```

Pro-tip：プロジェクトに新しい依存関係を追加するときは、既存の依存関係を調べて、その新しい依存関係が機能フラグとして設定されているかどうかを確認するとよいでしょう。この場合、`chrono`は`serde`を機能フラグとして持っており、これを有効にすると、`chrono`のすべての型に`serde::Serialize`と`serde::Deserialize`インプリメンテーションが追加されます。これにより、後々、`chrono`の型を独自の構造体で使用できるようになり、その構造体にも`serde::Serialize`と`serde::Deserialize`のインプリーションを導出することになります。

#### Domain Modeling

さて、ドメインのモデリングを始めましょう。ボードがあることはわかっているので

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

新しいものをアンパックします。

- `#[derive(serde::Serialize)]`Board用の`serde::Serialize impl`を派生させ、`serde_json crate`を使ってJSONにシリアライズできるようにします。
- `#[serde(rename_all = "camelCase")]`シリアル化の際に`snake_case`のメンバー識別子をすべてキャメルケースにリネームします（デシリアライズの際にはその逆も可能）。これは、Rustではスネークケースの名前を使うことが慣習となっていますが、JSONはしばしばJSコードによって生成・消費され、JSの慣習ではメンバー識別子にキャメルケースを使うことになっているためです。
- `id`を`u64`ではなく`i64`にしたのは奇妙な選択に思えるかもしれませんが、DBとしてPostgreSQLを使用しているので、PostgreSQLは符号付き整数型しかサポートしていないため、このようにしなければなりません。
- `created_at`メンバは、他に良いソート順がない場合にエンティティを時系列でソートすることができるという理由以外では、常に持っていると便利です。

さて、カードとステータスを追加してみましょう。

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Todo,
    Doing,
    Done,
}
```

また、ボード上のすべてのカードの数をステータスごとにまとめた「ボードサマリー」を返すこともサポートしたいので、そのためのモデルを用意しました。

```rust
#[derive(serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}
```

APIを使用して新しいボードを作成する際、ユーザーはボード名を提供することができますが、そのIDはDBによって設定されるため、そのためのモデルも必要です。

```rust
#[derive(serde::Deserialize)]
pub struct CreateBoard {
    pub name: String,
}
```

同様に、ユーザーはカードを作成することもできます。カードを作成する際に、ユーザーは新しいカードの説明と、どのボードに関連付けるかだけを提供したいと仮定します。新しいカードは、デフォルトのTodoステータスを取得し、そのIDはDBによって設定されます。

```rust
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}
```

カードを更新する際、ユーザーは説明やステータスのみを更新できるようにしたいとします。もしユーザーがボード間でカードを移動できるようにしたら、かなり奇妙なことになります。これはほとんどのプロジェクト管理アプリでは珍しい機能です。

```rust
#[derive(serde::Deserialize)]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}
```

それらをそれぞれのモジュールに投入すると、次のようになります。

```rust
// for GET requests

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}

// for POST requests

#[derive(serde::Deserialize)]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

// for PATCH requests

#[derive(serde::Deserialize)]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}
```

そしてmain.rsを以下のように更新します。

```rust
mod logger;
+mod models;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::init()?;

    Ok(())
}
```

---

### Sync Implementation

#### SQL Schema Migrations w/diesel-cli

crates

- diesel-cli

```
cargo install diesel_cli
```

上記のコマンドが最初に動作しない場合は、diesel-cliがサポートしているすべてのデータベース用の開発ライブラリが揃っていないことが原因です。ここではPostgreSQLを使用しているので、これらのコマンドで開発ライブラリがインストールされていることを確認できます。

```
# macOS
brew install postgresql

# ubuntu
apt-get install postgresql libpq-dev
```

そして、PostgreSQLをサポートするdiesel-cliのみをインストールするようにcargoに伝えることができます。

```
cargo install diesel_cli --no-default-features --features postgres
```

diesel-cliは、DATABASE_URL環境変数をチェックすることで、どのDBに接続すべきかを判断します。

DBが現在稼働しており、DATABASE_URL環境変数が存在すると仮定して、プロジェクトをブートストラップするために実行する最初のdiesel-cliコマンドは以下の通りです。

```
diesel setup
```

これにより diesel-cliは`migrations`ディレクトリを作成し、そこでDBスキーマのマイグレーションを生成したり書いたりすることができます。最初のマイグレーションを生成してみましょう。


```
diesel migration generate create_boards
```

これにより、例えば`migrations/<year>-<month>-<day>-<time>_create_boards`という新しいディレクトリが作成され、`up.sql`と`down.sql`が作成され、ここに SQL クエリを記述します。upクエリは、DBスキーマを作成または変更するためのもので、ここではboardsテーブルを作成します。

```sql
-- create_boards up.sql
CREATE TABLE IF NOT EXISTS boards (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

-- seed db with some test data for local dev
INSERT INTO boards
(name)
VALUES
('Test board 1'),
('Test board 2'),
('Test board 3');
```

そして、downクエリは、upクエリで行ったスキーマの変更を元に戻すためのもので、ここでは作成したボードテーブルを削除しています。

```sql
-- create_boards down.sql
DROP TABLE IF EXISTS boards;
```

また、いくつかのカードを保管する必要があります。

```
diesel migration generate create_cards
```

カードのアップクエリについて

```sql
-- create_cards up.sql
CREATE TYPE STATUS_ENUM AS ENUM ('todo', 'doing', 'done');

CREATE TABLE IF NOT EXISTS cards (
    id BIGSERIAL PRIMARY KEY,
    board_id BIGINT NOT NULL,
    description TEXT NOT NULL,
    status STATUS_ENUM NOT NULL DEFAULT 'todo',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc'),
    CONSTRAINT board_fk
        FOREIGN KEY (board_id)
        REFERENCES boards(id)
        ON DELETE CASCADE
);

-- seed db with some test data for local dev
INSERT INTO cards
(board_id, description, status)
VALUES
(1, 'Test card 1', 'todo'),
(1, 'Test card 2', 'doing'),
(1, 'Test card 3', 'done'),
(2, 'Test card 4', 'todo'),
(2, 'Test card 5', 'todo'),
(3, 'Test card 6', 'done'),
(3, 'Test card 7', 'done');
```

そして、カードのダウンクエリ。

```sql
-- create_cards down.sql
DROP TABLE IF EXISTS cards;
```









