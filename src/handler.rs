use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use chrono::offset::Utc;

use serenity::model::gateway::{Activity, Ready};
use serenity::model::{channel::Message, id::GuildId};
use serenity::{async_trait, prelude::*};

pub struct Handler {
    pub is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    // botがメッセージを受け取った時の処理
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // botがonlineになった時の処理
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // cacheに関する処理
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");
        let ctx = Arc::new(ctx);

        // ループがすでに実行されていないかどうかを確認
        if !self.is_loop_running.load(Ordering::Relaxed) {
            println!("Let's check the status of the loop");
            // let ctx1 = Arc::clone(&ctx);
            // 並行して実行できる新しいスレッドを作成
            tokio::spawn(async move {
                println!("Spawn first thread!");
                loop {
                    // log_system_load(Arc::clone(&ctx1)).await;
                    tokio::time::sleep(Duration::from_secs(120)).await;
                    println!("Running the loop...");
                }
            });

            // 複数のスレッドを異なるタイミングで実行
            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                println!("Spawn second thread!");
                loop {
                    set_status_to_current_time(Arc::clone(&ctx2)).await;
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });

            // ループが実行されているので、boolをtrueに設定。
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

// botのステータスを更新
async fn set_status_to_current_time(ctx: Arc<Context>) {
    let current_time = Utc::now();
    let formatted_time = current_time.to_rfc2822();

    ctx.set_activity(Activity::playing(&formatted_time)).await;
}
