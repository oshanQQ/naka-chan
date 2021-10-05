use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "那珂ちゃんがしゃべりまーす！"]
async fn nakachan(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "那珂ちゃんだよー")
        .await?;
    // CommandResultはResultを継承している
    // `Result?` は正常な値の場合、Resultの中身を返し、エラーの場合は即座にreturnする演算子
    Ok(())
}