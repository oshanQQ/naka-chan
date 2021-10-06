use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};

pub struct Handler;

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
}
