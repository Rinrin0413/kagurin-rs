use crate::util::fmt::Et;
use serenity::{
    model::{
        prelude::{UserId, User, Message}, 
        guild::Member,
    },
    prelude::Context,
};
pub fn restrict_users(author: UserId, auth_user: &[u64]) -> bool {
    auth_user.contains(&author.as_u64())
}

pub async fn get_user_from_arg(arg: &str, msg: &Message, ctx: &Context) -> Option<User> {
    match UserId(
        match arg.parse() {
            Ok(id) => id,
            Err(_) => {
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        &format!(
                            "無効な引数`{}`を確認\n引数には数値を入れてください",
                            arg
                        ),
                    )
                    .await
                    .unwrap();
                return None
            }
        }
    ).to_user(&ctx.http).await {
        Ok(u) => Some(u),
        Err(_) => {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("無効なユーザIDです: {}", arg))
                .await
                .unwrap();
            return None
        }
    }
}

pub async fn get_member_from_user(user: &User, msg: &Message, ctx: &Context) -> Option<Member> {
    match msg
        .guild_id
        .expect(&Et::Other("").l("_", "GET GUILD ID"))
        .member(&ctx.http, user.id)
        .await {
            Ok(m) => Some(m),
            Err(_) => None,
        }
}

pub mod fmt {
    use serenity::model::prelude::Message;
    use std::fmt::Debug;
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

    pub fn cb(content: &str, lang: &str) -> String {
        format!("```{}\n{}\n```", lang, content)
    }
    
    pub fn invalid_arg(false_arg: &str, true_args: &str) -> String {
        format!("無効な引数`{}`を確認\n利用可能な引数: `{}`", false_arg, true_args)
    }

}