use serenity::framework::standard as framework;
use serenity::model::prelude::*;
use serenity::prelude::*;
use super::utils;

#[framework::macros::command]
/// The nakachan command
/// **Usage**
///  - nakachan: 那珂ちゃんがしゃべりまーす！
async fn nakachan(ctx: &Context, msg: &Message) -> framework::CommandResult {
    log::info!("nakachan command requested.");
    utils::log_message_detail(&ctx.http, msg).await;
    msg.channel_id
        .say(&ctx.http, "那珂ちゃんだよー:sparkles:")
        .await?;
    Ok(())
}
