use crate::commands::*;
use nakachan::*;
use ping::*;
use serenity::framework::standard::macros::group;

#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(nakachan, ping)]
pub struct General;
