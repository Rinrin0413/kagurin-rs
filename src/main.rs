use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, prelude::*},
    prelude::*,
    utils::MessageBuilder,
};

use serde_json::json;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // MSG event
    async fn message(&self, ctx: Context, msg: Message) {
        //if msg.author.bot { return; }

        if msg.content == "kgrs!ping" {
            // Nomal MSG
            if let Err(why) = msg.channel_id.say(&ctx.http, "贵樣!").await {
                er("sending msg", why);
            }
        }

        if msg.content == "kgrs!dm" {
            // dm
            let dm = msg.author.dm(&ctx, |m| m.content("Yoooo")).await;
            if let Err(why) = dm {
                er("when direct msging user", why);
            }
        }

        if msg.content == "kgrs!MessageBuilder" {
            // dynamically(decorationary) MSG
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    er("getting channel", why);
                    return;
                }
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
            println!("{:?}", content);
            if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
                er("sending message", why);
            }
        }

        if msg.content == "kgrs!embed&img" {
            // The create message builder allows you to easily create embeds and messages
            // using a builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, an image, three fields, and a footer.
            let msg = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("ただの文章")
                        .embed(|e| {
                            e.title("全体タイトル")
                                .description("全体説明")
                                .image("https://media.discordapp.net/attachments/894239318330179624/938502857404088382/unknown.png?width=764&height=663")
                                .fields(vec![
                                    ("第壱フィールドタイトル", "第壱フィールドボディ(インライン)", true),
                                    ("第贰フィールドタイトル", "第贰フィールドボディ(インライン)", true),
                                ])
                                .field(
                                    "第叁フィールドタイトル",
                                    "第叁フィールドボディ(インラインじゃないよ)",
                                    false,
                                )
                                .footer(|f| f.text("フッター"))
                                // Add a timestamp for the current time
                                // This also accepts a rfc3339 Timestamp
                                .timestamp(chrono::Utc::now())
                        })
                        // ↓ ただの画像添付
                        .add_file("https://media.discordapp.net/attachments/894239318330179624/938470521748725840/unknown.png")//"./ferris_eyes.png")
                })
                .await;

            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
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
