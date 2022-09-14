//! These are utilities for kagurin-rs

use std::{env, process};

/// Get the current memory usage by this bot. (MiB)
pub fn get_memory_usage() -> f64 {
    let stdout = process::Command::new("sh")
        .args([
            format!("{}/../assets/shells/pmap.sh", env!("CARGO_MANIFEST_DIR")),
            process::id().to_string(),
        ])
        .output()
        .unwrap()
        .stdout;
    String::from_utf8_lossy(&stdout)
        .to_string()
        .replace('\n', "")
        .parse::<f64>()
        .unwrap()
        / 1024.0
}

/// Rounds the passed `f64` halfway.
/// Leaves up to the position specified by `decimal_places`argument.
///
/// # Examples:
///
/// ```
/// assert_eq!(round_mid(3.1415, 3), 3.142);
/// ```
pub fn round_mid(num: f64, decimal_places: u32) -> f64 {
    let n = 10_u64.pow(decimal_places) as f64;
    (num * n).round() / n
}

/// Get the current uptime of this bot.
pub fn get_uptime() -> String {
    let stdout = process::Command::new("sh")
        .args([
            format!("{}/../assets/shells/ps.sh", env!("CARGO_MANIFEST_DIR")),
            process::id().to_string(),
        ])
        .output()
        .unwrap()
        .stdout;
    String::from_utf8_lossy(&stdout)
        .to_string()
        .replace('\n', "")
}

pub mod fmt {
    //! This is a module for formatting texts.

    /// Formats to codeblocks.
    ///
    /// We can also specify language.
    ///
    /// # Examples:
    ///
    /// ```
    /// assert_eq!(
    ///     cb("print(Hello, World!)", "py"),
    ///     "```py\nprint(Hello, World!)\n```"
    /// );
    /// ```
    pub fn cb(content: &str, lang: &str) -> String {
        format!("```{}\n{}\n```", lang, content)
    }
}

/*
use crate::util::fmt::*;
use rand::Rng;
use serenity::{
    model::{
        guild::Member,
        prelude::{Message, User, UserId},
    },
    prelude::Context,
};
use std::{collections::HashMap, fmt::Display};
use uuid::Uuid;

/// Restricts users based on the passed user list.
///
/// Returns `true` if the user_list contains the author. Otherwise returns `false`.
///
/// # Examples:
///
/// ```
/// let authed_users: [u64; 2] = [
///     724976600873041940, // User1
///     801082943371477022, // User2
/// ];
///
/// assert!(
///     restricting_users(&authed_users,
///     UserId(724976600873041940))
/// );
///
/// assert!(!restricting_users(
///     &authed_users,
///     UserId(936116497502318654)
/// ));
/// ```
pub fn restricting_users(user_list: &[u64], author: UserId) -> bool {
    user_list.contains(&author.as_u64())
}

/// Returns `Option<User>` based on `id`.
/// Otherwise returns `None`.
///
/// `id` must be a type of &str.
pub async fn get_user_from_id(id_str: &str, msg: &Message, ctx: &Context) -> Option<User> {
    let user_id = UserId(match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("無効な引数`{}`を確認\n引数には数値を入れてください", id_str),
                )
                .await
                .unwrap();
            return None;
        }
    })
    .to_user(&ctx.http)
    .await;
    match user_id {
        Ok(u) => Some(u),
        Err(_) => {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("無効なユーザIDです: {}", id_str))
                .await
                .unwrap();
            return None;
        }
    }
}

/// Returns `Option<Member>` based on `User`.
/// Otherwise returns "None"
pub async fn get_member_from_user(user: &User, msg: &Message, ctx: &Context) -> Option<Member> {
    if let Some(g) = msg.guild_id {
        match g.member(&ctx.http, user.id).await {
            Ok(m) => Some(m),
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Randomly choice from the passed list.
///
/// # Examples:
///
/// ```
/// let list = ["a", "b", "c", "d"];
///
/// assert!(list.contains(rand_choice(&list)));
///
/// assert!(!"z".contains(rand_choice(&list)));
/// ```
pub fn rand_choice<T>(list: &[T]) -> &T {
    let max = list.len() - 1;
    &list[rand::thread_rng().gen_range(0..=max)]
}

/// Returns a upper case or lower case UUID(Option<String>).
///
/// Returns a lower case UUID(Option<String>) if arg`is_upper_case` is `None`.
///
/// If `Option` content in arg`is_upper_case` is true or True or t or T,
/// returns a upper case UUID(Option<String>).
///
/// If `Option` content in arg`is_upper_case` is false or False or f or F,
/// returns a lower case UUID(Option<String>).
///
/// And if arg`is_upper_case` does not fit into any of these case,
/// sends warning message and returns `None`.
pub async fn get_upper_or_lower_uuid(
    is_upper_case: Option<&str>,
    msg: &Message,
    ctx: &Context,
) -> Option<String> {
    let uc_uuid = Some(Uuid::new_v4().to_string().to_ascii_uppercase());
    let lc_uuid = Some(Uuid::new_v4().to_string().to_string().to_ascii_lowercase());
    if let Some(b) = is_upper_case {
        match b {
            "true" | "True" | "t" | "T" => uc_uuid,
            "false" | "False" | "f" | "F" => lc_uuid,
            _ => {
                let content = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "無効な引数を確認\n第2引数には真偽値を入れてください",
                    )
                    .await;
                Et::Rslt(content).l("uuid", "SEND");
                return None;
            }
        }
    } else {
        lc_uuid
    }
}

/// Arguments Setup.
///
/// Allow handling of arguments as vectors.
///
/// Specifies the number of arguments with arg`num_of_arg`.
/// This value should be changed if the number of arguments that need to be processed increases
///
/// In addition, index 0 of the return value is determined to be the command name.
/// So it is processed without being counted as the number of arguments
///
/// # Examples:
///
/// ```
/// let msg = String::from("/command test 16 true");
/// assert_eq!(
///     setup_arg(6, &msg),
///     vec![
///         Some("/command"),
///         Some("test"),
///         Some("16"),
///         Some("true"),
///         None,
///         None,
///         None
///     ]
/// );
/// ```
pub fn setup_arg(num_of_arg: usize, msg_content: &str) -> Vec<Option<&str>> {
    let mut args = Vec::new();
    for i in 0..num_of_arg + 1 {
        args.push(msg_content.split(' ').nth(i));
    }
    args
}

/// Extract the contents of `Option<String>`.
///
/// If `Option<T>` is `None`, returns the value specified in arg`if_none` instead.
///
/// And arg`if_none` must implement `ToString` trait
///
/// Examples:
///
/// ```
/// let optn_stuff: Option<String> = Some(String::from("content"));
/// assert_eq!(
///     optn_unzip(optn_stuff, "unuse"),
///     String::from("content")
/// );
///
/// let optn_stuff_none: Option<_> = None;
/// assert_eq!(
///     optn_unzip(optn_stuff_none, "none"),
///     String::from("none")
/// );
/// ```
pub fn optn_unzip<T: ToString>(optn: Option<String>, if_none: T) -> String {
    if let Some(v) = optn {
        v
    } else {
        if_none.to_string()
    }
}

/// Return the length of a vector from `Result<Vec<T>, E>`.
/// Returned length of vector is type of String.
///
/// If the passed type is `Result::Err`, return a formatted failure message.
///
/// # Examples:
///
/// ```
/// let rslt_stuff: Result<Vec<f64>, &str> = Ok(vec![36.8, 36.6, 36.7, 36.3, 36.6]);
/// assert_eq!(get_len_of_vec(rslt_stuff), String::from("5"));
///
/// let rslt_stuff_err: Result<Vec<&str>, &str> = Err("FAILURE MESSAGE");
/// assert_eq!(
///     get_len_of_vec(rslt_stuff_err),
///     String::from("Failed to get: FAILURE MESSAGE")
/// );
/// ```
pub fn get_len_of_vec<T, E>(elm: Result<Vec<T>, E>) -> String
where
    E: Display,
{
    match elm {
        Ok(hm) => hm.len().to_string(),
        Err(why) => ftg(why),
    }
}

/// Return the length of a HashMap from `Result<HashMap<K, V>, E>`.
/// Returned length of HashMap is type of String.
///
/// If the passed type is `Result::Err`, return a formatted failure message.
///
/// # Examples:
///
/// ```
/// let mut hashmap: HashMap<&str, u32> = HashMap::new();
/// hashmap.insert("Jesus", 0001);
/// hashmap.insert("Shakyamuni", 0002);
/// hashmap.insert("Zeus", 0003);
/// hashmap.insert("Allah", 0004);
/// let rslt_stuff: Result<HashMap<&str, u32>, &str> = Ok(hashmap);
/// assert_eq!(get_len_of_hm(rslt_stuff), String::from("4"));
///
/// let rslt_stuff_err: Result<HashMap<&str, &str>, &str> = Err("FAILURE MESSAGE");
/// assert_eq!(
///     get_len_of_hm(rslt_stuff_err),
///     String::from("Failed to get: FAILURE MESSAGE")
/// );
/// ```
pub fn get_len_of_hm<E, K, V>(elm: Result<HashMap<K, V>, E>) -> String
where
    E: Display,
{
    match elm {
        Ok(hm) => hm.len().to_string(),
        Err(why) => ftg(why),
    }
}

/// Rounds the passed `f64` halfway.
/// Leaves up to the position specified by `decimal_places`argument.
///
/// # Examples:
///
/// ```
/// assert_eq!(round_mid(3.1415, 3), 3.142);
/// ```
pub fn round_mid(num: f64, decimal_places: u32) -> f64 {
    let n = 10_u64.pow(decimal_places) as f64;
    (num * n).round() / n
}

pub mod fmt {
    //! This is a module for formatting texts.

    use serenity::model::prelude::Message;
    use std::fmt::{Debug, Display};

    /// Enumeration for attacking `.await` :)
    pub enum Et<'a, T> {
        Rslt(Result<Message, T>),
        Optn(Option<T>, &'a Message),
        Other(T),
    }

    impl<T> Et<'_, T>
    where
        T: ToString + Debug,
    {
        /// Generates value from Et enum, and handles errors.
        /// Can also be used as a builtin to the expect function.
        ///
        /// In other words... The BEST function!
        ///
        /// And, this is totally for kagurin-rs:(
        ///
        /// # Examples:
        ///
        /// ```
        /// assert_eq!(
        ///     Et::Other("").l("info", "SEND MSG"),
        ///     String::from("kgrs!info / SEND MSG")
        /// );
        /// ```
        pub fn l(&self, cmd: &str, at: &str) -> String {
            match self {
                Et::Rslt(v) => {
                    if let Err(why) = v {
                        format!("kgrs!{} / {} : {:?}", cmd, at, why)
                    } else {
                        format!("kgrs!{} / {} : success", cmd, at)
                    }
                }
                Et::Optn(v, ref msg) => {
                    if let Some(val) = v {
                        val.to_string()
                    } else {
                        msg.author.name.clone()
                    }
                }
                Et::Other(_) => format!("kgrs!{} / {}", cmd, at),
            }
        }
    }

    /// Formats to `Error: 最低{}つの引数が必要です`
    ///
    /// When an argument is missing, use it.
    ///
    /// # Examples:
    ///
    /// ```
    /// assert_eq!(missing_arg(5), "Error: 最低5つの引数が必要です");
    /// ```
    pub fn missing_arg(n: usize) -> String {
        format!("Error: 最低{}つの引数が必要です", n)
    }

    /// Formats to codeblocks.
    ///
    /// We can also specify language.
    ///
    /// # Examples:
    ///
    /// ```
    /// assert_eq!(
    ///     cb("print(Hello, World!)", "py"),
    ///     "```py\nprint(Hello, World!)\n```"
    /// );
    /// ```
    pub fn cb(content: &str, lang: &str) -> String {
        format!("```{}\n{}\n```", lang, content)
    }

    /// Formats to `無効な引数\`{}\`を確認\n利用可能な引数: \`{}\``
    ///
    /// Available arguments can also be enumerated
    ///
    /// When an unexpected argument is passed, use it.
    ///
    /// # Examples:
    ///
    /// ```
    /// let arg = "pockey";
    /// let available_arg = "[chocorooms, chococones]";
    /// assert_eq!(
    ///     invalid_arg(arg, available_arg),
    ///     "無効な引数`pockey`を確認\n利用可能な引数: `[chocorooms, chococones]`"
    /// );
    /// ```
    pub fn invalid_arg<T: Display>(false_arg: &str, true_args: T) -> String {
        format!(
            "無効な引数`{}`を確認\n利用可能な引数: `{}`",
            false_arg, true_args
        )
    }

    /// Formats to `Failed to get: {}`
    ///
    /// When something failed to get, use it.
    ///
    /// # Examples:
    ///
    /// ```
    /// let error = "Message not found";
    /// assert_eq!(ftg(error), "Failed to get: Message not found");
    /// ```
    pub fn ftg<T: Display>(why: T) -> String {
        format!("Failed to get: {}", why)
    }

    /// Disable codeblocks
    /// Technically, replaces u+0060(triple) with u+1fef(triple).
    ///
    /// # Examples:
    ///
    /// ```
    /// let cb = cb("console.log(\"Hello, World!\")", "js");
    /// assert_eq!(
    ///     anti_cb(&cb),
    ///     "```js\nconsole.log(\"Hello, World!\")\n```"
    /// );
    /// ```
    pub fn anti_cb(arg: &str) -> String {
        arg.replace("```", "```")
    }

    /// Converts content to inline monospaced text.
    ///
    /// # Examples:
    ///
    /// ```
    /// let content = "wolwol";
    /// assert_eq!(mono(content), "`wolwol`");
    /// ```
    pub fn mono(content: &str) -> String {
        format!("`{}`", content)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::{fmt::*, *};

    #[test]
    fn user_restriction() {
        let authed_users: [u64; 2] = [
            724976600873041940, // User1
            801082943371477022, // User2
        ];

        assert!(restricting_users(&authed_users, UserId(724976600873041940)));

        assert!(!restricting_users(
            &authed_users,
            UserId(936116497502318654)
        ));
    }

    #[test]
    fn return_based_on_passed_list() {
        let list = ["a", "b", "c", "d"];

        assert!(list.contains(rand_choice(&list)));

        assert!(!"z".contains(rand_choice(&list)));
    }

    #[test]
    fn assign_args() {
        let msg = String::from("/command test 16 true");
        assert_eq!(
            setup_arg(6, &msg),
            vec![
                Some("/command"),
                Some("test"),
                Some("16"),
                Some("true"),
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn extract_contents_of_optn() {
        let optn_stuff: Option<String> = Some(String::from("content"));
        assert_eq!(optn_unzip(optn_stuff, "unuse"), String::from("content"));

        let optn_stuff_none: Option<_> = None;
        assert_eq!(optn_unzip(optn_stuff_none, "none"), String::from("none"));
    }

    #[test]
    fn get_len_of_vec_() {
        let rslt_stuff: Result<Vec<f64>, &str> = Ok(vec![36.8, 36.6, 36.7, 36.3, 36.6]);
        assert_eq!(get_len_of_vec(rslt_stuff), String::from("5"));

        let rslt_stuff_err: Result<Vec<&str>, &str> = Err("FAILURE MESSAGE");
        assert_eq!(
            get_len_of_vec(rslt_stuff_err),
            String::from("Failed to get: FAILURE MESSAGE")
        );
    }

    #[test]
    fn get_len_of_hm_() {
        let mut hashmap: HashMap<&str, u32> = HashMap::new();
        hashmap.insert("Jesus", 0001);
        hashmap.insert("Shakyamuni", 0002);
        hashmap.insert("Zeus", 0003);
        hashmap.insert("Allah", 0004);
        let rslt_stuff: Result<HashMap<&str, u32>, &str> = Ok(hashmap);
        assert_eq!(get_len_of_hm(rslt_stuff), String::from("4"));

        let rslt_stuff_err: Result<HashMap<&str, &str>, &str> = Err("FAILURE MESSAGE");
        assert_eq!(
            get_len_of_hm(rslt_stuff_err),
            String::from("Failed to get: FAILURE MESSAGE")
        );
    }

    #[test]
    fn tests_for_et() {
        assert_eq!(
            Et::Other("").l("info", "SEND MSG"),
            String::from("kgrs!info / SEND MSG")
        );
    }

    #[test]
    fn fmt_missing_arg() {
        assert_eq!(missing_arg(5), "Error: 最低5つの引数が必要です");
    }

    #[test]
    fn fmt_cb() {
        assert_eq!(
            cb("print(Hello, World!)", "py"),
            "```py\nprint(Hello, World!)\n```"
        );
    }

    #[test]
    fn fmt_invalid_arg() {
        let arg = "pockey";
        let available_arg = "[chocorooms, chococones]";
        assert_eq!(
            invalid_arg(arg, available_arg),
            "無効な引数`pockey`を確認\n利用可能な引数: `[chocorooms, chococones]`"
        );
    }

    #[test]
    fn fmt_ftg() {
        let error = "Message not found";
        assert_eq!(ftg(error), "Failed to get: Message not found");
    }

    #[test]
    fn fmt_anti_cb() {
        let cb = cb("console.log(\"Hello, World!\")", "js");
        assert_eq!(anti_cb(&cb), "```js\nconsole.log(\"Hello, World!\")\n```");
    }

    #[test]
    fn fmt_mono() {
        let content = "wolwol";
        assert_eq!(mono(content), "`wolwol`");
    }

    #[test]
    fn round_to_the_midpoint() {
        assert_eq!(round_mid(3.1415, 3), 3.142);
    }
}
*/
