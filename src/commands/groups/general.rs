use super::super::utils::*;
use serenity::framework::standard::macros::*;
use serenity::framework::standard::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;
#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(nakachan, ping, join, leave)]
pub struct General;

#[command]
/// The ping command
/// **Usage**
///   - ping : return message 'Pong!'
///   - ping -v : print ping description
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    log::info!("ping command requested.");
    log_message_detail(&ctx.http, msg).await;
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

#[command]
/// The nakachan command
/// **Usage**
///  - nakachan: 那珂ちゃんがしゃべりまーす！
async fn nakachan(ctx: &Context, msg: &Message) -> CommandResult {
    log::info!("nakachan command requested.");
    log_message_detail(&ctx.http, msg).await;
    msg.channel_id
        .say(&ctx.http, "那珂ちゃんだよー:sparkles:")
        .await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
/// The join command
/// **Usage**
///   - join : join voice channel you are in
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    log::info!("join command requested.");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|state| state.channel_id);

    let channel_id = match channel_id {
        Some(channel) => channel,
        None => {
            if let Err(why) = msg.reply(ctx, "Not in a voice channel").await {
                log::error!("Error sending message: {}", why);
            }
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _ = manager.join(guild_id, channel_id).await;
    Ok(())
}

#[command]
#[only_in(guilds)]
/// The leave command
/// **Usage**
///   - leave : leave voice channel.
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    log::info!("leave command requested.");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }
        check_msg(msg.channel_id.say(&ctx.http, "お仕事しゅーりょー!").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[help]
#[individual_command_tip = "This is a help command of naka-chan"]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[wrong_channel = "Strike"]
#[strikethrough_commands_tip_in_guild = ""]
/// The Help Command
/// **Usage**
///  - help: This is a help command of naka-chan
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    log::info!("help command requested.");
    log_message_detail(&ctx.http, msg).await;
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
