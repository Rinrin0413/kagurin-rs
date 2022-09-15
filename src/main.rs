use chrono::{Duration, Utc};
use colored::*;
use kgrs::{
    response_interactions::{InteractMode, Interactions},
    tetr::*,
    //playground
    util::{fmt::*, *},
};
use lang::dict;
use serenity::{
    async_trait,
    builder::{CreateActionRow, CreateButton, CreateComponents, CreateEmbed, CreateEmbedFooter},
    http::typing::Typing,
    model::{
        application::{
            component::ButtonStyle,
            interaction::{Interaction, InteractionResponseType},
        },
        channel::Message,
        gateway::{Activity, Ready},
    },
    prelude::*,
};
use std::{collections::HashMap, env, process, time::Instant};
use tetr_ch::{client::Client as TetrClient, model::league::Rank};
use thousands::Separable;

const RUST_VERSION: &str = "1.64.0-nightly";
const OS: &str = "openSUSE Leap 15.4 x86_64";

const VER: &str = env!("CARGO_PKG_VERSION");
const MAIN_COL: u32 = 0xB89089;
const INVITE_URL: &str =
    "https://discord.com/api/oauth2/authorize?client_id=936116497502318654&permissions=8&scope=bot";
//const TRUSTED: [u64; 2] = [
//    724976600873041940, // Rinrin.rs
//    801082943371477022, // Rinrin.wgsl
//];
const DEVELOPER: [u64; 2] = [
    724976600873041940, // Rinrin.rs
    801082943371477022, // Rinrin.wgsl
];
//const IS_DST: bool = true; // Is daylight saving time(for Sky:CotL)

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.mentions_me(&ctx.http).await {
            Ok(b) => {
                if b {
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
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
            Err(why) => {
                println!("Failed to get mentions: {}", why);
            }
        };
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(interact) = interaction {
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
                let s = dict.get(key).unwrap();
                if interact.locale == "ja" {
                    s.1.clone()
                } else {
                    s.0.clone()
                }
            };

            let content = match interact.data.name.as_str() {
                // exit | 1019672344643522580
                "exit" => {
                    let dict = dict::exit();
                    let authed = DEVELOPER.contains(interact.user.id.as_u64());
                    let msg = if authed {
                        format!("Process exited at <t:{}:F>", Utc::now().timestamp())
                    } else {
                        dict_lookup(&dict, "unauthorized")
                    };

                    if let Err(why) = interact
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| m.content(msg))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }

                    if authed {
                        println!("Process exited at {}", Utc::now());
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
                        println!("Cannot respond to slash command: {}", why);
                    }
                    let after = Instant::now();
                    if let Err(why) = interact
                        .edit_original_interaction_response(&ctx.http, |m| {
                            m.content(format!("pong! ({}ms)", (after - before).as_millis()))
                        })
                        .await
                    {
                        println!("Cannot respond to edit message:: {}", why);
                    };
                    Interactions::None
                }

                // Show command help | 1014735729139662898
                "help" => {
                    let general_dict = &dict::help_cmd_general();
                    if let Some(k) = args.get(0) {
                        let arg_val = k.value.as_ref().unwrap().as_str().unwrap();
                        Interactions::Some(vec![InteractMode::Embed(match arg_val {
                            "display" => {
                                let dict = &dict::help_display();
                                CreateEmbed::default()
                                    .title(dict_lookup(dict, "title"))
                                    .description(dict_lookup(general_dict, "implSlashCmds"))
                                    .fields(vec![
                                        ("</ping:1014243185880465550>", "pong!".to_string(), false),
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
                                    .fields(vec![("/", "Nothing here yet :(".to_string(), false)])
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
                                    .fields(vec![("/", "Nothing here yet :(".to_string(), false)])
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
                                    .fields(vec![(
                                        "</tetr-user:1018530733314289737> <user:name/id>",
                                        dict_lookup(dict, "tetr-user"),
                                        false,
                                    )])
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
                                    .fields(vec![("/", "Nothing here yet :(".to_string(), false)])
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
                                    .fields(vec![("/", "Nothing here yet :(".to_string(), false)])
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
                                .author(|a| a.icon_url(bot_icon).name(dict_lookup(dict, "title")))
                                .title(dict_lookup(dict, "nameTitle"))
                                .description(format!(
                                    "```ansi\n[0;37m{}[0;0m#{}\n```",
                                    client.name, client.discriminator
                                ))
                                .fields(vec![
                                    ("ID:", format!("```ansi\n[0;34m{}\n```", client.id), true),
                                    (
                                        &dict_lookup(dict, "botVer"),
                                        format!("```ansi\n[0;32m{}\n```", VER),
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
                                        format!(
                                            "```ansi\n[0;36m{}{}\n```",
                                            if let Ok(g) = client.guilds(&ctx.http).await {
                                                g.len()
                                            } else {
                                                0
                                            },
                                            dict_lookup(dict, "guildsTxt")
                                        ),
                                        true,
                                    ),
                                    (
                                        &dict_lookup(dict, "dev"),
                                        "```ansi\n[0;0m@Rinrin.rs[0;30m#5671\n```".to_string(),
                                        true,
                                    ),
                                    (
                                        &dict_lookup(dict, "lang"),
                                        format!("```ansi\n[0;33mRust {}\n```", RUST_VERSION),
                                        true,
                                    ),
                                    (
                                        &dict_lookup(dict, "lib"),
                                        "```ansi\n[0;35mSerenity-rs v0.11.5```".to_string(),
                                        true,
                                    ),
                                    ("OS:", format!("```ansi\n[0;31m{}\n```", OS), true),
                                    ("\u{200B}", "\u{200B}".to_string(), true),
                                    (
                                        &dict_lookup(dict, "memory"),
                                        format!(
                                            "```\n{:.1}MiB / 31873MiB\n```",
                                            get_memory_usage()
                                        ),
                                        true,
                                    ),
                                    (
                                        &dict_lookup(dict, "uptime"),
                                        format!("```\n{}\n```", get_uptime()),
                                        true,
                                    ),
                                ])
                                .set_footer(ftr())
                                .timestamp(Utc::now().to_rfc3339())
                                .color(MAIN_COL)
                                .to_owned(),
                        ),
                        InteractMode::Button(
                            CreateButton::default()
                                .label("Invile me!")
                                .style(ButtonStyle::Link)
                                .url(INVITE_URL)
                                .to_owned(),
                        ),
                        InteractMode::Button(
                            CreateButton::default()
                                .label("Source code(GitHub)")
                                .style(ButtonStyle::Link)
                                .url(env!("CARGO_PKG_REPOSITORY"))
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
[33m:sRRRRRRRRRRR*-:-      [31mVersion[0m: {}
 [33m*RRRR*RRRRRRR-:sR     [31mOS[0m: {}
 [33mR***s==ss*****RRR     [31mHost[0m: Rinrin.rs#5671
  [33ms==s-.::=ss=sRRR     [31mMemory[0m: {:.1}MiB / 31873MiB
  [33m=::s=   :=s---RR     [31mUptime[0m: {}
   [33m- ..    ..:-RRR     [31mLanguage[0m: Rust {}
   [33mR:      .-=sRRR     [31mLibrary[0m: Serenity-rs v0.11.5
  [33mRRRs-....:==sRRR     [31mID[0m: {}
  [33m*R RR=-:::--sRRR     [31mServers[0m: {} guilds
 [33ms**:*s--:::==s***s    [31mCreated at[0m: {}
  [33ms==s=:-- :====ss=    
  [33m=s=s::*: =s==s**s    [0m███[30m███[31m███[32m███[33m███[34m███[35m███[36m███[37m███
  [33m%R=s:=*::ss=*RRR     [0m   [40m   [41m   [42m   [43m   [44m   [45m   [46m   [47m   [0m
```
                            "#,
                            VER,
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

                // Run Rust code with Rust playground | 1018021689793196142
                "rust" => {
                    /*if let Err(why) = interact
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| m.content("Running..."))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                    let code = args.get(0).unwrap().value.as_ref().unwrap().to_string();
                    let post_data = playground::PostData {
                        code,
                        crate_type: "bin".to_string(),
                        mode: "debug".to_string(),
                        channel: "stable".to_string(),
                        edition: "2021".to_string(),
                        backtrace: false,
                        tests: false,
                    };
                    let client = reqwest::Client::new().post("https://play.rust-lang.org/execute").json(
                        &post_data
                    );
                    match client.send().await {
                        Ok(res) => {
                            let res = res.json::<playground::Response>().await.unwrap();
                            if let Err(why) = interact
                                .edit_original_interaction_response(&ctx.http, |e| {
                                    e.content(format!(
                                        "```\n{:?}\n```",
                                        res.stderr
                                    ))
                                })
                                .await
                            {
                                println!("Cannot respond to edit message: {}", why);
                            }
                        }
                        Err(why) => {
                            println!("Request failed: {}", why);
                        }
                    }
                    Interactions::None*/
                    Interactions::Dev
                }

                // Display details of the target TETR.IO user | 1018530733314289737
                "tetr-user" => {
                    let mut user = args
                        .get(0)
                        .unwrap()
                        .value
                        .as_ref()
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    // Anti "Cannot GET" error
                    user.retain(|c| c != '#');
                    user.retain(|c| c != '\\');
                    user.retain(|c| c != '.');
                    user.retain(|c| c != '/');
                    user.retain(|c| c != '?');
                    user.retain(|c| c != ' ');

                    // Anti "500 error"
                    user.retain(|c| c != '%');

                    if user.is_empty() {
                        let dict = dict::tetr_user();
                        Interactions::Some(vec![InteractMode::Message(dict_lookup(
                            &dict,
                            "err.plzSendUserNameOrID",
                        ))])
                    } else if &user.to_lowercase() != "syabetarou" {
                        let _typing = Typing::start(
                            ctx.http.clone(),
                            interact.channel_id.as_u64().to_owned(),
                        )
                        .expect("Failed to start typing: ");
                        let before = Instant::now();

                        let response = TetrClient::new().get_user(&user).await;

                        let after = Instant::now();
                        let latency = || format!("latency: {}ms", (after - before).as_millis());

                        match response {
                            Ok(res) => {
                                if res.is_success {
                                    let record = TetrClient::new()
                                        .get_user_records(&user)
                                        .await
                                        .unwrap()
                                        .data
                                        .unwrap();

                                    #[allow(unused_variables)]
                                    let after = Instant::now();

                                    let usr = &res.data.as_ref().unwrap().user;
                                    if !usr.is_banned() {
                                        let mut e = CreateEmbed::default();
                                        e.title(format!(
                                            "{}{}  ||{}||",
                                            usr.name.to_uppercase(),
                                            if usr.is_verified { " ✓" } else { "" },
                                            usr.id
                                        ));
                                        if usr.is_supporter() {
                                            let supporter_card = format!(
                                                "**<{:★>st$}>**",
                                                "SUPPORTER",
                                                st = (usr.supporter_tier + 9 - 1) as usize
                                            );
                                            if usr.is_badstanding() {
                                                e.description(format!(
                                                    "{}\n| **- BAD STANDING -** |",
                                                    supporter_card
                                                ));
                                            } else {
                                                e.description(supporter_card);
                                            }
                                        } else if usr.is_badstanding() {
                                            e.description("| **- BAD STANDING -** |");
                                        }
                                        if usr.is_supporter() || usr.is_admin() {
                                            if let Some(url) = usr.banner() {
                                                e.image(url);
                                            }
                                        }
                                        if !usr.has_badge() {
                                            e.field(
                                                format!("Badges: {}", badge_emojis(usr)),
                                                "\u{200B}",
                                                false,
                                            );
                                        }
                                        let is_rating = 0. <= usr.league.rating;
                                        if is_rating {
                                            let rank = usr.league.rank.as_ref();
                                            e.field(
                                                format!(
                                                    "〔{} **{:.4}TR** 〕{}",
                                                    rank_emoji(&rank.to_string()),
                                                    usr.league.rating,
                                                    match rank {
                                                        Rank::Z => "".to_string(),
                                                        _ => format!(
                                                            "\n　Global: №{}\n　Local: №{}",
                                                            usr.league.standing,
                                                            usr.league.standing_local
                                                        ),
                                                    }
                                                ),
                                                match rank {
                                                    Rank::Z => format!(
                                                        "Probably around {}",
                                                        rank_emoji(
                                                            &usr.league.percentile_rank.to_string()
                                                        )
                                                    ),
                                                    _ => create_progress_bar(usr),
                                                },
                                                false,
                                            );
                                        } else {
                                            e.field(
                                                format!(
                                                    "**{}/10** rating games played",
                                                    usr.league.play_count
                                                ),
                                                format!(
                                                    "{} rating games won",
                                                    usr.league.win_count
                                                ),
                                                false,
                                            );
                                        }
                                        if let Some(bio) = &usr.bio {
                                            if !bio.is_empty()
                                                && (usr.is_supporter() || usr.is_admin())
                                            {
                                                e.field("About me:", cb(bio, ""), false);
                                            }
                                        }
                                        e.field("Role:", &usr.role.to_string(), true);
                                        if 0. <= usr.play_time {
                                            e.field(
                                                "Play time:",
                                                fmt_gametime(usr.play_time),
                                                true,
                                            );
                                        }
                                        e.field(
                                            "Friends:",
                                            if let Some(fc) = usr.friend_count {
                                                fc
                                            } else {
                                                0
                                            },
                                            true,
                                        );
                                        if is_rating {
                                            e.fields(vec![
                                                (
                                                    "\u{200B}",
                                                    format!(
                                                        "[**== TETRA LEAGUE ==**](https://ch.tetr.io/s/league_userrecent_{})",
                                                        usr.id,
                                                    ),
                                                    false,
                                                ),
                                                ("Glicko:", format!("{:.3}±{:.3}", usr.league.glicko.unwrap(), usr.league.rd.unwrap()), true),
                                                (
                                                    "Play count:",
                                                    usr.league.play_count.separate_with_commas(),
                                                    true,
                                                ),
                                                (
                                                    "Wins:",
                                                    format!(
                                                        "{} ({:.3}%)",
                                                        usr.league.win_count.separate_with_commas(),
                                                        (usr.league.win_count as f64
                                                            / usr.league.play_count
                                                                as f64
                                                            * 100.)
                                                    ),
                                                    true,
                                                ),
                                                ("APM:", usr.league.apm.unwrap().to_string(), true),
                                                ("PPS:", usr.league.pps.unwrap().to_string(), true),
                                                ("VS:", usr.league.vs.unwrap().to_string(), true),
                                            ]);
                                        }
                                        if let Some(fl) = record.records.forty_lines.record {
                                            e.fields(vec![
                                                (
                                                    "\u{200B}",
                                                    format!(
                                                        "[**== 40 LINES ==**]({}) | Achieved <t:{}:R>{}",
                                                        fl.record_url(),
                                                        fl.recorded_at(),
                                                        if let Some(r) = record.records.forty_lines.rank {
                                                            format!(" | №{}", r)
                                                        } else {
                                                            "".to_string()
                                                        },
                                                    ),
                                                    false,
                                                ),
                                                ("Time:", fmt_forty_lines_time(fl.endcontext.final_time.unwrap()), true),
                                                ("PPS:", round_mid(fl.pps(), 2).to_string(), true),
                                                ("Finesse:", fmt_finesse(fl), true),
                                            ]);
                                        }
                                        if let Some(bltz) = record.records.blitz.record {
                                            e.fields(vec![
                                                (
                                                    "\u{200B}",
                                                    format!(
                                                        "[**== BLITZ ==**]({}) | Achieved <t:{}:R>{}",
                                                        bltz.record_url(),
                                                        bltz.recorded_at(),
                                                        if let Some(r) = record.records.blitz.rank {
                                                            format!(" | №{}", r)
                                                        } else {
                                                            "".to_string()
                                                        },
                                                    ),
                                                    false,
                                                ),
                                                ("Score:", bltz.endcontext.score.unwrap().separate_with_commas(), true),
                                                ("PPS:", round_mid(bltz.pps(), 2).to_string(), true),
                                                ("Finesse:", fmt_finesse(bltz), true),
                                            ]);
                                        }
                                        e.field(
                                            "\u{200B}",
                                            format!("{} | <t:{}:R>", latency(), res.cached_at()),
                                            false,
                                        );
                                        e.timestamp(Utc::now().to_rfc3339());
                                        e.author(|a| {
                                            if let Some(f) = usr.national_flag_url() {
                                                a.icon_url(f);
                                            }
                                            a.name(&format!(
                                                "Lv.{} {} {}xp",
                                                usr.level(),
                                                level_symbol(usr.level()),
                                                usr.xp.separate_with_commas()
                                            ))
                                        });
                                        e.color(rank_col(
                                            &usr.league.rank,
                                            &usr.league.percentile_rank,
                                        ));
                                        e.thumbnail(usr.face());
                                        e.set_footer(ftr());
                                        Interactions::Some(vec![InteractMode::Embed(e)])
                                    } else {
                                        Interactions::Some(vec![InteractMode::Embed(
                                            CreateEmbed::default()
                                                .title(format!(
                                                    "{}  ||{}||",
                                                    usr.name.to_uppercase(),
                                                    usr.id
                                                ))
                                                .description("")
                                                .thumbnail("https://tetr.io/res/avatar-banned.png")
                                                .image("https://ch.tetr.io/res/cute.png")
                                                .field("| **BANNED** |", "\u{200B}", false)
                                                .footer(|f| {
                                                    f.text(format!(
                                                        "{}\n/tetr-user{}",
                                                        latency(),
                                                        if let Some(m) = interact.member.as_ref() {
                                                            format!(", Called by {}", m.user.name)
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
                                            .title(user.to_uppercase())
                                            .description(format!(
                                                "```\n{}\n```\n{}",
                                                res.error.unwrap(),
                                                latency()
                                            ))
                                            .set_footer(ftr())
                                            .timestamp(Utc::now().to_rfc3339())
                                            .color(MAIN_COL)
                                            .to_owned(),
                                    )])
                                }
                            }
                            Err(why) => Interactions::Some(vec![InteractMode::Message(format!(
                                "Error: {}",
                                why.to_string()
                            ))]),
                        }
                    } else {
                        let before = Instant::now();
                        let after = Instant::now();
                        let latency = format!("latency: {}ms", (after - before).as_millis());
                        Interactions::Some(vec![InteractMode::Embed(
                            CreateEmbed::default()
                                .title("SYABETAROU ✓ ||77a02950-bde0447f9851fd||")
                                .description("**<SUPPORTER>**")
                                .fields(vec![
                                    (
                                        "Badges: <:100player:992097864081735730><:allclear:992096168664383622><:20tsd:992097227260567553><:secretgrade:992079389611278477><:leaderboard1:992095621018308759>| More 18 badges",
                                        "\u{200B}",
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
                                        "\u{200B}",
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
                                        "\u{200B}",
                                        &format!(
                                            "{} | cached at: <t:{}:R>",
                                            latency,
                                            Utc::now().timestamp()
                                        ),
                                        false,
                                    )
                                ])
                                .set_footer(ftr())
                                .timestamp(Utc::now().to_rfc3339())
                                .author(|a| {
                                    a.icon_url("https://tetr.io/res/flags/jp.png");
                                    a.name("Lv.9999 ⬢ 8,401,9463,557xp")
                                })
                                .color(0xeca5ff)
                                .thumbnail("https://cdn.discordapp.com/avatars/518899666637553667/3ae6b018626d3b596c31c241a56df088.webp")
                                .to_owned()
                        )])
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

            match content {
                Interactions::Some(im) => {
                    if let Err(why) = interact
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| {
                                    let mut action_row = CreateActionRow::default();
                                    for i in im {
                                        match i {
                                            InteractMode::Message(c) => {
                                                m.content(c);
                                            }
                                            InteractMode::Embed(e) => {
                                                m.add_embed(e);
                                            }
                                            InteractMode::Button(b) => {
                                                action_row.add_button(b);
                                            }
                                        }
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
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
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
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
                Interactions::None => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        // Send a ready message for terminal.
        // Current date and time (JST)
        let now = Utc::now() + Duration::hours(9);
        println!(
            "{}{} {} Kagurin-rs v{} is connected.",
            now.format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            "INFO".bright_green(),
            VER,
        );

        // Set activity.
        ctx.set_activity(Activity::playing(
            "スラッシュコマンド実装中... | Dev:Rinrin.rs#5671",
        ))
        .await;

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
    let token = env::var("KAGURIN_RS_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents.
    let mut intents = GatewayIntents::default();
    intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents.insert(GatewayIntents::GUILD_MESSAGES);

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
