use serenity::model::prelude::Message;
use std::fmt::Debug;
//use serenity::self;

pub enum Et<E, T> {
    Rslt(Result<Message, E>),
    Optn(Option<T>, Message),
    Other,
}

pub fn l<E, T>(cmd: &str, at: &str, enm: Et<E, T>) -> String 
where
    E: Debug,
    T: ToString,
{
    match enm {
        Et::Rslt(v) => {
            if let Err(why) = v {
                format!("kgrs!{} / {} : {:?}", cmd, at, why)
            } else {
                format!("kgrs!{} / {} : success", cmd, at)
            }
        },
        Et::Optn(v, msg) => {
            if let Some(val) = v {
                val.to_string()
            } else {
                msg.author.name
            }
        },
        Et::Other => format!("kgrs!{} / {}", cmd, at),
    }
}

pub fn emsg<T>(cmd: &str, at_er: &str) -> String {
    format!("kgrs!{} / {}", cmd, at_er)
}

pub fn err_detect<T, E: std::fmt::Debug>(msg: Result<T, E>) {
    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }
}