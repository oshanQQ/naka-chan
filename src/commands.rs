pub mod groups;
pub mod help;
pub mod nakachan;
pub mod ping;

use serenity::http::client::Http;
use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};

pub struct Handler;

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

        let handling = match msg.content.as_str() {
            "ping" => {
                log::info!("the ping message received.");
                log_message_detail(ctx.clone(), &msg).await;
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

async fn log_message_detail(http: impl AsRef<Http> + CacheHttp, msg: &Message) {
    use serenity::model::channel::Channel::*;

    let channel = msg.channel_id.to_channel(http).await;
    if let Err(err) = channel {
        log::error!("the channel name cannot be got because {:?}", err);
        return;
    }

    let channel_name = match channel.unwrap() {
        Guild(guild) => guild.name,
        Private(_) => "the private channel".to_owned(),
        Category(category) => category.name, // unreachable but not need to panic
        _ => {
            log::error!("the channel is neither text channel or private channel");
            return;
        }
    };

    log::info!(
        "[[Message detail Info]] channel:'{}', author:'{}', msg:'{}'",
        channel_name,
        msg.author.name,
        msg.content
    );
}
