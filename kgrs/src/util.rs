pub fn emsg(cmd: &str, at: &str) -> String {
    format!("kgrs!{} / {}", cmd, at)
}

pub fn err_detect<T, E>(msg: Result<T, E>) 
where 
    T: serenity::model::channel::Message,
    E: serenity::Error,
{

}