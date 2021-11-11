use std::collections::HashSet;

use serenity::framework::standard as framework;
use serenity::framework::standard::macros::*;
use serenity::model::{channel::Message, id::UserId};
use serenity::prelude::Context;
use super::utils;

#[help]
#[individual_command_tip = "This is a help command of naka-chan"]
#[command_not_found_text="Could notr find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[wrong_channel = "Strike"]
#[strikethrough_commands_tip_in_guild=""]
/// The Help Command
/// **Usage**
///  - help: This is a help command of naka-chan
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: framework::Args,
    help_options: &'static framework::HelpOptions,
    groups: &[&'static framework::CommandGroup],
    owners: HashSet<UserId>,
) -> framework::CommandResult {
    log::info!("help command requested.");
    utils::log_message_detail(&ctx.http, msg).await;
    let _ =
        framework::help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
