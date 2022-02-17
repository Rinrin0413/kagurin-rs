use crate::util::fmt::Et;
use serenity::{
    model::{
        prelude::{UserId, User, Message}, 
        guild::Member,
    },
    prelude::Context,
};
use rand::Rng;
use uuid::Uuid;

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

pub fn rand_choise<T>(list: &[T]) -> &T {
    let max = list.len() - 1;
    &list[rand::thread_rng().gen_range(0..=max)]
}

pub async fn upper_lower_uuid(arg_ii: Option<&str>, msg: &Message, ctx: &Context) -> Option<String> {
    let uc_uuid = Some(Uuid::new_v4().to_string().to_ascii_uppercase());
    let lc_uuid = Some(Uuid::new_v4().to_string().to_string().to_ascii_lowercase());
    if let Some(b) = arg_ii {
        match b {
            "true"|"True"|"t"|"T" => uc_uuid,
            "false"|"False"|"f"|"F" => lc_uuid,
            _ => {
                let content = msg
                    .channel_id
                    .say(&ctx.http, "無効な引数を確認\n第2引数には真偽値を入れてください")
                    .await;
                Et::Rslt(content).l("uuid", "SEND");
                return None
            }
        }
    } else {
        lc_uuid
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