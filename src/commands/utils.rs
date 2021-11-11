
use serenity::http::client::Http;
use serenity::http::CacheHttp;
use serenity::model::channel::Message;

pub async fn log_message_detail(http: impl AsRef<Http> + CacheHttp, msg: &Message) {
    use serenity::model::channel::Channel::*;

    let channel = msg.channel_id.to_channel(http).await;
    if let Err(err) = channel {
        log::error!("the channel name cannot be got because {:?}", err);
        return;
    }

    let channel_name = match channel.unwrap() {
        Guild(guild) => guild.name,
        Private(_) => "the private channel".to_owned(),
        Category(category) => category.name, // unreachable but not need to panic
        _ => {
            log::error!("the channel is neither text channel or private channel");
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
