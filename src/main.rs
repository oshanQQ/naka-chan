use std::env;

use serenity::framework::StandardFramework;
use serenity::prelude::*;

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
        .group(&commands::groups::general::GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .event_handler(handler::Handler {})
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
