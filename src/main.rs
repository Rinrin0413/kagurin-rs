use kgrs::serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, prelude::*},
    prelude::{Client, Context, EventHandler, SerenityError},
    utils::{Colour, MessageBuilder},
};
use kgrs::util::{fmt::*, *};
use serde_json::{json, Value};
use std::{collections::HashMap, env, time::Instant};

struct Handler;

const VER: &str = env!("CARGO_PKG_VERSION");
const EMBED_LABEL_COL: u32 = 0xB89089;
const TRUSTED: [u64; 2] = [
    724976600873041940, // Rinrin.rs
    801082943371477022, // Rinrin.hlsl
];
const DEVELIPER: [u64; 2] = [
    724976600873041940, // Rinrin.rs
    801082943371477022, // Rinrin.hlsl
];

#[async_trait]
impl EventHandler for Handler {
    // MSG event
    async fn message(&self, ctx: Context, msg: Message) {
        // ▼ DB
        let cache = &ctx.cache;
        let client = cache.current_user().await;
        let bot_avatar = &client.face();
        // ▲ DB

        // COMMANDS
        if Some("kgrs") == msg.content.split('!').nth(0) {
            // ▼ DB
            let cmd = match msg.content.split(' ').nth(0) {
                Some(v) => v.to_string().replace("kgrs!", ""),
                None => return,
            };
            let cmd: &str = &cmd;
            let arg_i = msg.content.split(' ').nth(1);
            let arg_ii = msg.content.split(' ').nth(2);
            let ftr = &format!(
                "kgrs!{} by {}",
                cmd,
                Et::Optn(
                    msg.member
                        .as_ref()
                        .expect(&Et::Other("").l(cmd, "GET MEMBER"))
                        .nick
                        .as_ref(),
                    &msg
                )
                .l(cmd, "FOOTER")
            );
            // ▲ DB「

            // kgrs!help [type:HELP_TYPE] | show help
            //  HELP_TYPE = [display, util, fun, mod, trusted, dev]
            if cmd == "help" {
                let note_cp = "Command prefix: `kgrs!`";
                if let Some(hlp_type) = arg_i {
                    match hlp_type {
                        "display" => {
                            let content = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.embed(|e| {
                                        e.author(|a| {
                                            a.icon_url(bot_avatar)
                                                .name("かぐりん.rs's Commands(display)")
                                        });
                                        e.title("表示系コマンド一覧");
                                        e.description(note_cp);
                                        e.fields(vec![
                                            ("kgrs!help", "コマンドのヘルプ(ハブ)を表示", false),
                                            ("kgrs!info", "このボットの詳細を表示", false),
                                            ("kgrs!ping", "pong!", false),
                                            ("kgrs!profile [UserID:int]", "対象のユーザの詳細を表示\n引数がない場合は実行者の詳細が送られる", false),
                                        ]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(chrono::Utc::now());
                                        e.color(Colour(EMBED_LABEL_COL));
                                        e
                                    })
                                })
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }
                        "util" => {
                            let content = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.embed(|e| {
                                        e.author(|a| {
                                            a.icon_url(bot_avatar)
                                                .name("かぐりん.rs's Commands(util)")
                                        });
                                        e.title("機能系コマンド一覧");
                                        e.description(note_cp);
                                        e.fields(vec![(
                                            "kgrs!timestamp",
                                            "現在のタイムスタンプを取得",
                                            false,
                                        )]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(chrono::Utc::now());
                                        e.color(Colour(EMBED_LABEL_COL));
                                        e
                                    })
                                })
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }
                        "fun" => {
                            let content = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.embed(|e| {
                                        e.author(|a| {
                                            a.icon_url(bot_avatar)
                                                .name("かぐりん.rs's Commands(fun)")
                                        });
                                        e.title("娯楽系コマンド一覧");
                                        e.description(note_cp);
                                        e.fields(vec![("kgrs!-", "-", false)]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(chrono::Utc::now());
                                        e.color(Colour(EMBED_LABEL_COL));
                                        e
                                    })
                                })
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }
                        "mod" => {
                            let content = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.embed(|e| {
                                        e.author(|a| {
                                            a.icon_url(bot_avatar)
                                                .name("かぐりん.rs's Commands(mod)")
                                        });
                                        e.title("管理者用コマンド一覧");
                                        e.description(note_cp);
                                        e.fields(vec![("kgrs!-", "-", false)]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(chrono::Utc::now());
                                        e.color(Colour(EMBED_LABEL_COL));
                                        e
                                    })
                                })
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }
                        "trusted" => {
                            let content = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.author(|a| a.icon_url(bot_avatar).name("かぐりん.rs's Commands(trusted)"));
                                    e.title("開発者に信頼されている人用のコマンド一覧");
                                    e.description(note_cp);
                                    e.fields(vec![
                                        (
                                            "kgrs!set_activity <type:ACTIVITY-TYPE> <content:str>", 
                                            "`ACTIVITY-TYPE = [playing, listening, watching, competing]`\nBot のアクティビティを変更する", 
                                            false
                                        ),
                                    ]);
                                    e.footer(|f| {f.text(ftr)});
                                    e.timestamp(chrono::Utc::now());
                                    e.color(Colour(EMBED_LABEL_COL));
                                    e
                                })
                            })
                            .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }
                        "dev" => {
                            let content = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.embed(|e| {
                                        e.author(|a| {
                                            a.icon_url(bot_avatar)
                                                .name("かぐりん.rs's Commands(dev)")
                                        });
                                        e.title("Rinrin用コマンド一覧");
                                        e.description(note_cp);
                                        e.fields(vec![(
                                            "kgrs!sd",
                                            "serenity-rs のドキュメントの URL を出す",
                                            false,
                                        )]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(chrono::Utc::now());
                                        e.color(Colour(EMBED_LABEL_COL));
                                        e
                                    })
                                })
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }
                        other => {
                            let content = msg
                                .channel_id
                                .say(
                                    &ctx.http,
                                    invalid_arg(other, "[display, util, fun, mod, trusted, dev]"),
                                )
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }
                    }
                } else {
                    let content = msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.author(|a| {
                                    a.icon_url(bot_avatar).name("かぐりん.rs's Commands HUB")
                                });
                                e.title("コマンドの種類一覧");
                                e.description(note_cp);
                                e.fields(vec![
                                    ("kgrs!help", "コマンドのヘルプ(ハブ)を表示", false),
                                    ("kgrs!help display", "表示系のコマンド一覧を表示", false),
                                    ("kgrs!help util", "機能系のコマンド一覧を表示", false),
                                    ("kgrs!help fun", "娯楽系のコマンド一覧を表示", false),
                                    ("kgrs!help mod", "管理者用のコマンド一覧を表示", false),
                                    (
                                        "kgrs!help trusted",
                                        "開発者に信頼されている人用のコマンド一覧を表示",
                                        false,
                                    ),
                                    ("kgrs!help dev", "Rinrin用のコマンド一覧を表示", false),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(chrono::Utc::now());
                                e.color(Colour(EMBED_LABEL_COL));
                                e
                            })
                        })
                        .await;
                    Et::Rslt(content).l(cmd, "SEND");
                }
            }

            // kgrs!info | show info
            if cmd == "info" {
                let guilds = cache.guild_count().await;
                let content = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.author(|a| {
                                a.icon_url(bot_avatar)
                                    .name(format!("{} ℹnformation", client.name))
                            });
                            e.title("Bot name:");
                            e.description(&format!("```py\n{}#{}\n```", client.name, client.discriminator));
                            e.fields(vec![
                                // If don't make the first type a &String slice, won't be able to use &str behind it
                                ("ID:", &format!("```c\n{}\n```", client.id)[..], true),
                                ("Bot version:", &format!("```c\n{}```", VER), true),
                                ("Created at:", &format!("<t:{}:R>", client.id.created_at().timestamp()), true), // 1643258000
                                ("Guilds:", &format!("```c\n{} guilds\n```", guilds), true),
                                ("Invile link:", "[here](https://discord.com/api/oauth2/authorize?client_id=936116497502318654&permissions=8&scope=bot)", true),
                                ("Developer:", "```nim\n@Rinrin.rs#5671```", true),
                                ("Language:", "```yaml\nRust: [1.58.1]```", true),
                                ("Library:", "```yaml\nserenity-rs: [0.10.10]```", true),
                                ("Source code:", &format!("[GitHub]({})", env!("CARGO_PKG_REPOSITORY")), true),
                            ]);
                            e.footer(|f| {f.text(ftr)});
                            e.timestamp(chrono::Utc::now());
                            e.color(Colour(EMBED_LABEL_COL));
                            e
                        })
                    })
                    .await;

                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!timestamp | get the current timestamp
            if cmd == "timestamp" {
                let content = msg
                    .channel_id
                    .say(&ctx.http, chrono::Utc::now().timestamp())
                    .await;

                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!profile [userID:int] | get the user desctiption
            if cmd == "profile" {
                if let Some(id) = arg_i {
                    let user = match UserId(match id.parse() {
                        Ok(id) => id,
                        Err(_) => {
                            let _ = msg
                                .channel_id
                                .say(
                                    &ctx.http,
                                    &format!(
                                        "無効な引数`{}`を確認\n引数には数値を入れてください",
                                        id
                                    ),
                                )
                                .await
                                .unwrap();
                            return;
                        }
                    })
                    .to_user(&ctx.http)
                    .await
                    {
                        Ok(u) => u,
                        Err(_) => {
                            let _ = msg
                                .channel_id
                                .say(&ctx.http, &format!("無効なユーザIDです: {}", id))
                                .await
                                .unwrap();
                            return;
                        }
                    };
                    let member = match msg
                        .guild_id
                        .expect(&Et::Other("").l(cmd, "GET GUILD ID"))
                        .member(&ctx.http, user.id)
                        .await
                    {
                        Ok(m) => Some(m),
                        Err(_) => None,
                    };
                    let user_color = ctx.http.get_user(*user.id.as_u64()).await;
                    let content = msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.author(|a| {
                                    a.icon_url(user.face())
                                        .name(format!("{}'s ℹnformation", user.name))
                                });
                                if let Some(b) = user.banner_url() {
                                    e.thumbnail(b);
                                }
                                e.title("User name:");
                                e.description(format!(
                                    "```py\n{}#{}```",
                                    user.name, user.discriminator
                                ));
                                e.fields(vec![
                                    (
                                        "Nickname:",
                                        format!(
                                            "`{}`",
                                            if let Some(m) = &member {
                                                if let Some(n) = &m.nick {
                                                    n
                                                } else {
                                                    "None"
                                                }
                                            } else {
                                                "None"
                                            }
                                        ),
                                        true,
                                    ),
                                    (
                                        "Created at:",
                                        format!("<t:{}:R>", user.created_at().timestamp()),
                                        true,
                                    ),
                                    (
                                        "Joined this server at:",
                                        if let Some(m) = &member {
                                            if let Some(t) = m.joined_at {
                                                format!("<t:{}:R>", t.timestamp())
                                            } else {
                                                "None".to_string()
                                            }
                                        } else {
                                            "None".to_string()
                                        },
                                        true,
                                    ),
                                    (
                                        "Is mute:",
                                        format!(
                                            "`{}`",
                                            if let Some(m) = &member { m.mute } else { false },
                                        ),
                                        true,
                                    ),
                                    (
                                        "Roles:",
                                        format!(
                                            "`{} roles`",
                                            if let Some(m) = &member {
                                                m.roles.len()
                                            } else {
                                                0
                                            },
                                        ),
                                        true,
                                    ),
                                    ("Is Bot:", user.bot.to_string(), true),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(chrono::Utc::now());
                                e.color(if let Ok(u) = user_color {
                                    //msg.author.accent_colour
                                    if let Some(c) = u.accent_colour {
                                        c
                                    } else {
                                        Colour(EMBED_LABEL_COL)
                                    }
                                } else {
                                    Colour(EMBED_LABEL_COL)
                                });
                                e
                            })
                        })
                        .await;

                    Et::Rslt(content).l(cmd, "SEND");
                } else {
                    let author_color = ctx.http.get_user(*msg.author.id.as_u64()).await;
                    let content = msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.author(|a| {
                                    a.icon_url(msg.author.face())
                                        .name(format!("{}'s ℹnformation", msg.author.name))
                                });
                                if let Some(b) = msg.author.banner_url() {
                                    e.thumbnail(b);
                                }
                                e.title("User name:");
                                e.description(format!(
                                    "```py\n{}#{}```",
                                    msg.author.name, msg.author.discriminator
                                ));
                                e.fields(vec![
                                    (
                                        "Nickname:",
                                        format!(
                                            "`{}`",
                                            match &msg
                                                .member
                                                .as_ref()
                                                .expect(&Et::Other("").l(cmd, "GET NICKNAME"))
                                                .nick
                                            {
                                                Some(n) => n,
                                                None => "None",
                                            }
                                        ),
                                        true,
                                    ),
                                    (
                                        "Created at:",
                                        format!("<t:{}:R>", msg.author.created_at().timestamp()),
                                        true,
                                    ),
                                    (
                                        "Joined this server at:",
                                        match &msg
                                            .member
                                            .as_ref()
                                            .expect(&Et::Other("").l(cmd, "GET JOINED AT"))
                                            .joined_at
                                        {
                                            Some(t) => format!("<t:{}:R>", t.timestamp()),
                                            None => "None".to_string(),
                                        },
                                        true,
                                    ),
                                    (
                                        "Is mute:",
                                        format!(
                                            "`{}`",
                                            msg.member
                                                .as_ref()
                                                .expect(&Et::Other("").l(cmd, "GET MUTE BOOL"))
                                                .mute
                                        ),
                                        true,
                                    ),
                                    (
                                        "Roles:",
                                        format!(
                                            "`{} roles`",
                                            msg.member
                                                .as_ref()
                                                .expect(&Et::Other("").l(cmd, "GET ROLES"))
                                                .roles
                                                .len()
                                        ),
                                        true,
                                    ),
                                    ("Is Bot:", msg.author.bot.to_string(), true),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(chrono::Utc::now());
                                e.color(if let Ok(u) = author_color {
                                    if let Some(c) = u.accent_colour {
                                        c
                                    } else {
                                        Colour(EMBED_LABEL_COL)
                                    }
                                } else {
                                    Colour(EMBED_LABEL_COL)
                                });
                                e
                            })
                        })
                        .await;

                    Et::Rslt(content).l(cmd, "SEND");
                }
            }

            // kgrs!ping | pong!
            if cmd == "ping" {
                let before = Instant::now();
                let mut content = msg
                    .channel_id
                    .say(&ctx.http, "pong!")
                    .await
                    .expect(&Et::Other("").l(cmd, "SEND PONG"));
                //Et::Rslt(content).l(cmd, "SEND");
                let after = Instant::now();
                content
                    .edit(&ctx, |m| {
                        m.content(format!("pong! ({}ms)", (after - before).as_millis()))
                    })
                    .await
                    .expect(&Et::Other("").l(cmd, "SEND PONG"));
            }

            // FOR TRUSTED USER
            if restrict_users(msg.author.id, &TRUSTED) {
                // kgrs!set_activity <type:ACTIVITY-TYPE> <content:str> || Change Kagurin.rs activity
                //  ACTIVITY-TYPE = [playing, listening, watching, competing]
                if cmd == "set_activity" {
                    if let (Some(r#type), Some(content)) = (arg_i, arg_ii) {
                        match r#type {
                            "playing" => ctx.set_activity(Activity::playing(content)).await,
                            "listening" => ctx.set_activity(Activity::listening(content)).await,
                            "watching" => ctx.set_activity(Activity::watching(content)).await,
                            "competing" => ctx.set_activity(Activity::competing(content)).await,
                            other => {
                                let content = msg
                                    .channel_id
                                    .say(
                                        &ctx.http,
                                        invalid_arg(
                                            other,
                                            "[playing, listening, watching, competing]",
                                        ),
                                    )
                                    .await;
                                Et::Rslt(content).l(cmd, "SEND");
                                return;
                            }
                        }
                        /*// do not work
                        let reaction = msg
                            .reaction_users(
                                &ctx.http,
                                ReactionType::Custom {
                                    animated: false,
                                    id: EmojiId(856911443390627840),
                                    name: Some(String::from("HAAKU_death")),
                                },
                                Some(64),
                                Some(msg.author.id),
                            )
                            .await;
                        if let Err(why) = reaction {
                            println!("kgrs!{} / REACTION : {:?}", cmd, why);
                        }
                        */
                        println!(
                            "{} が botアクティビティを {} の `{}` に変更",
                            msg.author.name, r#type, content
                        );
                    } else {
                        let content = msg.channel_id.say(&ctx.http, &want_arg(2)).await;
                        Et::Rslt(content).l(cmd, "SEND");
                    }
                }
            }

            // DEV
            if restrict_users(msg.author.id, &DEVELIPER) {
                // kgrs!sd | show serenity-rs doc
                if cmd == "sd" {
                    let content = msg
                        .channel_id
                        .say(&ctx.http, "https://docs.rs/serenity/latest/serenity")
                        .await;

                    Et::Rslt(content).l(cmd, "SEND");
                }
            }
        }

        /*

        // directMsg
        if msg.content == "kgrs!directMsg" {
            let cn = "directMsg";
            let content = msg.author.dm(&ctx, |m| m.content("Yoooo")).await;
            Et::Rslt(content).l(cn, "SEND DM");
        }

        // MessageBuilder
        if msg.content == "kgrs!MessageBuilder" {
            let cn = "MessageBuilder";
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(c) => c,
                Err(why) => {
                    println!("kgrs!{} / {} : {:?}", cn, "GET CHANNEL", why);
                    return;
                }
            }; //l(cn, "GET CHANNEL", Et::Rslt(msg.channel_id.to_channel(&ctx).await)); // e fun not compatible
            let message = MessageBuilder::new()
                .push("贵樣(")
                .push_bold_safe(&msg.author.name)
                .push(") ば `kgrs!dynamiccmd` を ")
                .mention(&channel)
                .push(" で使用レだ。")
                .build();
            let content = msg.channel_id.say(&ctx.http, &message).await;
            Et::Rslt(content).l(cn, "SEND");
        }

        // MessageBuilder2
        if msg.content == "kgrs!MessageBuilder2" {
            let cn = "MessageBuilder2";
            let emoji = serde_json::from_value::<Emoji>(json!({
                "animated": false,
                "id": EmojiId(921759722170904618),
                "managed": true,
                "name": "yushyu_no_jinzay".to_string(),
                "require_colons": true,
                "roles": Vec::<Role>::new(),
            }))
            .unwrap();
            let content = MessageBuilder::new()
                .push("通常の文字列")
                .push_codeblock("print(\"コードブロック\")", Some("py"))
                .push_mono("コードインライン")
                .push_italic("斜体")
                .push_bold("太字")
                .push_underline("下線")
                .push_strike("取り消し線")
                .push_spoiler("スポイラー")
                .push_quote("引用")
                .push_line("末尾に改行")
                .push_safe("discord.gg/7QhMDfyPHR @here *アスタリスク* `グレイヴ・アクセント` _アンダーライン_")
                .push_underline_safe("下__線")
                .channel(ChannelId(894239318330179624))
                .emoji(&emoji)
                .mention(&UserId(724976600873041940))
                .role(RoleId(835851285982478346))
                .user(&UserId(801082943371477022))
                .build();
            //println!("{:?}", content);
            Et::Rslt(msg.channel_id.say(&ctx.http, &content).await).l(cn, "SEND");
        }

        // embed&img
        if msg.content == "kgrs!embed&img" {
            let cn = "!embed&img";
            let mut author: HashMap<&'static str, Value> = HashMap::new();
            author.insert("贵樣", Value::String(String::from("a")));
            let content = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("ただの文章")
                        .embed(|e| {
                            // 埋め込み(E.bed)のタイトル
                            e.title("全体タイトル");

                            // 埋め込みの説明
                            e.description("全体説明");

                            // 画像添付
                            e.image("https://raw.githubusercontent.com/Rinrin0413/kagurin-rs/master/static/MCSplashScreen.scale-200.png"); //./static/MCSplashScreen.scale-200.png")

                            // インラインのフィールド(複数)
                            e.fields(vec![
                                (
                                    "第壱フィールドタイトル",
                                    "第壱フィールドボディ(インライン)",
                                    true,
                                ),
                                (
                                    "第贰フィールドタイトル",
                                    "第贰フィールドボディ(インライン)",
                                    true,
                                ),
                            ]);

                            // ブロックのフィールド
                            e.field(
                                "第叁フィールドタイトル",
                                "第叁フィールドボディ(インラインじゃないよ)",
                                false,
                            );

                            // フッター
                            e.footer(|f| f.text("フッター"));

                            // タイムスタンプ
                            e.timestamp(chrono::Utc::now());

                            // 著者
                            e.author(|a| {
                                // authorアイコン
                                a.icon_url(&msg.author.face());
                                // author名
                                a.name(&msg.author.name)
                            });

                            // 埋め込みカラー
                            e.color(Colour(0xFFCDC9));

                            // サムネイル
                            e.thumbnail("https://raw.githubusercontent.com/Rinrin0413/kagurin-rs/master/static/atking-of-the-pancake.png"); //./static/atking-of-the-pancake.png")

                            e // いろいろ改変した挙句 e を返す
                        })
                        // ↓ ただの画像添付
                        .add_file("./static/atk-of-the-pancake.png")
                })
                .await;

            Et::Rslt(content).l(cn, "SEND");
        }
        */
    }

    // READY event
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "{} v{} is connected: {}",
            ready.user.name,
            VER,
            chrono::Utc::now()
        );
        ctx.set_activity(Activity::playing("kgrs!help | 開発者:Rinrin.rs#5671"))
            .await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("KAGURIN_RS_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
