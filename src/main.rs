use chrono::{DateTime, Duration, Utc};
use kgrs::{
    tetr::*,
    util::{fmt::*, *},
};
use reqwest;
use serde_json::{json, Value};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, prelude::*},
    prelude::{Client, Context, EventHandler},
    utils::{Colour, MessageBuilder},
};
use std::{collections::HashMap, env, process, time::Instant};
use thousands::Separable;

struct Handler {
    client: reqwest::Client,
}

const RUST_VERSION: &str = "1.64.0-nightly";
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
const BOT_ID: u64 = 936116497502318654;
const IS_DST: bool = true; // Is daylight saving time(for Sky:CotL)
const INVITE_URL: &str =
    "https://discord.com/api/oauth2/authorize?client_id=936116497502318654&permissions=8&scope=bot";

const TETRA_CHANNEL_API: &str = "https://ch.tetr.io/api/";

#[async_trait]
impl EventHandler for Handler {
    // MSG event
    async fn message(&self, ctx: Context, msg: Message) {
        // if msg author is bot, return;
        if msg.author.bot {
            return;
        }

        // DB
        let cache = &ctx.cache;
        let client = cache.current_user().await;
        let bot_avatar = &client.face();

        // If mentioned to bot, will send about help cmd
        if msg
            .mentions_me(&ctx.http)
            .await
            .expect("GET MENTIONS TO ME")
        {
            let content = msg.reply(&ctx.http, "ヘルプは `kgrs!help` でどうぞ").await;
            Et::Rslt(content).l("HLP", "SEND");
        }

        // COMMANDS
        if Some("kgrs") == msg.content.split('!').nth(0) {
            // ▼ DB
            let cmd = match msg.content.split(' ').nth(0) {
                Some(v) => v.to_string().replace("kgrs!", ""),
                None => return,
            };
            let cmd: &str = &cmd;
            let arg = setup_arg(7, &msg.content);
            let ftr = &format!(
                "kgrs!{}, Called by {}",
                cmd,
                Et::Optn(
                    // If there is no member data(dm...?), return nick, else name
                    if let Some(m) = &msg.member {
                        m.nick.as_ref()
                    } else {
                        Some(&msg.author.name)
                    },
                    &msg
                )
                .l(cmd, "FOOTER")
            );
            // ▲ DB

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

            // kgrs!help [type:HELP_TYPE] | show help
            //  HELP_TYPE = [display, util, fun, mod, trusted, dev]
            if cmd == "help" {
                let note_cp = "Command prefix: `kgrs!`";
                if let Some(hlp_type) = arg[1] {
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
                                            ("kgrs!avatar [UserID:int]", "対象のユーザのアイコンを表示\n引数がない場合は実行者のアイコンが表示される", false),
                                            ("kgrs!server_info [ServerID:int]", "対象のサーバーの詳細を表示\n引数がない場合は実行したサーバーの詳細が送られる", false),
                                            ("kgrs!sky", "Sky:CotL の次の更新時刻を表示", false),
                                            ("kgrs!invite", "このボットの招待URLを取得できる", false),
                                        ]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(Utc::now());
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
                                        e.fields(vec![
                                            ("kgrs!now", "現在の UNIX時間を取得", false),
                                            ("kgrs!timestamp <year:int> <month:int> <day:int> [hour:int] [minute:int] [second:int] [millisecond:int]", "指定された日時の UNIX時間を取得", false),
                                            (
                                                "kgrs!uuid [How-many:int] [Is-uppercase:bool]",
                                                "uuidを生成",
                                                false,
                                            ),
                                        ]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(Utc::now());
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
                                        e.fields(vec![(
                                            "kgrs!gtb",
                                            "伝統的なオニオンガーリックブリトーランダム",
                                            false,
                                        )]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(Utc::now());
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
                                        e.timestamp(Utc::now());
                                        e.color(Colour(EMBED_LABEL_COL));
                                        e
                                    })
                                })
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                        }

                        "tetr" => {
                            let content = msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m.embed(|e| {
                                        e.author(|a| {
                                            a.icon_url(bot_avatar)
                                                .name("かぐりん.rs's Commands(tetr)")
                                        });
                                        e.title("TETR.IO関連コマンド一覧");
                                        e.description(note_cp);
                                        e.fields(vec![(
                                            "kgrs!tetr-user <user:str>",
                                            "TETR.IOのユーザー情報を表示",
                                            false,
                                        )]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(Utc::now());
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
                                    e.timestamp(Utc::now());
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
                                        e.fields(vec![
                                            ("kgrs!exit", "プロセスを強制終了", false),
                                            (
                                                "kgrs!sd",
                                                "serenity-rs のドキュメントの URL を出す",
                                                false,
                                            ),
                                            ("kgrs!direct_msg", "実行者に dm を送る", false),
                                            ("kgrs!message_builder", "MessageBuilder test1", false),
                                            (
                                                "kgrs!message_builder2",
                                                "MessageBuilder test2",
                                                false,
                                            ),
                                            ("kgrs!embed_and_img", "embed & img test", false),
                                            (
                                                "kgrs!br",
                                                "bevy::render のドキュメントの URL を出す",
                                                false,
                                            ),
                                        ]);
                                        e.footer(|f| f.text(ftr));
                                        e.timestamp(Utc::now());
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
                                    ("kgrs!help tetr", "TETR.IO関連のコマンド一覧を表示", false),
                                    (
                                        "kgrs!help trusted",
                                        "開発者に信頼されている人用のコマンド一覧を表示",
                                        false,
                                    ),
                                    ("kgrs!help dev", "Rinrin用のコマンド一覧を表示", false),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
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
                            e.description(&format!(
                                "```py\n{}#{}\n```",
                                client.name, client.discriminator
                            ));
                            e.fields(vec![
                                // If don't make the first type a &String slice, won't be able to use &str behind it
                                ("ID:", &format!("```c\n{}\n```", client.id)[..], true),
                                ("Bot version:", &format!("```c\n{}```", VER), true),
                                (
                                    "Created at:",
                                    &format!("<t:{}:R>", client.id.created_at().timestamp()),
                                    true,
                                ), // 1643258000
                                ("Guilds:", &format!("```c\n{} guilds\n```", guilds), true),
                                ("Invitation link:", &format!("[here]({})", INVITE_URL), true),
                                ("Developer:", "```nim\n@Rinrin.rs#5671```", true),
                                (
                                    "Language:",
                                    &format!("```yaml\nRust {}```", RUST_VERSION),
                                    true,
                                ),
                                ("Library:", "```yaml\nserenity-rs: [0.10.10]```", true),
                                (
                                    "Source code:",
                                    &format!("[GitHub]({})", env!("CARGO_PKG_REPOSITORY")),
                                    true,
                                ),
                            ]);
                            e.footer(|f| f.text(ftr));
                            e.timestamp(Utc::now());
                            e.color(Colour(EMBED_LABEL_COL));
                            e
                        })
                    })
                    .await;

                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!now | get the current unixtime
            if cmd == "now" {
                let content = msg.channel_id.say(&ctx.http, Utc::now().timestamp()).await;
                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!timestamp <year:int> <month:int> <day:int> [hour:int] [minute:int] [second:int] [millisecond:int] | get unixtime
            if cmd == "timestamp" {
                let dt: String = if let (Some(yea), Some(mon), Some(day)) = (arg[1], arg[2], arg[3])
                {
                    if let Some(hou) = arg[4] {
                        if let Some(min) = arg[5] {
                            if let Some(sec) = arg[6] {
                                if let Some(ms) = arg[7] {
                                    format!(
                                        "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}.+{:0>4}",
                                        yea, mon, day, hou, min, sec, ms
                                    )
                                } else {
                                    format!(
                                        "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}.+0000",
                                        yea, mon, day, hou, min, sec
                                    )
                                }
                            } else {
                                format!(
                                    "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:00.+0000",
                                    yea, mon, day, hou, min
                                )
                            }
                        } else {
                            format!(
                                "{:0>4}-{:0>2}-{:0>2} {:0>2}:00:00.+0000",
                                yea, mon, day, hou
                            )
                        }
                    } else {
                        format!("{:0>4}-{:0>2}-{:0>2} 00:00:00.+0000", yea, mon, day)
                    }
                } else {
                    let content = msg.channel_id.say(&ctx.http, &missing_arg(3)).await;
                    Et::Rslt(content).l(cmd, "SEND");
                    return;
                };

                let dt_fmt = "%Y-%m-%d %H:%M:%S.%z";
                let content = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "{}",
                            match DateTime::parse_from_str(&dt, dt_fmt) {
                                Ok(dt) => dt.timestamp().to_string(),
                                Err(why) => format!("```rs\nError: {}\n{}\n```", why, dt),
                            }
                        ),
                    )
                    .await;
                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!profile [userID:int] | get the user desctiption
            if cmd == "profile" {
                if let Some(id) = arg[1] {
                    let user = if let Some(u) = get_user_from_id(id, &msg, &ctx).await {
                        u
                    } else {
                        return;
                    };
                    let member = get_member_from_user(&user, &msg, &ctx).await;
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
                                    ("Is Bot:", format!("`{}`", user.bot.to_string()), true),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
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
                                            if let Some(n) = if let Some(m) = &msg.member {
                                                m.nick.as_ref()
                                            } else {
                                                Some(&msg.author.name)
                                            } {
                                                n
                                            } else {
                                                "None"
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
                                        if let Some(n) = if let Some(m) = &msg.member {
                                            m.joined_at.as_ref()
                                        } else {
                                            None
                                        } {
                                            format!("<t:{}:R>", n.timestamp())
                                        } else {
                                            "None".to_string()
                                        },
                                        true,
                                    ),
                                    (
                                        "Is mute:",
                                        format!(
                                            "`{}`",
                                            if let Some(m) = &msg.member {
                                                m.mute
                                            } else {
                                                false
                                            }
                                        ),
                                        true,
                                    ),
                                    (
                                        "Roles:",
                                        format!(
                                            "`{} roles`",
                                            if let Some(m) = &msg.member {
                                                m.roles.len()
                                            } else {
                                                0
                                            }
                                        ),
                                        true,
                                    ),
                                    ("Is Bot:", format!("`{}`", msg.author.bot.to_string()), true),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
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

            // kgrs!avatar [userID:int] | get the user avatar
            if cmd == "avatar" {
                if let Some(id) = arg[1] {
                    let user = if let Some(u) = get_user_from_id(id, &msg, &ctx).await {
                        u
                    } else {
                        return;
                    };
                    let user_color = ctx.http.get_user(*user.id.as_u64()).await;
                    let content = msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.description(format!(
                                    "**[{}'s avatar]({})**",
                                    user.name,
                                    user.face()
                                ));
                                e.image(user.face());
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
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
                                e.description(format!(
                                    "**[{}'s avatar]({})**",
                                    msg.author.name,
                                    msg.author.face()
                                ));
                                e.image(msg.author.face());
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
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

            // kgrs!uuid [How-many:int] [Is-uppercase:bool] | genelate uuids
            if cmd == "uuid" {
                if let Some(n) = arg[1] {
                    let n: u8 = if let Ok(n) = n.parse() { n } else { 0 };
                    if 1 <= n && n <= 16 {
                        let mut uuids = String::new();
                        for _ in 0..n {
                            uuids = format!(
                                "{}```yaml\n{}\n```",
                                uuids,
                                match get_upper_or_lower_uuid(arg[2], &msg, &ctx).await {
                                    Some(id) => id,
                                    None => return,
                                }
                            );
                        }
                        let content = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.title("Successful!");
                                    e.description(format!("{}", uuids));
                                    e.footer(|f| f.text(ftr));
                                    e.timestamp(Utc::now());
                                    e.color(Colour(EMBED_LABEL_COL));
                                    e
                                })
                            })
                            .await;
                        Et::Rslt(content).l(cmd, "SEND");
                    } else {
                        let content = msg
                            .channel_id
                            .say(&ctx.http, "無効な引数を確認\n第1引数には `1` から `16` までの整数値を入れてください")
                            .await;
                        Et::Rslt(content).l(cmd, "SEND");
                    }
                } else {
                    let uuid = match get_upper_or_lower_uuid(arg[2], &msg, &ctx).await {
                        Some(id) => id,
                        None => return,
                    };
                    let content = msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.title("Successful!");
                                e.description(format!("```yaml\n{}\n```", uuid));
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
                                e.color(Colour(EMBED_LABEL_COL));
                                e
                            })
                        })
                        .await;
                    Et::Rslt(content).l(cmd, "SEND");
                }
            }

            // kgrs!sky
            if cmd == "sky" {
                // Time difference
                let td = Duration::hours(9);
                // now(JST)
                let now = Utc::now() + td;
                // now hour(JST)
                let now_h: u8 = now
                    .format("%H")
                    .to_string()
                    .parse()
                    .expect("String PARSE TO u8");
                // Daily Reset Time hours
                let drt_h: u8 = if IS_DST { 16 } else { 17 };
                // Base Daily Reset Time
                let drt_fmt = format!("%Y/%m/%d {}:00:00.+0000", drt_h);
                let drt_str: &str = &now.format(&drt_fmt).to_string();
                // Daily Reset Time(unfinished)
                let drt = match DateTime::parse_from_str(drt_str, "%Y/%m/%d %H:%M:%S.%z") {
                    Ok(dt) => dt - td,
                    Err(why) => {
                        let content = msg
                            .channel_id
                            .say(&ctx.http, &format!("えらー: ```rs\n{}\n```", why))
                            .await;
                        Et::Rslt(content).l(cmd, "SEND");
                        return;
                    }
                };

                // Daily Reset Time Result
                let ut =
                    // 00:00 ~ (DST hour - 1min) 
                    if now_h < drt_h {
                        drt.timestamp()
                    // DST hour ~ 23:59
                    } else if drt_h <= now_h {
                        let ut_next_day = drt + Duration::days(1);
                        ut_next_day.timestamp()
                    } else {
                        let content = msg
                            .channel_id
                            .say(&ctx.http, &format!("例外処理です\nError: {}", now_h))
                            .await;
                        Et::Rslt(content).l(cmd, "SEND");
                        return
                };
                let content = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Sky:CotL 次の更新時刻");
                            e.description(&format!("<t:{}:R>", ut));
                            e.footer(|f| f.text(ftr));
                            e.timestamp(Utc::now());
                            e.color(Colour(EMBED_LABEL_COL));
                            e
                        })
                    })
                    .await;
                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!gtb | onion garlic burrito
            if cmd == "gtb" {
                let tamanegi = ["オニオン", "ガーリック", "ブリトー"];
                let content = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("{}が当たりました", rand_choise(&tamanegi)),
                    )
                    .await;

                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!server_info
            if cmd == "server_info" {
                if let Some(id) = arg[1] {
                    let id: u64 = match id.parse() {
                        Ok(i) => i,
                        Err(_) => {
                            let content = msg
                                .channel_id
                                .say(
                                    &ctx.http,
                                    &format!(
                                        "無効な引数`{}`を確認\n引数にはサーバーIDを入れてください",
                                        arg[1].unwrap()
                                    ),
                                )
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                            return;
                        }
                    };
                    let guild = GuildId(id);
                    let g_name = optn_unzip(guild.name(cache).await, "Unknown Server");
                    let g_bans = get_len_of_vec(guild.bans(&ctx.http).await);
                    let g_channels = get_len_of_hm(guild.channels(&ctx.http).await);
                    let g_roles = get_len_of_hm(guild.roles(&ctx.http).await);
                    let g_emojis = get_len_of_vec(guild.emojis(&ctx.http).await);
                    //let g_members = get_len_from_vec(guild.members(&ctx.http, None, None).await);
                    let content = msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.author(|a| {
                                    a.icon_url(msg.author.face())
                                        .name(format!("{} ℹnformation", g_name))
                                });
                                if let Some(b) = msg.author.banner_url() {
                                    e.thumbnail(b);
                                }
                                e.fields(vec![
                                    ("ID:", format!("```c\n{}\n```", guild), true),
                                    //("Members", format!("```c\n{}\n```", g_members), true),
                                    ("Bans:", format!("```c\n{} users\n```", g_bans), true),
                                    ("Channels:", format!("```c\n{}\n```", g_channels), true),
                                    ("Roles:", format!("```c\n{}\n```", g_roles), true),
                                    ("Emojis:", format!("```c\n{}\n```", g_emojis), true),
                                    (
                                        "Created at",
                                        format!("<t:{}:R>", guild.created_at().timestamp()),
                                        true,
                                    ),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
                                e.color(Colour(EMBED_LABEL_COL));
                                e
                            })
                        })
                        .await;
                    Et::Rslt(content).l(cmd, "SEND");
                } else {
                    let guild = match msg.guild_id {
                        Some(g) => g,
                        None => {
                            let content = msg
                                .channel_id
                                .say(&ctx.http, "ここはサーバーではないようです")
                                .await;
                            Et::Rslt(content).l(cmd, "SEND");
                            return;
                        }
                    };
                    let g_name = optn_unzip(guild.name(cache).await, "Unknown Server");
                    let g_bans = get_len_of_vec(guild.bans(&ctx.http).await);
                    let g_channels = get_len_of_hm(guild.channels(&ctx.http).await);
                    let g_roles = get_len_of_hm(guild.roles(&ctx.http).await);
                    let g_emojis = get_len_of_vec(guild.emojis(&ctx.http).await);
                    //let g_members = get_len_from_vec(guild.members(&ctx.http, None, None).await);
                    let content = msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.author(|a| {
                                    a.icon_url(msg.author.face())
                                        .name(format!("{} ℹnformation", g_name))
                                });
                                if let Some(b) = msg.author.banner_url() {
                                    e.thumbnail(b);
                                }
                                e.fields(vec![
                                    ("ID:", format!("```c\n{}\n```", guild), true),
                                    //("Members", format!("```c\n{}\n```", g_members), true),
                                    ("Bans:", format!("```c\n{} users\n```", g_bans), true),
                                    ("Channels:", format!("```c\n{}\n```", g_channels), true),
                                    ("Roles:", format!("```c\n{}\n```", g_roles), true),
                                    ("Emojis:", format!("```c\n{}\n```", g_emojis), true),
                                    (
                                        "Created at",
                                        format!("<t:{}:R>", guild.created_at().timestamp()),
                                        true,
                                    ),
                                ]);
                                e.footer(|f| f.text(ftr));
                                e.timestamp(Utc::now());
                                e.color(Colour(EMBED_LABEL_COL));
                                e
                            })
                        })
                        .await;
                    Et::Rslt(content).l(cmd, "SEND");
                }
            }

            // kgrs!invite | get the invitation URL
            if cmd == "invite" {
                let content = msg
                    .channel_id
                    .say(&ctx.http, format!("{}", INVITE_URL))
                    .await;
                Et::Rslt(content).l(cmd, "SEND");
            }

            // kgrs!tetr-user |
            if cmd == "tetr-user" {
                if let Some(usr1) = arg[1] {
                    // If the request to the API contains `/`, the API will returns `Cannot GET Error`.
                    // So removes `/` before to request.
                    let mut usr1 = usr1.to_string();
                    usr1.retain(|c| c != '/');
                    usr1.retain(|c| c != '%');
                    usr1.retain(|c| c != '#');
                    usr1.retain(|c| c != '?');
                    usr1.retain(|c| c != '\\');
                    usr1.retain(|c| c != '.');
                    // Also, if only `/`, '%', '#', '?', '\', '.' is passed, the API gets nothing.
                    // So warns the user to enter the correct arguments.
                    // And returns.
                    if usr1 == "" {
                        let content = msg
                            .channel_id
                            .say(&ctx.http, "Error: ユーザー名又は ID を指定してください")
                            .await;
                        Et::Rslt(content).l(cmd, "SEND");
                        return;
                    }
                    usr1 = usr1.to_lowercase();
                    let mut content = msg
                        .channel_id
                        .say(&ctx.http, "Please wait...")
                        .await
                        .expect(&Et::Other("").l(cmd, "SEND \"PleaseWait\""));
                    let before_timestamp = Instant::now();
                    let user_data_url = format!("{}users/{}", TETRA_CHANNEL_API, usr1);
                    content
                        .edit(&ctx, |m| m.content("Please wait.."))
                        .await
                        .expect(&Et::Other("").l(cmd, "MSG EDIT(wait..)"));
                    let response = self.client.get(&user_data_url).send().await;
                    let after_timestamp = Instant::now();
                    let ping = format!(
                        "ping: {}ms",
                        (after_timestamp - before_timestamp).as_millis()
                    );
                    content
                        .edit(&ctx, |m| m.content("Please wait."))
                        .await
                        .expect(&Et::Other("").l(cmd, "MSG EDIT(wait.)"));
                    if usr1 == "syabetarou" {
                        content
                            .edit(&ctx, |m| m.content("Please wait"))
                            .await
                            .expect(&Et::Other("").l(cmd, "MSG EDIT(wait)"));
                        let after_timestamp2 = Instant::now();
                        let ping2 = format!(
                            "ping: {}ms",
                            (after_timestamp2 - before_timestamp).as_millis()
                        );
                        content
                            .edit(&ctx, |m| {
                                m.embed(|e| {
                                    e.title("SYABETAROU ✓ ||77a02950-bde0447f9851fd||");
                                    e.description("**<SUPPORTER>**");
                                    e.fields(vec![
                                        (
                                            "Badges: <:100player:992097864081735730><:allclear:992096168664383622><:20tsd:992097227260567553><:secretgrade:992079389611278477><:leaderboard1:992095621018308759>| More 18 badges",
                                            "​",
                                            false,
                                        ),
                                        (
                                            "〔<:xx:994631831460790272> **25000.0000TR** 〕\n　Global: №1\n　Local: №1",
                                            "<:x_:993091489376776232>|`━━━━━━━━━━━━━━━━━`👑\n　　　　　ℕ°𝟙",
                                            false,
                                        ),
                                        ("About me:", &cb("まいど。", ""), false),
                                        ("Role:", "User", true),
                                        (
                                            "Play time:",
                                            "2000 years",
                                            true,
                                        ),
                                        (
                                            "Friends:",
                                            "0",
                                            true,
                                        ),
                                        (
                                            "​",
                                            "[**== TETRA LEAGUE ==**](https://ch.tetr.io/s/league_userrecent_77a02950-bde0447f9851fd)",
                                            false,
                                        ),
                                        ("Glicko:", "9999.999±60.00", true),
                                        (
                                            "Play count:",
                                            "999,999",
                                            true,
                                        ),
                                        (
                                            "Wins:",
                                            "999,999 (100.000%)",
                                            true,
                                        ),
                                        ("APM:", "1003.84", true),
                                        ("PPS:", "84.40", true),
                                        ("VS:", "6301.33", true),
                                        (
                                            "​",
                                            &format!(
                                                "{} | cached at: <t:{}:R>",
                                                ping2,
                                                Utc::now().timestamp()
                                            ),
                                            false,
                                        )
                                    ]);
                                    e.footer(|f| f.text(ftr));
                                    e.timestamp(Utc::now());

                                    e.author(|a| {
                                        a.icon_url("https://tetr.io/res/flags/jp.png");
                                        a.name(&format!(
                                            "Lv.9999 ⬢ 8,401,9463,557xp",
                                        ))
                                    });

                                    e.color(Colour(0xeca5ff));
                                    e.thumbnail("https://cdn.discordapp.com/avatars/518899666637553667/3ae6b018626d3b596c31c241a56df088.webp");
                                    e
                                });
                                m.content("")
                            })
                            .await
                            .expect(&Et::Other("").l(cmd, "MSG EDIT(TETR-USER)"));
                    } else {
                        match response {
                            Ok(loot) => {
                                let tetr_usr = TetraUser::new(loot).await;
                                if let Err(err) = tetr_usr.get_err() {
                                    content
                                        .edit(&ctx, |m| {
                                            m.embed(|e| {
                                                e.title(usr1.to_uppercase());
                                                e.description(format!(
                                                    "```\n{}\n```\n{}",
                                                    err, &ping
                                                ));
                                                e.footer(|f| f.text(ftr));
                                                e.timestamp(Utc::now());
                                                e.color(Colour(EMBED_LABEL_COL))
                                            })
                                            .content("")
                                        })
                                        .await
                                        .expect(
                                            &Et::Other("").l(cmd, "MSG EDIT(FAILED TO RESPONSE)"),
                                        );
                                } else if tetr_usr.is_banned() {
                                    content
                                        .edit(&ctx, |m| {
                                            m.embed(|e| {
                                                e.title(format!(
                                                    "{}  ||{}||",
                                                    usr1.to_uppercase(),
                                                    tetr_usr.get_id()
                                                ));
                                                e.description("​");
                                                e.image("https://ch.tetr.io/res/cute.png");
                                                e.field("| **BANNED** |", "​", false);
                                                e.footer(|f| f.text(format!("{}\n{}", ping, ftr)));
                                                e.timestamp(Utc::now());
                                                e.color(Colour(0xf81c1c));
                                                e.thumbnail(
                                                    "https://tetr.io/res/avatar-banned.png",
                                                );
                                                e
                                            });
                                            m.content("")
                                        })
                                        .await
                                        .expect(&Et::Other("").l(cmd, "MSG EDIT(TETR-USER)"));
                                } else {
                                    let user_records_url = format!("{}/records", user_data_url);
                                    let response2 = self.client.get(&user_records_url).send().await;
                                    let records = match response2 {
                                        Ok(loot) => TetraRecords::new(loot).await,
                                        Err(err) => {
                                            content
                                                .edit(&ctx, |m| {
                                                    m.embed(|e| {
                                                        e.title(usr1.to_uppercase());
                                                        e.description(format!(
                                                            "```\n{}\n```\n{}",
                                                            err, &ping
                                                        ));
                                                        e.footer(|f| f.text(ftr));
                                                        e.timestamp(Utc::now());
                                                        e.color(Colour(EMBED_LABEL_COL))
                                                    })
                                                    .content("")
                                                })
                                                .await
                                                .expect(
                                                    &Et::Other("")
                                                        .l(cmd, "MSG EDIT(FAILED TO RESPONSE)"),
                                                );
                                            return;
                                        }
                                    };
                                    content
                                        .edit(&ctx, |m| m.content("Please wait"))
                                        .await
                                        .expect(&Et::Other("").l(cmd, "MSG EDIT(wait)"));
                                    let after_timestamp2 = Instant::now();
                                    let ping2 = format!(
                                        "ping: {}ms",
                                        (after_timestamp2 - before_timestamp).as_millis()
                                    );
                                    content
                                        .edit(&ctx, |m| {
                                            m.embed(|e| {
                                                e.title(format!(
                                                    "{}{}  ||{}||",
                                                    usr1.to_uppercase(),
                                                    if tetr_usr.is_verified() { " ✓" } else { "" },
                                                    tetr_usr.get_id()
                                                ));
                                                if tetr_usr.is_supporter() {
                                                    let supporter_card = format!(
                                                        "**<{:★>st$}>**",
                                                        "SUPPORTER",
                                                        st = tetr_usr.get_supporter_tier() + 9 - 1
                                                    );
                                                    if tetr_usr.is_bad_standing() {
                                                        e.description(format!(
                                                            "{}\n| **- BAD STANDING -** |",
                                                            supporter_card
                                                        ));
                                                    } else {
                                                        e.description(supporter_card);
                                                    }
                                                } else if tetr_usr.is_bad_standing() {
                                                    e.description("| **- BAD STANDING -** |");
                                                }
                                                if let Some(url) = tetr_usr.get_banner_url() {
                                                    e.image(url);
                                                }
                                                if tetr_usr.has_badges() {
                                                    e.field(
                                                        format!("Badges: {}", tetr_usr.get_bages()),
                                                        "​",
                                                        false,
                                                    );
                                                }
                                                if tetr_usr.is_rating() {
                                                    e.field(
                                                        format!(
                                                            "〔{} **{:.4}TR** 〕{}",
                                                            tetr_usr.get_rank_emoji(),
                                                            tetr_usr.get_rating(),
                                                            match tetr_usr.get_rank() {
                                                                "z" => "".to_string(),
                                                                _ => format!(
                                                                    "\n　Global: №{}\n　Local: №{}",
                                                                    tetr_usr.get_standing(),
                                                                    tetr_usr.get_standing_local()
                                                                ),
                                                            }
                                                        ),
                                                        match tetr_usr.get_rank() {
                                                            "z" => format!(
                                                                "Probably around {}",
                                                                tetr_usr.get_percentile_rank()
                                                            ),
                                                            _ => tetr_usr.get_demotion_on_next_loss(),
                                                        },
                                                        false,
                                                    );
                                                } else {
                                                    e.field(
                                                        format!(
                                                            "**{}/10** rating games played",
                                                            tetr_usr.get_gamesplayed()
                                                        ),
                                                        format!(
                                                            "{} rating games won",
                                                            tetr_usr.get_gameswon()
                                                        ),
                                                        false,
                                                    );
                                                };
                                                if let Some(bio) = tetr_usr.get_bio() {
                                                    if 0 < bio.len() {
                                                        e.field("About me:", cb(bio, ""), false);
                                                    }
                                                }
                                                e.field("Role:", &tetr_usr.get_role_name(), true);
                                                if !tetr_usr.is_gameplay_hidden() {
                                                    e.field(
                                                        "Play time:",
                                                        &tetr_usr.get_gametime(),
                                                        true,
                                                    );
                                                }
                                                e.field(
                                                    "Friends:",
                                                    &tetr_usr.get_friend_count().to_string(),
                                                    true,
                                                );
                                                if tetr_usr.is_rating() {
                                                    e.fields(vec![
                                                        (
                                                            "​",
                                                            format!(
                                                                "[**== TETRA LEAGUE ==**]({})",
                                                                tetr_usr.get_recent_league()
                                                            ),
                                                            false,
                                                        ),
                                                        ("Glicko:", tetr_usr.get_glicko(), true),
                                                        (
                                                            "Play count:",
                                                            tetr_usr
                                                                .get_gamesplayed_league()
                                                                .separate_with_commas(),
                                                            true,
                                                        ),
                                                        (
                                                            "Wins:",
                                                            format!(
                                                                "{} ({:.3}%)",
                                                                tetr_usr.get_gameswon_league().separate_with_commas(),
                                                                (tetr_usr.get_gameswon_league() as f64
                                                                    / tetr_usr.get_gamesplayed_league()
                                                                        as f64
                                                                    * 100.)
                                                            ),
                                                            true,
                                                        ),
                                                        ("APM:", tetr_usr.get_apm().to_string(), true),
                                                        ("PPS:", tetr_usr.get_pps().to_string(), true),
                                                        ("VS:", tetr_usr.get_vs().to_string(), true),
                                                    ]);
                                                }
                                                if records.has_40l_record() {
                                                    e.fields(vec![
                                                        (
                                                            "​",
                                                            format!(
                                                                "[**== 40 LINES ==**]({}) | Achieved <t:{}:R>{}",
                                                                records.get_best_40l_record(),
                                                                records.get_40l_ts(),
                                                                if records.is_40l_top1000() {
                                                                    format!(" | №{}", records.get_40l_rank())
                                                                } else {
                                                                    "".to_string()
                                                                },
                                                            ),
                                                            false,
                                                        ),
                                                        ("Time:", records.get_40l_time(), true),
                                                        ("PPS:", records.get_40l_pps().to_string(), true),
                                                        ("Finesse:", records.get_40l_finesse(), true),
                                                    ]);
                                                }
                                                if records.has_blitz_record() {
                                                    e.fields(vec![
                                                        (
                                                            "​",
                                                            format!(
                                                                "[**== BLITZ ==**]({}) | Achieved <t:{}:R>{}",
                                                                records.get_best_blitz_record(),
                                                                records.get_blitz_ts(),
                                                                if records.is_blitz_top1000() {
                                                                    format!(" | №{}", records.get_blitz_rank())
                                                                } else {
                                                                    "".to_string()
                                                                },
                                                            ),
                                                            false,
                                                        ),
                                                        ("Score:", records.get_blitz_score(), true),
                                                        ("PPS:", records.get_blitz_pps().to_string(), true),
                                                        ("Finesse:", records.get_blitz_finesse(), true),
                                                    ]);
                                                }
                                                e.field(
                                                    "​",
                                                    format!(
                                                        "{} | {}",
                                                        ping2,
                                                        tetr_usr.get_cacherd_at()
                                                    ),
                                                    false,
                                                );
                                                e.footer(|f| f.text(ftr));
                                                e.timestamp(Utc::now());
                                                e.author(|a| {
                                                    if let Some(flag) = tetr_usr.get_national_flag() {
                                                        a.icon_url(flag);
                                                    }
                                                    a.name(&format!(
                                                        "Lv.{} {} {}xp",
                                                        tetr_usr.get_level(),
                                                        tetr_usr.get_level_mark(),
                                                        tetr_usr.get_formatted_xp()
                                                    ))
                                                });
                                                e.color(Colour(tetr_usr.get_col()));
                                                e.thumbnail(tetr_usr.get_face_url());
                                                e
                                            });
                                            m.content("")
                                        })
                                        .await
                                        .expect(&Et::Other("").l(cmd, "MSG EDIT(TETR-USER)"));
                                }
                            }
                            Err(err) => {
                                content
                                    .edit(&ctx, |m| {
                                        m.embed(|e| {
                                            e.title(usr1.to_uppercase());
                                            e.description(format!(
                                                "```\n{}\n```\n{}",
                                                ftg(err),
                                                &ping
                                            ));
                                            e.footer(|f| f.text(ftr));
                                            e.timestamp(Utc::now());
                                            e.color(Colour(EMBED_LABEL_COL))
                                        })
                                        .content("")
                                    })
                                    .await
                                    .expect(&Et::Other("").l(cmd, "MSG EDIT(FAILED TO RESPONSE)"));
                            }
                        }
                    }
                } else {
                    let content = msg.channel_id.say(&ctx.http, missing_arg(1)).await;
                    Et::Rslt(content).l(cmd, "SEND");
                }
            }

            // FOR TRUSTED USER
            if restricting_users(&TRUSTED, msg.author.id) {
                // kgrs!set_activity <type:ACTIVITY-TYPE> <content:str> || Change Kagurin.rs activity
                //  ACTIVITY-TYPE = [playing, listening, watching, competing]
                if cmd == "set_activity" {
                    if let (Some(r#type), Some(content)) = (arg[1], arg[2]) {
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
                        // do not work
                        let reaction = msg
                            .react(
                                &ctx,
                                ReactionType::Custom {
                                    animated: false,
                                    id: EmojiId(856911443390627840),
                                    name: Some(String::from("HAAKU_death")),
                                },
                            )
                            .await;
                        if let Err(why) = reaction {
                            println!("kgrs!{} / REACTION : {:?}", cmd, why);
                        }
                        println!(
                            "{} が botアクティビティを {} の `{}` に変更",
                            msg.author.name, r#type, content
                        );
                    } else {
                        let content = msg.channel_id.say(&ctx.http, &missing_arg(2)).await;
                        Et::Rslt(content).l(cmd, "SEND");
                    }
                }
            }

            // DEV
            if restricting_users(&DEVELIPER, msg.author.id) {
                // kgrs!exit | process exit
                if cmd == "exit" {
                    let content = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            format!("Process exited at <t:{}:T>", Utc::now().timestamp()),
                        )
                        .await;
                    Et::Rslt(content).l(cmd, "SEND");
                    process::exit(0b1110111);
                }

                // kgrs!sd | show serenity-rs doc
                if cmd == "sd" {
                    let content = msg
                        .channel_id
                        .say(&ctx.http, "https://docs.rs/serenity/latest/serenity")
                        .await;
                    Et::Rslt(content).l(cmd, "SEND");
                }

                // kgrs!direct_msg | send dm yo
                if cmd == "direct_msg" {
                    let content = msg.author.dm(&ctx, |m| m.content("Yoooo")).await;
                    Et::Rslt(content).l(cmd, "SEND DM");
                }

                // kgrs!message_builder | MessageBuilder test1
                if cmd == "message_builder" {
                    let channel = msg
                        .channel_id
                        .to_channel(&ctx)
                        .await
                        .expect(&Et::Other("").l(cmd, "GET CHANNEL"));
                    let mb = MessageBuilder::new()
                        .push("贵樣(")
                        .push_bold_safe(&msg.author.name)
                        .push(") ば `kgrs!dynamiccmd` を ")
                        .mention(&channel)
                        .push(" で使用レだ。")
                        .build();
                    let content = msg.channel_id.say(&ctx.http, &mb).await;
                    Et::Rslt(content).l(cmd, "SEND");
                }

                // kgrs!message_builder2 | MessageBuilder test2
                if cmd == "message_builder2" {
                    let emoji = serde_json::from_value::<Emoji>(json!({
                        "animated": false,
                        "id": EmojiId(921759722170904618),
                        "managed": true,
                        "name": "yushyu_no_jinzay".to_string(),
                        "require_colons": true,
                        "roles": Vec::<Role>::new(),
                    }))
                    .unwrap();
                    let mb = MessageBuilder::new()
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
                    //println!("{:?}", mb);
                    let content = msg.channel_id.say(&ctx.http, &mb).await;
                    Et::Rslt(content).l(cmd, "SEND");
                }

                // kgrs!embed_and_img | embed & img test
                if cmd == "embed_and_img" {
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
                                    e.timestamp(Utc::now());

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

                    Et::Rslt(content).l(cmd, "SEND");
                }

                // kgrs!br | show bevy::render docs
                if cmd == "br" {
                    let content = msg
                        .channel_id
                        .say(&ctx.http, "https://docs.rs/bevy/latest/bevy/render")
                        .await;
                    Et::Rslt(content).l(cmd, "SEND");
                }
            }
        }
    }

    // READY event
    async fn ready(&self, ctx: Context, ready: Ready) {
        // now(JST)
        let now = Utc::now() + Duration::hours(9);
        println!(
            "{} v{} is connected: {} (JST)",
            ready.user.name,
            VER,
            now.format("%Y-%m-%d %H:%M:%S.%z")
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
        .application_id(BOT_ID)
        .event_handler(Handler {
            client: reqwest::Client::new(),
        })
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
