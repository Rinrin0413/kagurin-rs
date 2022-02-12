use serenity::model::prelude::Message;
use std::fmt::Debug;
//use serenity::self;

pub enum Et<'a, T, E, U> {
    Rslt(Result<T, E>),
    Optn(Option<U>, &'a Message),
    Other,
}

pub fn l<T, E, U>(cmd: &str, at: &str, enm: Et<T, E, U>) -> String 
where
    E: Debug,
    U: ToString,
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
            if let Ok(val) = v {
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