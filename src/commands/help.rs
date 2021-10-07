use std::collections::HashSet;

use serenity::framework::standard as framework;
use serenity::model::{channel::Message, id::UserId};
use serenity::prelude::Context;

#[framework::macros::help]
#[individual_command_tip = "This is a help command of naka-chan"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: framework::Args,
    help_options: &'static framework::HelpOptions,
    groups: &[&'static framework::CommandGroup],
    owners: HashSet<UserId>,
) -> framework::CommandResult {
    log::info!("help command requested.");
    let _ =
        framework::help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
