use std::{env, collections::HashSet};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
    framework::{StandardFramework, standard::{Args, CommandGroup, CommandResult, HelpOptions, help_commands, macros::{group, help}}},
};

mod commands;

#[help]
#[individual_command_tip = "This is a help command of naka-chan"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

use commands::{nakachan::*};
#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(nakachan)]
struct General;

struct Handler;
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

#[tokio::main]
async fn main() {
    // Discordのbot Tokenを設定
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // コマンド系の設定
    let framework = StandardFramework::new().configure(|c| c.prefix("!")).help(&MY_HELP).group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
