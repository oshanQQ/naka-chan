use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::messages::find_message;

pub struct Help<'a> {
    ctx: &'a Context,
    msg: &'a Message,
}

#[async_trait::async_trait]
impl<'a> super::Command<'a> for Help<'a> {
    crate::accessor_impl!((get = get_ctx) ctx: &Context);
    crate::accessor_impl!((get = get_msg) msg: &Message);
    fn new(ctx: &'a Context, msg: &'a Message) -> Help<'a> {
        Help { ctx, msg }
    }

    async fn execute(self, _argv: &[&str]) -> anyhow::Result<Message> {
        log::info!("help command requested.");
        self.log_message_detail().await;

        self.send(&find_message("help")?).await
    }
    fn descript() -> &'static str {
        "help: This is a help command of naka-chan"
    }
}
