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
        use std::fs::File;
        use std::io::Read;
        use std::path::Path;

        log::info!("help command requested.");
        self.log_message_detail().await;

        let path = Path::new("resources/massages.txt");
        let mut file = File::open(&path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        self.send(content.to_string().as_str()).await
    }
    fn descript() -> &'static str {
        "help: This is a help command of naka-chan"
    }
}
