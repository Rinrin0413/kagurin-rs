use serenity::model::prelude::{Message, UserId};
use std::fmt::Debug;
//use serenity::self;

pub enum Et<'a, T> {
    Rslt(Result<Message, T>),
    Optn(Option<T>, &'a Message),
    Other(T),
}

impl<T> Et<'_, T> 
where 
    T: ToString + Debug,
{
    pub fn l(&self, cmd: &str, at: &str) -> String {
        match self {
            Et::Rslt(v) => {
                if let Err(why) = v {
                    format!("kgrs!{} / {} : {:?}", cmd, at, why)
                } else {
                    format!("kgrs!{} / {} : success", cmd, at)
                }
            },
            Et::Optn(v, ref msg) => {
                if let Some(val) = v {
                    val.to_string()
                } else {
                    msg.author.name.clone()
                }
            },
            Et::Other(_) => format!("kgrs!{} / {}", cmd, at),
        }
    }
}

pub fn want_arg(n: usize) -> String {
    format!("Error: 最低{}つの引数が必要です", n)
}

pub fn is_trusted(author: UserId, trusted: &[u64]) -> bool {
    trusted.contains(author.as_u64())
}

pub fn is_developer(author: UserId, developer: [&'static u64; 2]) -> bool {
    developer.contains(&author.as_u64())
}

pub fn cb(content: &str, lang: &str) -> String {
    format!("```{}\n{}\n```", lang, content)
}

pub fn invalid_arg(false_arg: &str, true_args: &str) -> String {
    format!("無効な引数`{}`を確認\n利用可能な引数: `{}`", false_arg, true_args)
}