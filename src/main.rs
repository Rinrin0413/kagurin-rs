use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // MSG event
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "kgrs!ping" { // Nomal MSG
            if let Err(why) = msg.channel_id.say(&ctx.http, "贵樣!").await {
                er("sending msg", why);
            }
        }
        if msg.content == "kgrs!dm" { // dm
            let dm = msg.author.dm(&ctx, |m| m.content("Yoooo")).await;
            if let Err(why) = dm {
                er("when direct msging user", why);
            }
        }
        if msg.content == "kgrs!MessageBuilder" { // dynamically(decorationary) MSG
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    er("getting channel", why);
                    return;
                },
            };
            let content = MessageBuilder::new()
                .push("贵樣(")
                .push_bold_safe(&msg.author.name)
                .push(") ば `kgrs!dynamiccmd` を ")
                .mention(&channel)
                .push(" で使用レだ。")
                .build();
            if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
                er("sending message", why);
            }
        }
        if msg.content == "kgrs!MessageBuilder2" {
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
                .push_safe("*アスタリスク*`グレイヴ・アクセント`_アンダーライン_")
                .build();
                println!("{:?}", content);
            if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
                er("sending message", why);
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

fn er(err: &str, why: serenity::Error) {
    println!("Error {}: {:?}", err, why);
}
