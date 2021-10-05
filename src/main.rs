use std::{
    collections::HashSet,
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use chrono::offset::Utc;
use serenity::{
    async_trait,
    framework::{
        standard::{
            help_commands,
            macros::{group, help},
            Args, CommandGroup, CommandResult, HelpOptions,
        },
        StandardFramework,
    },
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        // id::{ChannelId, GuildId, UserId},
        id::{GuildId, UserId},
    },
    prelude::*,
};

mod commands;

#[help]
#[individual_command_tip = "This is a help command of naka-chan"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

use commands::nakachan::*;
#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(nakachan)]
struct General;

struct Handler {
    is_loop_running: AtomicBool,
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

// async fn log_system_load(ctx: Arc<Context>) {
//     let cpu_load = sys_info::loadavg().unwrap();
//     let mem_use = sys_info::mem_info().unwrap();

//     // 特定のチャンネルにメッセージを送信
//     if let Err(why) = ChannelId(381926291785383946)
//         .send_message(&ctx, |m| {
//             m.embed(|e| {
//                 e.title("System Resource Load");
//                 e.field(
//                     "CPU Load Average",
//                     format!("{:.2}%", cpu_load.one * 10.0),
//                     false,
//                 );
//                 e.field(
//                     "Memory Usage",
//                     format!(
//                         "{:.2} MB Free out of {:.2} MB",
//                         mem_use.free as f32 / 1000.0,
//                         mem_use.total as f32 / 1000.0
//                     ),
//                     false,
//                 );
//                 e
//             })
//         })
//         .await
//     {
//         eprintln!("Error sending message: {:?}", why);
//     };
// }

// botのステータスを更新
async fn set_status_to_current_time(ctx: Arc<Context>) {
    let current_time = Utc::now();
    let formatted_time = current_time.to_rfc2822();

    ctx.set_activity(Activity::playing(&formatted_time)).await;
}

#[tokio::main]
async fn main() {
    // Discordのbot Tokenを設定
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // コマンド系の設定
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
