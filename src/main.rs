use std::env;

use serenity::client::ClientBuilder;
use serenity::framework::StandardFramework;
use serenity::http::HttpBuilder;

mod commands;
mod handler;

#[tokio::main]
async fn main() {
    // Discordのbot Tokenを設定
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let port = env::var("PORT").expect("Expected a port in the environment");

    // コマンド系の設定
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&commands::help::MY_HELP)
        .group(&commands::groups::general::GENERAL_GROUP);

    let http = HttpBuilder::new(token)
        .proxy(format!("http://0.0.0.0:{}", port))
        .expect("Invalid proxy URL")
        .await
        .expect("Error creating Http");
    let mut client = ClientBuilder::new_with_http(http)
        .event_handler(handler::Handler {})
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
