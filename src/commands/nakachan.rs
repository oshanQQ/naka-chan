use serenity::framework::standard as framework;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[framework::macros::command]
#[description = "那珂ちゃんがしゃべりまーす！"]
async fn nakachan(ctx: &Context, msg: &Message) -> framework::CommandResult {
    msg.channel_id
        .say(&ctx.http, "那珂ちゃんだよー:sparkles:")
        .await?;
    Ok(())
}
