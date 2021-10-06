use std::env;
use std::sync::atomic::AtomicBool;

use serenity::framework::StandardFramework;
use serenity::prelude::*;

use commands::nakachan::*;

mod commands;
mod handler;

#[tokio::main]
async fn main() {
    // Discordのbot Tokenを設定
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // コマンド系の設定
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&commands::help::MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(handler::Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
