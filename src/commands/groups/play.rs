use crate::commands::*;
use serenity::framework::standard::macros::group;

#[group]
#[description("再生コマンド")]
#[summary("動画音声再生")]
//#[commands(play)]
pub struct Play;

