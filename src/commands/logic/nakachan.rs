use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct Nakachan<'a> {
    ctx: &'a Context,
    msg: &'a Message,
}

#[async_trait::async_trait]
impl<'a> super::Command<'a> for Nakachan<'a> {
    crate::accessor_impl!((get = get_ctx) ctx: &Context);
    crate::accessor_impl!((get = get_msg) msg: &Message);
    fn new(ctx: &'a Context, msg: &'a Message) -> Nakachan<'a> {
        Nakachan { ctx, msg }
    }

    async fn execute(self, _argv: &[&str]) -> anyhow::Result<Message> {
        log::info!("nakachan: nakachan command requested.");
        self.log_message_detail().await;
        self.send("那珂ちゃんだよー:sparkles:").await
    }
    fn descript() -> &'static str {
        "nakachan: 那珂ちゃんがしゃべりまーす！"
    }
    fn help(&self) -> String {
        Self::descript().to_owned()
    }
}
