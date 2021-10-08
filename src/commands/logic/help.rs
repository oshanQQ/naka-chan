use serenity::model::channel::Message;
use serenity::prelude::Context;

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
        self.send(self.help().as_str()).await
    }
    fn descript() -> &'static str {
        "help: This is a help command of naka-chan"
    }
    fn help(&self) -> String {
        use std::io::Write;

        let mut s = vec![];
        writeln!(s, "{}", Help::descript()).unwrap();
        writeln!(s, "{}", super::Ping::descript()).unwrap();
        writeln!(s, "{}", super::Nakachan::descript()).unwrap();

        format!("```{}```", String::from_utf8(s).unwrap())
    }
}
