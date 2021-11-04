use std::env;

use serenity::framework::StandardFramework;
use serenity::prelude::*;

mod commands;
mod utils;

#[tokio::main]
async fn main() {
    // ロガーを初期化
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "naka_chan")
        .write_style_or("LOG_STYLE", "always");
    env_logger::Builder::from_env(env)
        .target(env_logger::Target::Stdout)
        .init();

    // Discordのbot Tokenを設定
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // コマンド系の設定
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&commands::help::MY_HELP)
        .group(&commands::groups::general::GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .event_handler(commands::Handler {})
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
