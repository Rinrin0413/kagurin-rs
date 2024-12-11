use chrono::{DateTime, Utc};
use cjp::AsCjp;
use cjp_v0::AsCJp as AsCjpV0;
use colored::Colorize;
use kgrs::{
    cmds::{playground, sfinder, tetr::*},
    response_interactions::{self, InteractMode, Interactions},
    util::{fmt::*, *},
};
use lang::dict;
use regex::Regex;
use rogger::*;
use serenity::{
    async_trait,
    builder::{
        CreateActionRow, CreateButton, CreateComponents, CreateEmbed, CreateEmbedFooter,
        CreateInputText,
    },
    http::typing::Typing,
    model::{
        application::{
            component::{ButtonStyle, ComponentType},
            interaction::{Interaction, InteractionResponseType},
        },
        channel::Message,
        gateway::{Activity, Ready},
        prelude::{
            component::{ActionRowComponent, InputTextStyle},
            interaction::application_command::CommandDataOptionValue,
            *,
        },
    },
    prelude::*,
};
use std::{collections::HashMap, env, fs::File, io::Read, process, time::Instant};
use tetr_ch::{model::prelude::*, prelude::*};
use thousands::Separable;

const RUST_VERSION: &str = "1.83.0";
const OS: &str = "openSUSE Leap 15.5 x86_64";

const VER: &str = env!("CARGO_PKG_VERSION");
const MAIN_COL: u32 = 0xB89089;
const INVITE_URL: &str =
    "https://discord.com/api/oauth2/authorize?client_id=936116497502318654&permissions=8&scope=bot";
const GITHUB_EMOJI: ReactionType = ReactionType::Custom {
    animated: false,
    id: EmojiId(1072455141350973471),
    name: None,
};
const TRUSTED: [u64; 3] = [
    724976600873041940, // rinrin0413
    801082943371477022, // rinrin.wgsl
    680687014072287265, // rinloid
];
const DEVELOPERS: [u64; 2] = [
    724976600873041940, // rinrin0413
    801082943371477022, // rinrin.wgsl
];
//const IS_DST: bool = true; // Is daylight saving time(for Sky:CotL)

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Shows about the help command if mentioned
        match msg.mentions_me(&ctx.http).await {
            Ok(b) => {
                if b && msg.kind != MessageType::InlineReply {
                    if let Err(why) = msg
                        .reply(
                            &ctx.http,
                            "\
English: Do you need help? If so, please use </help:1014735729139662898>.\n\
\n\
日本語: ヘルプが必要ですか? もしそうなら、</help:1014735729139662898> でヘルプを参照してください。\
                                    ",
                        )
                        .await
                    {
                        error!("Failed sending message: {:?}", why);
                    }
                }
            }
            Err(why) => {
                error!("Failed to get mentions: {}", why);
            }
        }

        // Unwraps the message link
        if msg.content.contains("https://discord.com/channels/") {
            let re = if let Ok(r) = Regex::new(r"https://discord.com/channels/(\d+)/(\d+)/(\d+)") {
                r
            } else {
                return;
            };
            let (guild_id, channel_id, msg_id) = if let Some(c) = re.captures(&msg.content) {
                let mut vec = Vec::new();
                for i in 1..=3 {
                    if let Some(Ok(v)) = c.get(i).map(|m| m.as_str().parse::<u64>()) {
                        vec.push(v);
                    } else {
                        return;
                    }
                }
                (vec[0], vec[1], vec[2])
            } else {
                return;
            };
            let guild = if let Ok(g) = GuildId(guild_id).to_partial_guild(&ctx.http).await {
                g
            } else {
                return;
            };
            let ch = if let Ok(c) = ChannelId(channel_id).to_channel(&ctx.http).await {
                if let Some(c) = c.guild() {
                    c
                } else {
                    return;
                }
            } else {
                return;
            };
            let that_msg = if let Ok(m) = ctx.http.get_message(channel_id, msg_id).await {
                m
            } else {
                return;
            };
            if ch.is_nsfw()
                || (that_msg.embeds.is_empty()
                    && that_msg.content.is_empty()
                    && that_msg.attachments.is_empty())
            {
                return;
            }
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.author(|a| a.name(
                            format!("{}#{}", 
                                that_msg.author.name, that_msg.author.discriminator
                            )
                        ).icon_url(that_msg.author.face()));
                        e.description(format!(
                            "**[Jump to the message](https://discord.com/channels/{}/{}/{})**\n\n{}",
                            guild_id, channel_id, msg_id, if that_msg.content.is_empty() {
                                format!("||NO CONTENT, BUT HAS {}(S)||", if that_msg.embeds.is_empty() {
                                    "ATTACHMENT".to_string()
                                } else {
                                    "EMBED".to_string()
                                })
                            } else {
                                that_msg.content
                            }
                        ));
                        e.field("Sent date:", format!("<t:{}:R>", that_msg.timestamp.unix_timestamp()), true);
                        if !that_msg.attachments.is_empty() {
                            e.image(&that_msg.attachments[0].url);
                            if 2 <= that_msg.attachments.len() {
                                e.thumbnail(&that_msg.attachments[1].url);
                                e.field("Files:", format!(
                                    "{} attachments",
                                    that_msg.attachments.len()
                                ), true);
                            } else {
                                e.field("Files:", format!(
                                    "{} attachment",
                                    that_msg.attachments.len()
                                ), true);
                            }
                        }
                        if let Some(ts) = that_msg.edited_timestamp {
                            e.field("Last edited at:", format!(
                                "<t:{}:R>", 
                                ts.unix_timestamp()
                            ), true);
                        }
                        e.fields(vec![
                            ("Message Id:" , mn(msg_id), true)
                        ]);
                        e.fields(vec![
                            ("Guild:", guild.name, true),
                            ("Channel:", format!("<#{}>", channel_id), true),
                        ]);
                        e.color(MAIN_COL);
                        e
                    })
                    .components(|c| c.create_action_row(|a| {
                        a.create_button(|b| {
                            b.label("❌")
                                .style(ButtonStyle::Danger)
                                .custom_id("deleteUnwrappedMsg")
                        })
                    }))
                })
                .await
                .expect("Failed to send message");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(interact) => {
                // DB
                let cmd_data = &interact.data;
                let args = &cmd_data.options;
                let cache = &ctx.cache;
                let client = cache.current_user();
                let bot_icon = &client.face();
                let ftr = || {
                    CreateEmbedFooter::default()
                        .text(format!(
                            "/{}{}",
                            interact.data.name,
                            if let Some(m) = interact.member.as_ref() {
                                format!(", Called by {}", m.user.name)
                            } else {
                                String::new()
                            }
                        ))
                        .to_owned()
                };
                let dict_lookup = |dict: &HashMap<String, (String, String)>, key: &str| {
                    let s = if let Some(s) = dict.get(key) {
                        s
                    } else {
                        error!("Invalid dict key: {}", key);
                        panic!();
                    };
                    if interact.locale == "ja" {
                        s.1.clone()
                    } else {
                        s.0.clone()
                    }
                };
                let defer = || async {
                    if let Err(why) = interact.defer(&ctx).await {
                        error!("Error while deferring interaction: {}", why);
                    }
                };

                let mut is_ephemeral = false;

                let content = match interact.data.name.as_str() {
                    // Kill the bot | 1019672344643522580
                    "exit" => {
                        let dict = dict::exit();
                        let authed = DEVELOPERS.contains(interact.user.id.as_u64());

                        if let Err(why) = interact
                            .create_interaction_response(&ctx.http, |response| {
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|m| {
                                        if authed {
                                            m.content(format!(
                                                "Process exited at <t:{}:F>",
                                                Utc::now().timestamp()
                                            ))
                                        } else {
                                            m.content(dict_lookup(&dict, "unauthorized"))
                                                .ephemeral(true)
                                        }
                                    })
                            })
                            .await
                        {
                            error!("Cannot respond to slash command: {}", why);
                        }

                        if authed {
                            info!("Process exited at {}", Utc::now());
                            process::exit(0x00);
                        }

                        Interactions::None
                    }

                    // pong! | 1014243185880465550
                    "ping" => {
                        let before = Instant::now();
                        if let Err(why) = interact
                            .create_interaction_response(&ctx.http, |response| {
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|m| m.content("pong!"))
                            })
                            .await
                        {
                            error!("Cannot respond to slash command: {}", why);
                        }
                        let after = Instant::now();
                        if let Err(why) = interact
                            .edit_original_interaction_response(&ctx.http, |m| {
                                m.content(format!("pong! ({}ms)", (after - before).as_millis()))
                            })
                            .await
                        {
                            error!("Cannot respond to edit message: {}", why);
                        };
                        Interactions::None
                    }

                    // Show command help | 1014735729139662898
                    "help" => {
                        let general_dict = &dict::help_cmd_general();
                        if let Some(k) = args.first() {
                            let arg_val = k.value.as_ref().unwrap().as_str().unwrap();
                            Interactions::Some(vec![InteractMode::Embed(match arg_val {
                                "display" => {
                                    let dict = &dict::help_display();
                                    CreateEmbed::default()
                                        .title(dict_lookup(dict, "title"))
                                        .description(dict_lookup(general_dict, "implSlashCmds"))
                                        .fields(vec![
                                            (
                                                "</ping:1014243185880465550>",
                                                "pong!".to_string(),
                                                false,
                                            ),
                                            (
                                                "</info:1015567292022673449>",
                                                dict_lookup(dict, "info"),
                                                false,
                                            ),
                                            (
                                                "</neofetch:1015944810647011328>",
                                                dict_lookup(dict, "neofetch"),
                                                false,
                                            ),
                                        ])
                                        .set_footer(ftr())
                                        .timestamp(Utc::now().to_rfc3339())
                                        .color(MAIN_COL)
                                        .to_owned()
                                }
                                "util" => {
                                    let dict = &dict::help_util();
                                    CreateEmbed::default()
                                        .title(dict_lookup(dict, "title"))
                                        .description(dict_lookup(general_dict, "implSlashCmds"))
                                        .fields(vec![
                                            (
                                                "</cjp:1021847038545100810> <sentence:string> [version:string]",
                                                dict_lookup(dict, "cjp"),
                                                false,
                                            ),
                                            (
                                                "</now:1040285205874888787>",
                                                dict_lookup(dict, "now"),
                                                false,
                                            ),
                                            (
                                                "</ts:1040293233839845396> <year:integer> <month:integer> <day:integer> [hour:integer] [minute:integer] [second:integer] [millisecond:integer]",
                                                dict_lookup(dict, "ts"),
                                                false,
                                            ),
                                            (
                                                "</rust:1097501737994162228>",
                                                dict_lookup(dict, "rust"),
                                                false,
                                            )
                                        ])
                                        .set_footer(ftr())
                                        .timestamp(Utc::now().to_rfc3339())
                                        .color(MAIN_COL)
                                        .to_owned()
                                }
                                "fun" => {
                                    let dict = &dict::help_fun();
                                    CreateEmbed::default()
                                        .title(dict_lookup(dict, "title"))
                                        .description(dict_lookup(general_dict, "implSlashCmds"))
                                        .fields(vec![(
                                            "/",
                                            "Nothing here yet :(".to_string(),
                                            false,
                                        )])
                                        .set_footer(ftr())
                                        .timestamp(Utc::now().to_rfc3339())
                                        .color(MAIN_COL)
                                        .to_owned()
                                }
                                "tetrio" => {
                                    let dict = &dict::help_tetrio();
                                    CreateEmbed::default()
                                        .title(dict_lookup(dict, "title"))
                                        .description(dict_lookup(general_dict, "implSlashCmds"))
                                        .fields(vec![
                                            (
                                                "</tetr-user:1018530733314289737> <username/user-id:string>",
                                                dict_lookup(dict, "tetr-user"),
                                                false,
                                            ),
                                            (
                                                "</tetr-user-search:1035478275910275093> <user>",
                                                dict_lookup(dict, "tetr-user-search"),
                                                false,
                                            ),
                                        ])
                                        .set_footer(ftr())
                                        .timestamp(Utc::now().to_rfc3339())
                                        .color(MAIN_COL)
                                        .to_owned()
                                }
                                "admin" => {
                                    let dict = &dict::help_admin();
                                    CreateEmbed::default()
                                        .title(dict_lookup(dict, "title"))
                                        .description(dict_lookup(general_dict, "implSlashCmds"))
                                        .fields(vec![(
                                            "/",
                                            "Nothing here yet :(".to_string(),
                                            false,
                                        )])
                                        .set_footer(ftr())
                                        .timestamp(Utc::now().to_rfc3339())
                                        .color(MAIN_COL)
                                        .to_owned()
                                }
                                "dev" => {
                                    let dict = &dict::help_dev();
                                    CreateEmbed::default()
                                        .title(dict_lookup(dict, "title"))
                                        .description(dict_lookup(general_dict, "implSlashCmds"))
                                        .fields(vec![(
                                            "</exit:1019672344643522580>",
                                            dict_lookup(dict, "exit"),
                                            false,
                                        )])
                                        .set_footer(ftr())
                                        .timestamp(Utc::now().to_rfc3339())
                                        .color(MAIN_COL)
                                        .to_owned()
                                }
                                "trust" => {
                                    let dict = &dict::help_trust();
                                    CreateEmbed::default()
                                        .title(dict_lookup(dict, "title"))
                                        .description(dict_lookup(general_dict, "implSlashCmds"))
                                        .fields(vec![(
                                            "</sfinder-path:1072236238574190754> <field:string> [next:string]",
                                            dict_lookup(dict, "sfinder-path"),
                                            false,
                                        )])
                                        .set_footer(ftr())
                                        .timestamp(Utc::now().to_rfc3339())
                                        .color(MAIN_COL)
                                        .to_owned()
                                }
                                _ => unreachable!("Invalid help command: {}", arg_val),
                            })])
                        } else {
                            let dict = &dict::help();
                            Interactions::Some(vec![InteractMode::Embed(
                                CreateEmbed::default()
                                    .title(dict_lookup(dict, "title"))
                                    .description(dict_lookup(general_dict, "implSlashCmds"))
                                    .fields(vec![
                                        (
                                            "</help:1014735729139662898>",
                                            dict_lookup(dict, "help"),
                                            false,
                                        ),
                                        (
                                            "</help display:1014735729139662898>",
                                            dict_lookup(dict, "help.display"),
                                            false,
                                        ),
                                        (
                                            "</help util:1014735729139662898>",
                                            dict_lookup(dict, "help.util"),
                                            false,
                                        ),
                                        (
                                            "</help fun:1014735729139662898>",
                                            dict_lookup(dict, "help.fun"),
                                            false,
                                        ),
                                        (
                                            "</help tetrio:1014735729139662898>",
                                            dict_lookup(dict, "help.tetrio"),
                                            false,
                                        ),
                                        (
                                            "</help admin:1014735729139662898>",
                                            dict_lookup(dict, "help.admin"),
                                            false,
                                        ),
                                        (
                                            "</help dev:1014735729139662898>",
                                            dict_lookup(dict, "help.dev"),
                                            false,
                                        ),
                                        (
                                            "</help trust:1014735729139662898>",
                                            dict_lookup(dict, "help.trust"),
                                            false,
                                        ),
                                    ])
                                    .set_footer(ftr())
                                    .timestamp(Utc::now().to_rfc3339())
                                    .color(MAIN_COL)
                                    .to_owned(),
                            )])
                        }
                    }

                    // Show information about this bot | 1015567292022673449
                    "info" => {
                        let dict = &dict::info();
                        Interactions::Some(vec![
                            InteractMode::Embed(
                                CreateEmbed::default()
                                    .author(|a| {
                                        a.icon_url(bot_icon).name(dict_lookup(dict, "title"))
                                    })
                                    .title(dict_lookup(dict, "nameTitle"))
                                    .description(cb(
                                        format!(
                                            "[0;37m{}[0;0m#{}",
                                            client.name, client.discriminator
                                        ),
                                        "ansi",
                                    ))
                                    .fields(vec![
                                        ("ID:", cb(format!("[0;34m{}", client.id), "ansi"), true),
                                        (
                                            &dict_lookup(dict, "botVer"),
                                            cb(format!("[0;32m{}", VER), "ansi"),
                                            true,
                                        ),
                                        (
                                            &dict_lookup(dict, "createdAt"),
                                            format!(
                                                "<t:{}:R>",
                                                client.id.created_at().unix_timestamp()
                                            ),
                                            true,
                                        ),
                                        (
                                            &dict_lookup(dict, "guildsTitle"),
                                            cb(
                                                format!(
                                                    "[0;36m{}{}",
                                                    if let Ok(g) = client.guilds(&ctx.http).await {
                                                        g.len()
                                                    } else {
                                                        0
                                                    },
                                                    dict_lookup(dict, "guildsTxt")
                                                ),
                                                "ansi",
                                            ),
                                            true,
                                        ),
                                        (
                                            &dict_lookup(dict, "dev"),
                                            cb("@rinrin0413".to_string(), "ansi"),
                                            true,
                                        ),
                                        (
                                            &dict_lookup(dict, "lang"),
                                            cb(format!("Rust {}", RUST_VERSION), "ansi"),
                                            true,
                                        ),
                                        (
                                            &dict_lookup(dict, "lib"),
                                            cb("Serenity-rs v0.11.5".to_string(), "ansi"),
                                            true,
                                        ),
                                        ("OS:", cb(format!("[0;31m{}", OS), "ansi"), true),
                                        ("\u{200B}", "\u{200B}".to_string(), true),
                                        (
                                            &dict_lookup(dict, "memory"),
                                            cb(
                                                format!("{:.1}MiB / 31873MiB", get_memory_usage()),
                                                "",
                                            ),
                                            true,
                                        ),
                                        (&dict_lookup(dict, "uptime"), cb(get_uptime(), ""), true),
                                    ])
                                    .set_footer(ftr())
                                    .timestamp(Utc::now().to_rfc3339())
                                    .color(MAIN_COL)
                                    .to_owned(),
                            ),
                            InteractMode::Button(
                                CreateButton::default()
                                    .label(dict_lookup(dict, "btn.invite"))
                                    .style(ButtonStyle::Link)
                                    .url(INVITE_URL)
                                    .to_owned(),
                            ),
                            InteractMode::Button(
                                CreateButton::default()
                                    .label(dict_lookup(dict, "btn.sourceCode"))
                                    .style(ButtonStyle::Link)
                                    .url(env!("CARGO_PKG_REPOSITORY"))
                                    .emoji(GITHUB_EMOJI)
                                    .to_owned(),
                            ),
                        ])
                    }

                    // Display information about this bot in an aesthetic and visually pleasing way | 1015944810647011328
                    "neofetch" => Interactions::Some(vec![InteractMode::Embed(
                        CreateEmbed::default()
                            .description(format!(
                                r#"
```ansi
[0mkgrs@rinrin:~> neofetch
     [33mRRRRRRRRR         [31mKagurin.rs
 [33m.s*R*RRRRRR*===       [0m---------- 
[33m:sRRRRRRRRRRR*-:-      [34mVersion[0m: {}
 [33m*RRRR*RRRRRRR-:sR     [34mOS[0m: {}
 [33mR***s==ss*****RRR     [34mHost[0m: rinrin0413
  [33ms==s-.::=ss=sRRR     [34mMemory[0m: {:.1}MiB / 31873MiB
  [33m=::s=   :=s---RR     [34mUptime[0m: {}
   [33m- ..    ..:-RRR     [34mLanguage[0m: Rust {}
   [33mR:      .-=sRRR     [34mLibrary[0m: Serenity-rs v0.11.5
  [33mRRRs-....:==sRRR     [34mID[0m: {}
  [33m*R RR=-:::--sRRR     [34mServers[0m: {} guilds
 [33ms**:*s--:::==s***s    [34mCreated at[0m: {}
  [33ms==s=:-- :====ss=    
  [33m=s=s::*: =s==s**s    [0m███[30m███[31m███[32m███[33m███[34m███[35m███[36m███[37m███
  [33m%R=s:=*::ss=*RRR     [0m   [40m   [41m   [42m   [43m   [44m   [45m   [46m   [47m   [0m
```
                            "#,
                                if cfg!(debug_assertions) {
                                    format!("{} (debug)", VER)
                                } else {
                                    VER.to_string()
                                },
                                OS,
                                get_memory_usage(),
                                get_uptime(),
                                RUST_VERSION,
                                client.id,
                                if let Ok(g) = client.guilds(&ctx.http).await {
                                    g.len()
                                } else {
                                    0
                                },
                                client.id.created_at().unix_timestamp()
                            ))
                            .color(MAIN_COL)
                            .to_owned(),
                    )]),

                    // Run Rust code with Rust playground | 1097501737994162228
                    "rust" => {
                        let dict = dict::rust();

                        Interactions::Modal {
                            id: "rustCode".to_string(),
                            title: "Rust playground".to_string(),
                            input_texts: vec![CreateInputText::default()
                                .custom_id("rustCode")
                                .style(InputTextStyle::Paragraph)
                                .label(dict_lookup(&dict, "code"))
                                .placeholder("fn main() {\n    println!(\"Hello, Rust!\");\n}")
                                .to_owned()],
                        }
                    }

                    // Display details of the target TETR.IO user | 1018530733314289737
                    "tetr-user" => {
                        let username = args
                            .first()
                            .unwrap()
                            .value
                            .as_ref()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();

                        if username.is_empty() {
                            let dict = dict::tetr_user();
                            is_ephemeral = true;
                            Interactions::Some(vec![InteractMode::Message(dict_lookup(
                                &dict,
                                "err.plzSendUserNameOrID",
                            ))])
                        } else {
                            let _typing = Typing::start(
                                ctx.http.clone(),
                                interact.channel_id.as_u64().to_owned(),
                            )
                            .expect("Failed to start typing: ");

                            let before = Instant::now();
                            let response = tetr_ch::Client::new().get_user(&username).await;
                            let mut latency = format!("latency: {}ms", (Instant::now() - before).as_millis());

                            match response {
                                Ok(res) => {
                                    if res.is_success {
                                        let user_data = res.data.as_ref().unwrap();

                                        if !user_data.is_banned() {
                                            let mut e = CreateEmbed::default();

                                            let league = tetr_ch::Client::new()
                                                .get_user_league(&user_data.username)
                                                .await
                                                .unwrap()
                                                .data
                                                .unwrap()
                                                .unwrap();

                                            latency = format!("latency: {}ms", (Instant::now() - before).as_millis());

                                            e.title(format!(
                                                "{}  ||{}||",
                                                user_data.username.to_uppercase(),
                                                user_data.id
                                            ));
                                            if user_data.is_supporter {
                                                let supporter_card = format!(
                                                    "**<{:★>st$}>**",
                                                    "SUPPORTER",
                                                    st = (user_data.supporter_tier + 9 - 1) as usize
                                                );
                                                if user_data.is_badstanding {
                                                    e.description(format!(
                                                        "{}\n| **- BAD STANDING -** |",
                                                        supporter_card
                                                    ));
                                                } else {
                                                    e.description(supporter_card);
                                                }
                                            } else if user_data.is_badstanding {
                                                e.description("| **- BAD STANDING -** |");
                                            }
                                            if let Some(rev) = user_data.banner_revision {
                                                if (user_data.is_supporter || user_data.is_admin()) && rev != 0 {
                                                    e.image(format!(
                                                        "https://tetr.io/user-content/banners/{}.jpg?rv={}",
                                                        user_data.id, rev
                                                    ));
                                                }
                                            }
                                            if user_data.has_badge() {
                                                e.field(
                                                    format!("Badges: {}", badge_emojis(user_data)),
                                                    "\u{200B}",
                                                    false,
                                                );
                                            }
                                            let is_rating = 0. <= league.tr;
                                            if is_rating && league.standing.is_some() {
                                                let rank = &league.rank;
                                                e.field(
                                                    format!(
                                                        "〔{} **{:.4}TR** 〕{}",
                                                        rank_emoji(rank),
                                                        league.tr,
                                                        match rank {
                                                            Rank::Z => "".to_string(),
                                                            _ => format!(
                                                                "\n　Global: №{}{}",
                                                                league.standing.unwrap(),
                                                                if let Some(s) = league.standing_local {
                                                                    if s != -1 {
                                                                        format!("\n　Local: №{}", s)
                                                                    } else {
                                                                        String::new()
                                                                    }
                                                                } else {
                                                                    String::new()
                                                                }
                                                            ),
                                                        }
                                                    ),
                                                    match rank {
                                                        Rank::Z => if let Some(r) = &league.percentile_rank {
                                                            format!(
                                                                "Probably around {}",
                                                                rank_emoji(r)
                                                            )
                                                        } else {
                                                            String::new()
                                                        },
                                                        _ => generate_progress_bar(&league).unwrap_or_default(),
                                                    },
                                                    false,
                                                );
                                            } else {
                                                e.field(
                                                    format!(
                                                        "**{}/10** rating games played",
                                                        league.games_played
                                                    ),
                                                    format!(
                                                        "{} rating games won",
                                                        league.games_won
                                                    ),
                                                    false,
                                                );
                                            }
                                            if let Some(bio) = &user_data.bio {
                                                if !bio.is_empty()
                                                    && (user_data.is_supporter || user_data.is_admin())
                                                {
                                                    e.field("About me:", cb(bio, ""), false);
                                                }
                                            }
                                            e.field("Role:", &user_data.role, true);
                                            if 0. <= user_data.play_time {
                                                e.field(
                                                    "Play time:",
                                                    fmt_gametime(user_data.play_time),
                                                    true,
                                                );
                                            }
                                            e.field(
                                                "Friends:",
                                                user_data.friend_count.unwrap_or_default(),
                                                true,
                                            );
                                            if let Some(br) = &league.best_rank {
                                                e.field(
                                                    "Top rank:",
                                                    rank_emoji(br),
                                                    true,
                                                );
                                            }
                                            if let Some(d) = &user_data.connections.discord {
                                                e.field("Discord:", format!("<@!{}>", d.id), true);
                                            }
                                            if is_rating {
                                                e.fields(vec![
                                                    (
                                                        "\u{200B}",
                                                        format!(
                                                            "[**== TETRA LEAGUE ==**](https://ch.tetr.io/s/league_userrecent_{})",
                                                            user_data.id,
                                                        ),
                                                        false,
                                                    ),
                                                    ("Glicko:", format!("{:.3}±{:.3}", league.glicko, league.rd.unwrap()), true),
                                                    (
                                                        "Play count:",
                                                        league.games_played.separate_with_commas(),
                                                        true,
                                                    ),
                                                    (
                                                        "Wins:",
                                                        format!(
                                                            "{} ({:.3}%)",
                                                            league.games_won.separate_with_commas(),
                                                            (league.games_won as f64
                                                                / league.games_played
                                                                    as f64
                                                                * 100.)
                                                        ),
                                                        true,
                                                    ),
                                                    ("APM:", league.apm.unwrap().to_string(), true),
                                                    ("PPS:", league.pps.unwrap().to_string(), true),
                                                    ("VS:", league.vs.unwrap().to_string(), true),
                                                ]);
                                            }
                                            // if let Some(fl) = &record.records.forty_lines.record {
                                            //     let end_ctx = if let EndContext::SinglePlay(ec) =
                                            //         &fl.endcontext
                                            //     {
                                            //         ec
                                            //     } else {
                                            //         unreachable!()
                                            //     };
                                            //     e.fields(vec![
                                            //         (
                                            //             "\u{200B}",
                                            //             format!(
                                            //                 "[**== 40 LINES ==**]({}) | Achieved <t:{}:R>{}",
                                            //                 fl.record_url(),
                                            //                 fl.recorded_at(),
                                            //                 if let Some(r) = record.records.forty_lines.rank {
                                            //                     format!(
                                            //                         " | №{}{}",
                                            //                         r,
                                            //                         if r == 1 {
                                            //                             "\n| 𝟜𝟘 𝐋𝐈𝐍𝐄𝐒 𝐂𝐇𝐀𝐌𝐏𝐈𝐎𝐍 |"
                                            //                         } else {
                                            //                             ""
                                            //                         }
                                            //                     )
                                            //                 } else {
                                            //                     String::new()
                                            //                 },
                                            //             ),
                                            //             false,
                                            //         ),
                                            //         ("Time:", fmt_forty_lines_time(end_ctx.final_time.unwrap()), true),
                                            //         ("PPS:", round_mid(end_ctx.pps(), 2).to_string(), true),
                                            //         ("Finesse:", fmt_finesse(end_ctx), true),
                                            //     ]);
                                            // }
                                            // if let Some(bltz) = record.records.blitz.record {
                                            //     let end_ctx = if let EndContext::SinglePlay(ec) =
                                            //         &bltz.endcontext
                                            //     {
                                            //         ec
                                            //     } else {
                                            //         unreachable!()
                                            //     };
                                            //     e.fields(vec![
                                            //         (
                                            //             "\u{200B}",
                                            //             format!(
                                            //                 "[**== BLITZ ==**]({}) | Achieved <t:{}:R>{}",
                                            //                 bltz.record_url(),
                                            //                 bltz.recorded_at(),
                                            //                 if let Some(r) = record.records.blitz.rank {
                                            //                     format!(
                                            //                         " | №{}{}",
                                            //                         r,
                                            //                         if r == 1 {
                                            //                             "\n| 𝐁𝐋𝐈𝐓𝐙 𝐂𝐇𝐀𝐌𝐏𝐈𝐎𝐍 |"
                                            //                         } else {
                                            //                             ""
                                            //                         }
                                            //                     )
                                            //                 } else {
                                            //                     "".to_string()
                                            //                 },
                                            //             ),
                                            //             false,
                                            //         ),
                                            //         ("Score:", end_ctx.score.unwrap().separate_with_commas(), true),
                                            //         ("PPS:", round_mid(end_ctx.pps(), 2).to_string(), true),
                                            //         ("Finesse:", fmt_finesse(end_ctx), true),
                                            //     ]);
                                            // }
                                            if let Some(m) = &user_data.bot_master {
                                                e.field("This bot is operated by:", m, false);
                                            }
                                            if let Some(c) = res.cache {
                                                e.field(
                                                    "\u{200B}",
                                                    format!(
                                                        "{} | <t:{}:R>",
                                                        latency,
                                                        c.cached_at()
                                                    ),
                                                    false,
                                                );
                                            }
                                            e.timestamp(Utc::now().to_rfc3339());
                                            e.author(|a| {
                                                if let Some(u) = user_data.national_flag_url() {
                                                    a.icon_url(u);
                                                }
                                                let level = user_data.level();
                                                a.name(format!(
                                                    "Lv.{} {} {}xp",
                                                    level,
                                                    level_symbol(level),
                                                    user_data.xp.floor().separate_with_commas()
                                                ))
                                            });
                                            e.color(rank_col(
                                                &league.rank,
                                                &league.percentile_rank,
                                            ));
                                            e.thumbnail(user_data.avatar_url());
                                            e.set_footer(ftr());
                                            Interactions::Some(vec![InteractMode::Embed(e)])
                                        } else {
                                            Interactions::Some(vec![InteractMode::Embed(
                                                CreateEmbed::default()
                                                    .title(format!(
                                                        "{}  ||{}||",
                                                        user_data.username.to_uppercase(),
                                                        user_data.id
                                                    ))
                                                    .description("")
                                                    .thumbnail(
                                                        "https://tetr.io/res/avatar-banned.png",
                                                    )
                                                    .image("https://ch.tetr.io/res/cute.png")
                                                    .field("| **BANNED** |", "\u{200B}", false)
                                                    .footer(|f| {
                                                        f.text(format!(
                                                            "{}\n/tetr-user{}",
                                                            latency,
                                                            if let Some(m) =
                                                                interact.member.as_ref()
                                                            {
                                                                format!(
                                                                    ", Called by {}",
                                                                    m.user.name
                                                                )
                                                            } else {
                                                                String::new()
                                                            }
                                                        ))
                                                    })
                                                    .timestamp(Utc::now().to_rfc3339())
                                                    .color(0xf81c1c)
                                                    .to_owned(),
                                            )])
                                        }
                                    } else {
                                        Interactions::Some(vec![InteractMode::Embed(
                                            CreateEmbed::default()
                                                .title(username.to_uppercase())
                                                .description(format!(
                                                    "{}\n{}",
                                                    cb(res.error.expect("there is no error data").msg.expect("there is no error message"), ""),
                                                    latency
                                                ))
                                                .set_footer(ftr())
                                                .timestamp(Utc::now().to_rfc3339())
                                                .color(MAIN_COL)
                                                .to_owned(),
                                        )])
                                    }
                                }
                                Err(e) => Interactions::Some(vec![InteractMode::Message(
                                    format!("Error: {}", e),
                                )]),
                            }
                        }
                    }

                    // Convert the string to 怪レい日本语(Ayashī Nihongo) | 1021847038545100810
                    "cjp" => {
                        let dict = dict::cjp();
                        let original = args
                            .first()
                            .unwrap()
                            .value
                            .as_ref()
                            .unwrap()
                            .as_str()
                            .unwrap();
                        let cjp_ver = if let Some(v) = args.get(1) {
                            v.value.as_ref().unwrap().as_str().unwrap()
                        } else {
                            "latest"
                        };

                        let correct = match cjp_ver {
                            "v0" => AsCjpV0::cjp(original),
                            _ => AsCjp::cjp(original),
                        };

                        if 1016 < original.chars().count() || 1016 < correct.chars().count() {
                            Interactions::Some(vec![InteractMode::Message(dict_lookup(
                                &dict,
                                "err.strTooLong",
                            ))])
                        } else {
                            Interactions::Some(vec![InteractMode::Embed(
                                CreateEmbed::default()
                                    .title(dict_lookup(&dict, "title"))
                                    .fields([
                                        (dict_lookup(&dict, "input"), cb(original, ""), false),
                                        (dict_lookup(&dict, "output"), cb(correct, ""), false),
                                        ("".to_string(), format!(
                                            "Converted with [cjp.rs](https://github.com/Rinrin0413/cjp-rs) ({})",
                                            match cjp_ver { "v0" => "v0.1.0", _ => "latest" }
                                        ), false),
                                    ])
                                    .set_footer(ftr())
                                    .timestamp(Utc::now().to_rfc3339())
                                    .color(MAIN_COL)
                                    .to_owned(),
                            )])
                        }
                    }

                    // Generate a image with Japanese Stable diffusion | 1078586252393197659
                    /* "jsd" => {
                        let dict = dict::jsd();

                        // Say "Please wait..."
                        if let Err(why) = interact
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|m| {
                                    m.content(dict_lookup(&dict, "plzWait"))
                                })
                            })
                            .await
                        {
                            error!("Cannot respond to slash command: {}", why);
                        }

                        let prompts = args
                            .get(0)
                            .unwrap()
                            .value
                            .as_ref()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();

                        let scale = args.get(1);

                        jsd_interact_to_discord(
                            &ctx,
                            &jsd::Interaction::AppCmd(&interact),
                            prompts,
                            scale,
                            dict,
                        )
                        .await;

                        Interactions::None
                    } */
                    // Search for a TETR.IO account by Discord account | 1035478275910275093
                    "tetr-user-search" => {
                        if let CommandDataOptionValue::User(discord_usr, _) =
                            args.first().unwrap().resolved.as_ref().unwrap()
                        {
                            let dict = dict::tetr_user_search();

                            let _typing = Typing::start(
                                ctx.http.clone(),
                                interact.channel_id.as_u64().to_owned(),
                            )
                            .expect("Failed to start typing: ");
                            let before = Instant::now();

                            let response = tetr_ch::Client::new().search_user(SocialConnection::Discord(discord_usr.id.to_string())).await;

                            let after = Instant::now();

                            match response {
                                Ok(res) => {
                                    if let Some(d) = res.data {
                                        let user_data = d.user.unwrap();
                                        let latency = format!("latency: {}ms", (after - before).as_millis());
                                        let mut e = CreateEmbed::default();
                                        e.description(format!(
                                            "**{}{}**",
                                            discord_usr.mention(),
                                            dict_lookup(&dict, "accountOf"),
                                        ));
                                        e.field(
                                            "\u{200B}",
                                            format!(
                                                "**Name**: {}\n**ID**: `{}`",
                                                user_data.username.to_uppercase(),
                                                user_data.id
                                            ),
                                            false,
                                        );
                                        if let Some(c) = res.cache {
                                            e.field(
                                                "\u{200B}",
                                                format!(
                                                    "{} | <t:{}:R>",
                                                    latency,
                                                    c.cached_at()
                                                ),
                                                false,
                                            );
                                        }
                                        e.timestamp(Utc::now().to_rfc3339());
                                        e.color(MAIN_COL);
                                        e.set_footer(ftr());
                                        Interactions::Some(vec![
                                            InteractMode::Embed(e),
                                            InteractMode::Button(
                                                CreateButton::default()
                                                    .label(dict_lookup(&dict, "btn.label"))
                                                    .style(ButtonStyle::Link)
                                                    .url(format!(
                                                        "https://ch.tetr.io/u/{}",
                                                        user_data.username
                                                    ))
                                                    .to_owned(),
                                            ),
                                        ])
                                    } else {
                                        Interactions::Some(vec![InteractMode::Message(format!(
                                            "{} `{}{}`({}) {}",
                                            dict_lookup(&dict, "err.notFound.0"),
                                            discord_usr.name,
                                            if discord_usr.discriminator == 0 {
                                                String::new()
                                            } else {
                                                format!("#{}", discord_usr.discriminator)
                                            },
                                            discord_usr.id,
                                            dict_lookup(&dict, "err.notFound.1"),
                                        ))])
                                    }
                                }
                                Err(why) => Interactions::Some(vec![InteractMode::Message(
                                    format!("Error: {}", why),
                                )]),
                            }
                        } else {
                            unreachable!();
                        }
                    }

                    // Get current UNIX timestamp | 1040285205874888787
                    "now" => Interactions::Some(vec![InteractMode::Message(
                        Utc::now().timestamp().to_string(),
                    )]),

                    // Get UNIX timestamp of the specified datetime(UTC) | 1040293233839845396
                    "ts" => {
                        let dict = dict::ts();

                        let yea = args[0].value.as_ref().unwrap().to_string();
                        let mon = args[1].value.as_ref().unwrap().to_string();
                        let day = args[2].value.as_ref().unwrap().to_string();

                        let dt = if let Some(hour) = args.get(3) {
                            let hou = hour.value.as_ref().unwrap();
                            if let Some(minute) = args.get(4) {
                                let min = minute.value.as_ref().unwrap();
                                if let Some(second) = args.get(5) {
                                    let sec = second.value.as_ref().unwrap();
                                    if let Some(millisecond) = args.get(6) {
                                        let ms = millisecond.value.as_ref().unwrap();
                                        Ok(format!(
                                            "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}.+{:0>4}",
                                            yea, mon, day, hou, min, sec, ms
                                        ))
                                    } else if second.name == "second" {
                                        Ok(format!(
                                            "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}.+0000",
                                            yea, mon, day, hou, min, sec
                                        ))
                                    } else {
                                        Err("sec")
                                    }
                                } else if hour.name == "minute" {
                                    Ok(format!(
                                        "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:00.+0000",
                                        yea, mon, day, hou, min
                                    ))
                                } else {
                                    Err("min")
                                }
                            } else if hour.name == "hour" {
                                Ok(format!(
                                    "{:0>4}-{:0>2}-{:0>2} {:0>2}:00:00.+0000",
                                    yea, mon, day, hou
                                ))
                            } else {
                                Err("hou")
                            }
                        } else {
                            Ok(format!(
                                "{:0>4}-{:0>2}-{:0>2} 00:00:00.+0000",
                                yea, mon, day
                            ))
                        };

                        Interactions::Some(vec![InteractMode::Message(match dt {
                            Ok(dt) => {
                                if let Ok(dt) =
                                    DateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S.%z")
                                {
                                    dt.timestamp().to_string()
                                } else {
                                    dict_lookup(&dict, "err.invalid")
                                }
                            }
                            Err(err) => dict_lookup(&dict, &format!("err.{}", err)),
                        })])
                    }

                    // Get the solution to Perfect Clear from a specified field with solution-finder | 1072236238574190754
                    "sfinder-path" => {
                        let dict = dict::sfinder();
                        let authed = TRUSTED.contains(interact.user.id.as_u64());

                        if authed {
                            sfinder::init_output_dir();
                            defer().await;

                            let stdout = process::Command::new("sh")
                                .args([
                                    format!(
                                        "{}/../assets/shells/sfinder.sh",
                                        env!("CARGO_MANIFEST_DIR")
                                    ),
                                    "path".to_string(),
                                    // Tetfu
                                    format!("-t {}", args[0].value.as_ref().unwrap()),
                                    // Patterns
                                    format!(
                                        "-p {}",
                                        if let Some(cdo) = args.get(1) {
                                            cdo.value.as_ref().unwrap().to_string()
                                        } else {
                                            "*!".to_string()
                                        }
                                    ),
                                ])
                                .output()
                                .unwrap()
                                .stdout;
                            let stdout = String::from_utf8_lossy(&stdout);
                            let is_unique_path_exists = !stdout.contains("Found path [unique] = 0");
                            let is_minimal_path_exists =
                                !stdout.contains("Found path [minimal] = 0");

                            let mut e = CreateEmbed::default();
                            e.title("solution-finder (path)");
                            e.color(MAIN_COL);

                            if !stdout.is_empty() {
                                e.field(dict_lookup(&dict, "output"), cb(stdout, "bash"), false);
                            }

                            let output_dir = sfinder::output_dir();

                            if let Ok(mut f) = File::open(format!("{}/error.txt", output_dir)) {
                                let mut stderr = String::new();
                                if let Err(why) = f.read_to_string(&mut stderr) {
                                    stderr = cb(why.to_string(), "bash");
                                    error!("Failed to read /output/error.txt:\n{}", why);
                                } else if let Some(i) = stderr.rfind("\n\n\n") {
                                    // Delete the Stack trace.
                                    stderr = stderr[..i].to_string();
                                }
                                e.field(dict_lookup(&dict, "error"), cb(stderr, "bash"), false);
                            }

                            let mut interactions = vec![InteractMode::Embed(e)];

                            // Set the link buttons to the fumen.zui.jp with.
                            for pattern in ["unique", "minimal"] {
                                if (pattern == "unique" && is_unique_path_exists)
                                    || (pattern == "minimal" && is_minimal_path_exists)
                                {
                                    if let Ok(mut f) =
                                        File::open(format!("{}/path_{}.html", output_dir, pattern))
                                    {
                                        let mut mu = String::new();
                                        if let Err(why) = f.read_to_string(&mut mu) {
                                            error!(
                                                "Failed to read /output/path_{}.html: {}",
                                                pattern, why
                                            );
                                        } else {
                                            let url = Regex::new(r"href='(.+?)'")
                                                .unwrap()
                                                .captures(&mu)
                                                .unwrap()
                                                .get(1)
                                                .unwrap()
                                                .as_str()
                                                .replace("v115@", "D115@");

                                            // Use TinyURL API
                                            match reqwest::Client::new()
                                                .post("http://tinyurl.com/api-create.php")
                                                .form(&[("url", url)])
                                                .send()
                                                .await
                                            {
                                                Ok(tinyurl_res) => {
                                                    match tinyurl_res.text().await {
                                                        Ok(tinyurl) => {
                                                            // Add the button.
                                                            interactions.push(
                                                                InteractMode::Button(
                                                                    CreateButton::default()
                                                                        .label(dict_lookup(
                                                                            &dict,
                                                                            &format!(
                                                                                "patterns.{}",
                                                                                pattern
                                                                            ),
                                                                        ))
                                                                        .style(ButtonStyle::Link)
                                                                        .url(tinyurl)
                                                                        .to_owned(),
                                                                ),
                                                            );
                                                        }
                                                        Err(why) => {
                                                            error!("Failed to read TinyURL API response: {}", why);
                                                        }
                                                    }
                                                }
                                                Err(why) => {
                                                    error!("Failed to get TinyURL: {}", why);
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Set the link button to solution-finder (GitHub).
                            interactions.push(InteractMode::Button(
                                CreateButton::default()
                                    .label("solution-finder (GitHub)")
                                    .style(ButtonStyle::Link)
                                    .url("https://github.com/knewjade/solution-finder")
                                    .emoji(GITHUB_EMOJI)
                                    .to_owned(),
                            ));

                            sfinder::init_output_dir();
                            Interactions::Edit(interactions)
                        } else {
                            Interactions::Some(vec![InteractMode::Message(dict_lookup(
                                &dict,
                                "unauthorized",
                            ))])
                        }
                    }

                    _ => Interactions::Some(vec![InteractMode::Message(
                        "\
                    not implemented yet :<\n\
                    <@!724976600873041940><@!724976600873041940>\
                    <@!724976600873041940><@!724976600873041940>\
                    <@!724976600873041940><@!724976600873041940>\
                    "
                        .to_string(),
                    )]),
                };

                let is_edit = content.is_edit();
                match content {
                    Interactions::Some(im) | Interactions::Edit(im) => {
                        let response = if is_edit {
                            if let Err(e) = interact
                                .edit_original_interaction_response(&ctx.http, |res| {
                                    let mut action_row = CreateActionRow::default();
                                    for i in im {
                                        match i {
                                            InteractMode::Message(c) => {
                                                res.content(c);
                                            }
                                            InteractMode::Attach(_) => {
                                                /*res.add_file(a);*/
                                                warn!(
                                                    "Cannot attach files at edit (cmd: {})",
                                                    interact.data.name
                                                );
                                            }
                                            InteractMode::Embed(e) => {
                                                res.add_embed(e);
                                            }
                                            InteractMode::Button(b) => {
                                                action_row.add_button(b);
                                            }
                                        }
                                    }
                                    /*if is_ephemeral {
                                        res.ephemeral(true);
                                        is_ephemeral = false;
                                    }*/
                                    if action_row.0.is_empty() {
                                        res
                                    } else {
                                        res.components(|c| c.set_action_row(action_row))
                                    }
                                })
                                .await
                            {
                                Result::Err(e)
                            } else {
                                Result::Ok(())
                            }
                        } else {
                            interact
                                .create_interaction_response(&ctx.http, |res| {
                                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                                        .interaction_response_data(|m| {
                                            let mut action_row = CreateActionRow::default();
                                            for i in im {
                                                match i {
                                                    InteractMode::Message(c) => {
                                                        m.content(c);
                                                    }
                                                    InteractMode::Attach(a) => {
                                                        m.add_file(a);
                                                    }
                                                    InteractMode::Embed(e) => {
                                                        m.add_embed(e);
                                                    }
                                                    InteractMode::Button(b) => {
                                                        action_row.add_button(b);
                                                    }
                                                }
                                            }
                                            if is_ephemeral {
                                                m.ephemeral(true);
                                                is_ephemeral = false;
                                            }
                                            if action_row.0.is_empty() {
                                                m
                                            } else {
                                                m.set_components(
                                                    CreateComponents::default()
                                                        .set_action_row(action_row)
                                                        .to_owned(),
                                                )
                                            }
                                        })
                                })
                                .await
                        };
                        if let Err(err) = response {
                            response_interactions::handle_err(
                                is_edit,
                                err,
                                &interact,
                                ctx,
                                dict::btml(),
                            )
                            .await;
                        }
                    }
                    Interactions::Modal {
                        id,
                        title,
                        input_texts,
                    } => {
                        if let Err(err) = interact
                            .create_interaction_response(&ctx.http, |res| {
                                res.kind(InteractionResponseType::Modal)
                                    .interaction_response_data(|m| {
                                        let mut action_row = CreateActionRow::default();
                                        for input_text in input_texts {
                                            action_row.add_input_text(input_text);
                                        }
                                        m.custom_id(id).title(title).set_components(
                                            CreateComponents::default()
                                                .set_action_row(action_row)
                                                .to_owned(),
                                        )
                                    })
                            })
                            .await
                        {
                            response_interactions::handle_err(
                                is_edit,
                                err,
                                &interact,
                                ctx,
                                dict::btml(),
                            )
                            .await;
                        };
                    }
                    Interactions::Dev => {
                        if let Err(why) = interact
                            .create_interaction_response(&ctx.http, |response| {
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|message| {
                                        message.content("Not implemented yet :<")
                                    })
                            })
                            .await
                        {
                            error!("Cannot respond to slash command: {}", why);
                        } else {
                            warn!("Unimplemented slash command: {}", interact.data.name);
                        }
                    }
                    Interactions::None => {}
                }
            }
            Interaction::MessageComponent(msg_cmp) => {
                /* let dict_lookup = |dict: &HashMap<String, (String, String)>, key: &str| {
                    let s = if let Some(s) = dict.get(key) {
                        s
                    } else {
                        error!("Invalid dict key: {}", key);
                        panic!();
                    };
                    if msg_cmp.locale == "ja" {
                        s.1.clone()
                    } else {
                        s.0.clone()
                    }
                };
                let defer = || async {
                    if let Err(why) = msg_cmp.defer(&ctx).await {
                        error!("Error while deferring interaction: {}", why);
                    }
                }; */

                #[allow(clippy::single_match)]
                match msg_cmp.data.component_type {
                    ComponentType::Button => match msg_cmp.data.custom_id.as_ref() {
                        "deleteUnwrappedMsg" => {
                            if let Err(why) = msg_cmp.message.delete(&ctx.http).await {
                                error!("Failed deleting message: {}", why);
                            }
                        }
                        /* "jsdRetry" => {
                            defer().await;

                            let dict = dict::jsd_retry();
                            let cmd_dict = dict::jsd();

                            if let Err(why) = msg_cmp
                                .clone()
                                .message
                                .edit(&ctx.http, |m| {
                                    m.content(dict_lookup(&dict, "retrying")).components(|c| {
                                        c.create_action_row(|a| {
                                            a.create_button(|b| {
                                                b.label(dict_lookup(&cmd_dict, "btn.retry"))
                                                    .style(ButtonStyle::Primary)
                                                    .custom_id("DISABLED")
                                                    .disabled(true)
                                            })
                                        })
                                    })
                                })
                                .await
                            {
                                error!("Failed deleting message: {}", why);
                            }

                            let prompts = Regex::new(r": (.*) \|")
                                .unwrap()
                                .captures(&msg_cmp.message.content)
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .as_str()
                                .to_string();

                            let scale = None; // TODO: Get scale from message...

                            jsd_interact_to_discord(
                                &ctx,
                                &jsd::Interaction::MsgCmp(msg_cmp),
                                prompts,
                                scale,
                                cmd_dict,
                            )
                            .await;
                        } */
                        _ => (),
                    },
                    _ => (),
                }
            }
            Interaction::ModalSubmit(modal_submit) => {
                let dict_lookup = |dict: &HashMap<String, (String, String)>, key: &str| {
                    let s = if let Some(s) = dict.get(key) {
                        s
                    } else {
                        error!("Invalid dict key: {}", key);
                        panic!();
                    };
                    if modal_submit.locale == "ja" {
                        s.1.clone()
                    } else {
                        s.0.clone()
                    }
                };

                #[allow(clippy::single_match)]
                match modal_submit.data.custom_id.as_ref() {
                    "rustCode" => {
                        let dict = dict::rust();

                        if let ActionRowComponent::InputText(it) =
                            &modal_submit.data.components[0].components[0]
                        {
                            let code = &it.value;

                            if let Err(why) = modal_submit
                                .create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(InteractionResponseType::ChannelMessageWithSource)
                                        .interaction_response_data(|m| {
                                            m.content(format!(
                                                "{}\n```rs\n{}\n```",
                                                dict_lookup(&dict, "code"),
                                                if 3984 < code.len() {
                                                    code.chars().take(3984).collect::<String>()
                                                } else {
                                                    code.to_string()
                                                }
                                            ))
                                        })
                                })
                                .await
                            {
                                error!("Cannot respond to input text: {}", why);
                            }

                            let channel_id = modal_submit.channel_id;

                            let post_data = playground::PostData {
                                code: code.to_string(),
                                crate_type: "bin".to_string(),
                                mode: "debug".to_string(),
                                channel: "stable".to_string(),
                                edition: "2021".to_string(),
                                backtrace: false,
                                tests: false,
                            };
                            let client = reqwest::Client::new()
                                .post("https://play.rust-lang.org/execute")
                                .json(&post_data);

                            let playground_error = || async {
                                if let Err(why) = channel_id
                                    .send_message(&ctx.http, |m| {
                                        m.embed(|e| {
                                            e.title(dict_lookup(&dict, "err.playground"))
                                                .color(MAIN_COL)
                                        })
                                    })
                                    .await
                                {
                                    error!("Failed to send message: {}", why);
                                }
                            };

                            match client.send().await {
                                Ok(response) => match response.json::<playground::Response>().await
                                {
                                    Ok(res) => {
                                        let mut stderr = res.stderr;
                                        if 1016 < stderr.len() {
                                            stderr = stderr.chars().take(1013).collect();
                                            stderr.push_str("...");
                                        }
                                        let mut stdout = res.stdout;
                                        if 1016 < stdout.len() {
                                            stdout = stdout.chars().take(1013).collect();
                                            stdout.push_str("...");
                                        }
                                        if let Err(why) = channel_id
                                            .send_message(&ctx.http, |m| {
                                                m.embed(|e| {
                                                    e.fields(vec![
                                                        (
                                                            dict_lookup(&dict, "stderr"),
                                                            format!("```\n{}\n```", stderr),
                                                            false,
                                                        ),
                                                        (
                                                            dict_lookup(&dict, "stdout"),
                                                            format!("```\n{}\n```", stdout),
                                                            false,
                                                        ),
                                                    ])
                                                    .color(MAIN_COL)
                                                })
                                            })
                                            .await
                                        {
                                            error!("Failed to send message: {}", why);
                                        }
                                    }
                                    Err(why) => {
                                        error!("Cannot parse response: {}", why);
                                        playground_error().await;
                                    }
                                },
                                Err(why) => {
                                    error!("Error was occurred on Rust Playground: {}", why);
                                    playground_error().await;
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        info!("Kagurin-rs v{} is connected.", VER);

        // Set activity.
        ctx.set_activity(Activity::listening("/help")).await;

        // Management of the commands.
        // If you want to run in command management mode,
        // run command line arguments as "cmdmng":
        // $ cargo run cmdmng
        #[allow(unused)]
        if if let Some(arg1) = env::args().nth(1) {
            arg1 == "cmdmng"
        } else {
            false
        } {
            use kgrs::cmd_mng::{cmd_list, CmdManager};
            use serenity::model::application::command::{CommandOptionType, CommandType};

            println!(
                "{}",
                "In command management mode ⚙".white().on_black().bold()
            );
            cmd_list(&ctx.http).await;

            // Main manager.
            // ! WARNING: If manage multiple commands at once, Clone the variable `cmd`.
            // !          Recommend always cloning to avoid mistakes.
            let cmd = serenity::builder::CreateApplicationCommand::default();
            CmdManager::new().run(&ctx.http).await;
        }
    }
}

mod lang;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = match env::var("KAGURIN_RS_TOKEN") {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to get token from environment: {}", e);
            return;
        }
    };

    // Set gateway intents.
    let mut intents = GatewayIntents::default();
    intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents.insert(GatewayIntents::GUILD_MESSAGES);

    // Build client.
    let mut client = match serenity::Client::builder(token, intents)
        .event_handler(Handler)
        .await
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build client: {}", e);
            return;
        }
    };

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
