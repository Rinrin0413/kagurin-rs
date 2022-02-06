use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("kgrs!")) // cmd prefix
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("KAGURIN_RS_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{:?}", ctx.shard);
    msg.reply(ctx, "Pong!").await?;
    msg.reply(ctx, "Pong!2").await?;
    Ok(())
}
#[command]
async fn ping2(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!2").await?;
    Ok(())
}