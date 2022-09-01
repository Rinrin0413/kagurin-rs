use chrono::{Duration, Utc};
use colored::*;
use kgrs::response_interactions::InteractMode;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateEmbedFooter},
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        gateway::{Activity, Ready},
    },
    prelude::*,
};
use std::{env, time::Instant};

//const RUST_VERSION: &str = "1.64.0-nightly";
const VER: &str = env!("CARGO_PKG_VERSION");
const MAIN_COL: u32 = 0xB89089;
//const TRUSTED: [u64; 2] = [
//    724976600873041940, // Rinrin.rs
//    801082943371477022, // Rinrin.hlsl
//];
//const DEVELIPER: [u64; 2] = [
//    724976600873041940, // Rinrin.rs
//    801082943371477022, // Rinrin.hlsl
//];
//const BOT_ID: u64 = 936116497502318654;
//const IS_DST: bool = true; // Is daylight saving time(for Sky:CotL)
//const INVITE_URL: &str =
//    "https://discord.com/api/oauth2/authorize?client_id=936116497502318654&permissions=8&scope=bot";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
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
            let jp_or = |other: &str, jp: &str| {
                if interact.locale == "ja" {
                    jp
                } else {
                    other
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
                    InteractMode::None
                }

                // Show command help | 1014735729139662898
                "help" => {
                    if args.is_empty() {
                        InteractMode::Embed(
                            CreateEmbed::default()
                                .author(|a| a.icon_url(bot_icon).name(" "))
                                .title("Help commands")
                                .description(jp_or(
                                    "Slash commands is now implemented! (in the middle)",
                                    "スラッシュコマンドが実装されました! (実装途中)",
                                ))
                                .fields(vec![
                                    (
                                        "/help",
                                        jp_or(
                                            "Show help for help commands.",
                                            "ヘルプコマンドのヘルプを表示します。",
                                        ),
                                        false,
                                    ),
                                    // ("/help display", "Show help for display commands", false),
                                    // ("/help util", "Show help for utility commands", false),
                                    // ("/help fun", "Show help for entertainment commands", false),
                                    // ("/help tetrio", "Show help for [TETR.IO](https://tetr.io) related commands", false),
                                    // ("/help admin", "Show help for admin commands", false),
                                    // ("/help dev", "Show help for Rinrin", false),
                                    // ("/help trust", "Show help for trust commands", false),
                                ])
                                .set_footer(ftr())
                                .timestamp(Utc::now().to_rfc3339())
                                .color(MAIN_COL)
                                .to_owned(),
                        )
                    } else {
                        InteractMode::Dev
                    }
                }

                _ => InteractMode::Message(
                    "\
                    not implemented yet :<\n\
                    <@!724976600873041940><@!724976600873041940>\
                    <@!724976600873041940><@!724976600873041940>\
                    <@!724976600873041940><@!724976600873041940>\
                    "
                    .to_string(),
                ),
            };

            match content {
                InteractMode::Message(content) => {
                    if let Err(why) = interact
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(content))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
                InteractMode::Embed(e) => {
                    if let Err(why) = interact
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.add_embed(e))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
                InteractMode::Dev => {
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
                InteractMode::None => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        // Send a ready message for terminal.
        // Current date and time (JST)
        let now = Utc::now() + Duration::hours(9);
        println!(
            "Kagurin-rs v{} is connected at {} (JST)",
            VER,
            now.format("%Y-%m-%d %H:%M:%S.%z")
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
        if if let Some(arg1) = env::args().nth(1) {
            arg1 == "cmdmng"
        } else {
            false
        } {
            use kgrs::cmd_mng::{cmd_list, CmdManager};
            use serenity::{model::application::command::{
                    CommandOptionType,
                    //CommandType,
            }};

            println!(
                "{}",
                "In command management mode ⚙".white().on_black().bold()
            );
            cmd_list(&ctx.http).await;

            // Main manager.
            // ! WARNING: If manage multiple commands at once, Clone the variable `cmd`.
            // !          Recommend always cloning to avoid mistakes.
            let cmd = serenity::builder::CreateApplicationCommand::default();
            CmdManager::new()
                .edit(1014735729139662898, 
                    cmd.clone()
                        .name("help").description("Show command help")
                        .description_localized("ja", "コマンドのヘルプを表示します")
                        .create_option(|o| {
                            o.name("kind").description("Input kind of help commands.")
                                .name_localized("ja", "種類").description_localized("ja", "ヘルプコマンドの種類を入れてください。")
                                .add_string_choice("display", "Display commands")
                                .add_string_choice("util", "Utility commands")
                                .add_string_choice("fun", "Entertainment commands")
                                .add_string_choice("tetrio", "TETR.IO related commands")
                                .add_string_choice("admin", "Admin commands")
                                .add_string_choice("trusted", "Commands for user trusted by developer")
                                .add_string_choice("dev", "Developer commands")
                                .kind(CommandOptionType::String)
                        })
                )
                .run(&ctx.http)
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("KAGURIN_RS_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
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
