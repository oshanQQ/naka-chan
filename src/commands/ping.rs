use serenity::framework::standard as framework;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[framework::macros::command]
#[description = "the ping command"]
async fn ping(ctx: &Context, msg: &Message) -> framework::CommandResult {
    log::info!("ping command requested.");
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}
