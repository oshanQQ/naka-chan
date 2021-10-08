use serenity::framework::standard as framework;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[framework::macros::command]
#[description = "the ping command"]
async fn ping(ctx: &Context, msg: &Message) -> framework::CommandResult {
    log::info!("ping command requested.");
    super::log_message_detail(&ctx.http, msg).await;
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}
