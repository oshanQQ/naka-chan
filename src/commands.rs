pub mod groups;
pub mod help;
pub mod nakachan;
pub mod ping;
mod utils;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // botが特定のメッセージを受け取った時の処理
    #[cfg(debug_assertions)]
    async fn message(&self, ctx: Context, msg: Message) {
        {
            if msg.content == "quit" {
                log::info!("quit command received: the user was {:?}", msg.author.name);
                log::warn!("naka-chan will be stopped...");
                std::process::exit(0);
            }
        }

        let handling = match msg.content.as_str() {
            "ping" => {
                log::info!("the ping message received.");
                utils::log_message_detail(ctx.clone(), &msg).await;
                msg.channel_id.say(&ctx.http, "Pong!")
            }
            _ => return,
        };

        if let Err(why) = handling.await {
            log::error!("Error {:?} of sending message: {:?}", why, msg.content);
        }
    }

    // botがonlineになった時の処理
    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);
    }
}
