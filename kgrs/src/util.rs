use serenity::model::prelude::Message;
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