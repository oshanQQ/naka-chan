use serenity::framework::standard::macros::*;
use serenity::framework::standard as framework;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
/// The ping command
/// **Usage**
///   - ping : return message 'Pong!'
///   - ping -v : print ping description
async fn ping(ctx: &Context, msg: &Message) -> framework::CommandResult {
    log::info!("ping command requested.");
    super::log_message_detail(&ctx.http, msg).await;
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}


