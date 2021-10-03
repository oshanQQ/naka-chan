# serenity は何ができるか

example を片っ端から調べていくよ。

## example 1

```rs
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // メッセージイベントのハンドラを設定し、
    // 新しいメッセージを受信するたびに、渡されたクロージャ（または関数）が呼び出されるようする。
    //
    // イベントハンドラーはスレッドプールを介してディスパッチされるため、
    // 複数のイベントを同時にディスパッチすることができる。
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
          // メッセージの送信は、ネットワークエラー、認証エラー、チャンネルに投稿する権限がないなどの理由で失敗することがある。
          // 何らかのエラーが発生した場合には、その内容をstdoutに記録する。
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // readyイベントで呼び出されるハンドラを設定する。
    // これは、Shardが起動して、DiscordからREADYペイロードが送信されたときに呼び出される。
    // このペイロードには、現在のユーザーのサーバーId、現在のユーザーデータ、プライベートチャンネルなどのデータが含まれています。
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // botとしてログイン！
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    // 1つのシャードを起動し、イベントの受信を始める。
    // シャードは自動的に再接続しようとする。再接続されるまで指数関数的なバックオフを行います。
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
```

## example 2 Shard で複数サーバーと通信

- 複数のサーバーに入っている bot を通信させるときに Shard を使う。
- `naka-chan`では使わなさそう。

## example 3 便利なメソッド

```rs
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!messageme" {
            // utils機能が有効になっている場合、
            // モデル構造体には多くの便利なメソッドが実装されており、
            // かさばるContextや、より低レベルのrestメソッドを使わずに済むようになっています。

            // この場合、Userのインスタンスのメソッドを呼び出すだけで、
            // メッセージの内容をUserに直接伝えることができます。
            let dm = msg
                .author
                .dm(&context, |m| {
                    m.content("Hello!");

                    m
                })
                .await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {:?}", why);
            }
        }
    }
}

```

- モデル構造体には便利なメソッドが使えるものもある。
- この例だと、`msg.author.dm(...)`みたいにメソッドが使える。

## example 4 メッセージの作成

```rs
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                },
            };

            // メッセージビルダーでは、
            // ユーザーを動的に言及したり、
            // コンテンツの「安全な」バージョンをプッシュしたり（正規化されたコンテンツを太字にするなど）、
            // 絵文字を表示するなどして、
            // メッセージを作成することができます。
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the 'ping' command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

```

## example 6 bot 構成の例

````rs
// コマンドを扱う場合、
// Cargo.tomlにframeworkが必要
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
mod commands;

use std::{collections::HashSet, env, sync::Arc};

use commands::{math::*, meta::*, owner::*};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// イベントを受け取るやつ
struct Handler;

// メッセージを受け取った時の処理
#[async_trait]
impl EventHandler for Handler {
    // 起動して待機
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

// Generalグループのコマンドを定義
#[group]
#[commands(multiply, ping, quit)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    // 環境変数を使用するためにロガーを初期化します。
    // この場合、環境変数 RUST_LOG をデバッグに設定するのが良いデフォルトです。
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // botの所有者(owner)とidを取得
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // General グループのコマンドを追加
    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("~")).group(&GENERAL_GROUP);

    // botのクライアントを作成
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    // ctrl + c で停止
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
````
