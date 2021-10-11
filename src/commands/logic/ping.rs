use serenity::model::prelude::Message;
use serenity::prelude::*;

pub struct Ping<'a> {
    ctx: &'a Context,
    msg: &'a Message,
}

#[async_trait::async_trait]
impl<'a> super::Command<'a> for Ping<'a> {
    crate::accessor_impl!((get = get_ctx) ctx: &Context);
    crate::accessor_impl!((get = get_msg) msg: &Message);

    fn new(ctx: &'a Context, msg: &'a Message) -> Ping<'a> {
        Ping { ctx, msg }
    }

    async fn execute(self, _argv: &[&str]) -> anyhow::Result<Message> {
        log::info!("ping command requested.");
        self.log_message_detail().await;
        self.send("Pong!").await
    }

    fn descript() -> &'static str {
        "ping: the ping command"
    }
}
