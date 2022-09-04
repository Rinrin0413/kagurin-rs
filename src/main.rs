use chrono::{Duration, Utc};
use colored::*;
use kgrs::response_interactions::{InteractMode, Interactions};
use lang::dict;
use serenity::{
    async_trait,
    builder::{CreateActionRow, CreateButton, CreateComponents, CreateEmbed, CreateEmbedFooter},
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
use std::{collections::HashMap, env, time::Instant};

const RUST_VERSION: &str = "1.64.0-nightly";
const OS: &str = "openSUSE Leap 15.4 x86_64";

const VER: &str = env!("CARGO_PKG_VERSION");
const MAIN_COL: u32 = 0xB89089;
const INVITE_URL: &str =
    "https://discord.com/api/oauth2/authorize?client_id=936116497502318654&permissions=8&scope=bot";
//const TRUSTED: [u64; 2] = [
//    724976600873041940, // Rinrin.rs
//    801082943371477022, // Rinrin.hlsl
//];
//const DEVELOPER: [u64; 2] = [
//    724976600873041940, // Rinrin.rs
//    801082943371477022, // Rinrin.hlsl
//];
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
English: Do you need help? If so, please use slash command `/help`.\n\
日本語: ヘルプが必要ですか? もしそうなら、スラッシュコマンド `/help` を使用されたし。\
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
                                        ("/ping", "pong!".to_string(), false),
                                        ("/info", dict_lookup(dict, "info"), false),
                                        ("/neofetch", dict_lookup(dict, "neofetch"), false),
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
                                    .fields(vec![("/", "Nothing here yet :(".to_string(), false)])
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
                                    .fields(vec![("/", "Nothing here yet :(".to_string(), false)])
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
                                    ("/help", dict_lookup(dict, "help"), false),
                                    ("/help display", dict_lookup(dict, "help.display"), false),
                                    ("/help util", dict_lookup(dict, "help.util"), false),
                                    ("/help fun", dict_lookup(dict, "help.fun"), false),
                                    ("/help tetrio", dict_lookup(dict, "help.tetrio"), false),
                                    ("/help admin", dict_lookup(dict, "help.admin"), false),
                                    ("/help dev", dict_lookup(dict, "help.dev"), false),
                                    ("/help trust", dict_lookup(dict, "help.trust"), false),
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
                                    (
                                        "OS:",
                                        format!("```ansi\n[0;31m{}\n```", OS),
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
                "neofetch" => {
                    Interactions::Some(vec![
                        InteractMode::Embed(
                            CreateEmbed::default()
                                .description(format!(
                                    r#"
```ansi
kgrs@rinrin:~> neofetch
     [0;33mRRRRRRRRR         [0;31mKagurin.rs
 [0;33m.s*R*RRRRRR*===       [0;0m---------- 
[0;33m:sRRRRRRRRRRR*-:-      [0;31mVersion[0;0m: {}
 [0;33m*RRRR*RRRRRRR-:sR     [0;31mOS[0;0m: {}
 [0;33mR***s==ss*****RRR     [0;31mHost[0;0m: Rinrin.rs#5671
  [0;33ms==s-.::=ss=sRRR     [0;31mLanguage[0;0m: Rust {}
  [0;33m=::s=   :=s---RR     [0;31mLibrary[0;0m: Serenity-rs v0.11.5
   [0;33m- ..    ..:-RRR     [0;31mTheme[0;0m: kgrs
   [0;33mR:      .-=sRRR     [0;31mLocale[0;0m: ja_JP.UTF-8 / en_US.UTF-8
  [0;33mRRRs-....:==sRRR     [0;31mID[0;0m: {}
  [0;33m*R RR=-:::--sRRR     [0;31mServers[0;0m: {} guilds
 [0;33ms**:*s--:::==s***s    [0;31mCreated at[0;0m: {}
  [0;33ms==s=:-- :====ss=    
  [0;33m=s=s::*: =s==s**s       [0;0m███[0;30m███[0;31m███[0;32m███[0;33m███[0;34m███[0;35m███[0;36m███[0;37m███
  [0;33m%R=s:=*::ss=*RRR     
```
                                    "#, 
                                    VER,
                                    OS,
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
                        ),
                    ])
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
            "{}",
            format!(
                "{}{} {} Kagurin-rs v{} is connected.",
                now.format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                "INFO".bright_green(),
                VER,
            )
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
