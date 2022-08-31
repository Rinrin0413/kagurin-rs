use chrono::{Duration, Utc};
use colored::*;
use serenity::{
    async_trait,
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        gateway::{Activity, Ready},
    },
    prelude::*,
};
use std::{env, time::Instant};

//const RUST_VERSION: &str = "1.64.0-nightly";
const VER: &str = env!("CARGO_PKG_VERSION");
//const EMBED_LABEL_COL: u32 = 0xB89089;
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
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                // pong! | 1014243185880465550
                "ping" => {
                    let before = Instant::now();
                    if let Err(why) = command
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
                    if let Err(why) = command
                        .edit_original_interaction_response(&ctx.http, |m| {
                            m.content(format!("pong! ({}ms)", (after - before).as_millis()))
                        })
                        .await
                    {
                        println!("Cannot respond to edit message:: {}", why);
                    };
                    String::new()
                }

                _ => "\
                    not implemented this command :<\n\
                    <@!724976600873041940><@!724976600873041940>\
                    <@!724976600873041940><@!724976600873041940>\
                    <@!724976600873041940><@!724976600873041940>\
                    "
                .to_string(),
            };

            if content.is_empty() {
                return;
            }

            if let Err(why) = command
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
            //use serenity::model::application::command::CommandType;

            println!(
                "{}",
                "In command management mode ⚙".white().on_black().bold()
            );
            cmd_list(&ctx.http).await;

            // Main manager.
            CmdManager::new()
                // TODO: impl a function `type_annot_dummy`
                .create(|f| f)
                .edit(0, |f| f)
                .delete(0)
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
