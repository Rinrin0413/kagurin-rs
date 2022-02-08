use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // MSG event
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "kgrs!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "贵樣!").await {
                er(why);
            }
        }
        if msg.content == "kgrs!ping2" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "贵樣！!").await {
                er(why);
            }
        }
    }
    // READY event
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
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

fn er(why: serenity::Error) {
    println!("Error sending message: {:?}", why);
}
