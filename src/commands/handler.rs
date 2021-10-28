use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};

use super::logic::*;
use super::Handler;

#[async_trait]
impl EventHandler for Handler {
    // botが特定のメッセージを受け取った時の処理
    async fn message(&self, ctx: Context, msg: Message) {
        #[cfg(debug_assertions)]
        {
            if msg.content == "quit" {
                log::info!("quit command received: the user was {:?}", msg.author.name);
                log::warn!("naka-chan will be stopped...");
                std::process::exit(0);
            }
        }

        let argv = msg.content.split_whitespace().collect::<Vec<&str>>();
        if !msg.content.starts_with('!') {
            return;
        }

        let result = match argv[0].trim_start_matches('!') {
            "ping" => Ping::new(&ctx, &msg).execute(&argv[..]),
            "help" => Help::new(&ctx, &msg).execute(&argv[..]),
            "nakachan" => Nakachan::new(&ctx, &msg).execute(&argv[..]),
            _ => {
                log::warn!("unknown command '{}'requested.", msg.content);
                todo!("存在しないコマンドです");
                // return;
            }
        };

        if let Err(why) = result.await {
            log::error!("Error {:?} of sending message: {:?}", why, msg.content);
        }
    }

    // botがonlineになった時の処理
    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);
    }
}
