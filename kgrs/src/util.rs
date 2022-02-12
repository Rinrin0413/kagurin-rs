use serenity::{
    self,
    model::channel::Message,
};

pub fn er(err: &str, why: serenity::Error) {
    println!("Error {}: {:?}", err, why);
}

pub fn emsg(cmd: &str, at: &str) -> String {
    format!("kgrs!{} / {}", cmd, at)
}

pub fn err_detect(msg: Result<Message, serenity::Error>) {
    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }
}