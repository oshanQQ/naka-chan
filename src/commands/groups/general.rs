use super::super::nakachan::*;
use serenity::framework::standard::macros::group;

#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(nakachan)]
pub struct General;
