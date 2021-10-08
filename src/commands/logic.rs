pub mod help;
pub mod nakachan;
pub mod ping;

use serenity::model::channel::Message;
use serenity::prelude::*;

pub use help::Help;
pub use nakachan::Nakachan;
pub use ping::Ping;

#[async_trait::async_trait]
pub trait Command<'a> {
    crate::accessor!((get = get_ctx): &Context);
    crate::accessor!((get = get_msg): &Message);

    fn new(ctx: &'a Context, msg: &'a Message) -> Self;
    async fn execute(self, argv: &[&str]) -> anyhow::Result<Message>;
    fn descript() -> &'static str;
    fn help(&self) -> String;

    // default implementation for Command
    fn info(&self) -> (&Context, &Message) {
        (self.get_ctx(), self.get_msg())
    }
    async fn send(&self, content: &str) -> anyhow::Result<Message> {
        let (ctx, msg) = self.info();
        msg.channel_id
            .say(ctx.http.clone(), content)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn log_message_detail(&self) {
        use serenity::model::channel::Channel::*;

        let (ctx, msg) = self.info();
        let channel = msg.channel_id.to_channel(ctx.http.clone()).await;
        if let Err(err) = channel {
            log::error!("the channel name cannot be got because {:?}", err);
            return;
        }

        let channel_name = match channel.unwrap() {
            Guild(guild) => guild.name,
            Private(_) => "the private channel".to_owned(),
            Category(category) => category.name, // unreachable but not need to panic
            _ => {
                log::error!("the channel is neither text channel nor private channel");
                return;
            }
        };

        log::info!(
            "[[Message detail Info]] channel:'{}', author:'{}', msg:'{}'",
            channel_name,
            msg.author.name,
            msg.content
        );
    }
}
