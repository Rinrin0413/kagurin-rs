use std::{collections::HashMap, env};

use serenity::{
    async_trait,
    //builder::CreateEmbedAuthor,
    model::{channel::Message, gateway::Ready, prelude::*},
    prelude::*,
    utils::{Colour, MessageBuilder},
};

use serde_json::{json, Value};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // MSG event
    async fn message(&self, ctx: Context, msg: Message) {
        //if msg.author.bot { return; }

        // help
        if msg.content == "kgrs!help" {
            let content = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("ただの文章").embed(|e| {
                        e.author(|a| {
                            a.icon_url("https://cdn.discordapp.com/avatars/936116497502318654/a0b82d4e3d428cd578e24029ad05d2aa.png")
                                .name(&"かぐりん.rs's Commands")
                        });
                        e.title("コマンド一覧");
                        e.description("Command prefix: `kgrs!`");
                        e.fields(vec![
                            ("```kgrs!help```", "`コマンドのヘルプを表示`", true),
                            ("```kgrs!info```", "`このボットの詳細を表示`", true)
                        ]);
                        e.footer(|f|
                            f.text(format!(
                                "kgrs!help by {}", 
                                match &msg.member.as_ref().expect("kgrs!help / FOOTER").nick {
                                    Some(n) => n,
                                    None => &msg.author.name,
                                }
                            ))
                        );
                        e.timestamp(chrono::Utc::now());
                        e.color(Colour(0xB89089));
                        e
                    })
                })
                .await;

            if let Err(why) = content {
                println!("Error sending message: {:?}", why);
            }
        }

        // ping
        if msg.content == "kgrs!ping" {
            // Nomal MSG
            if let Err(why) = msg.channel_id.say(&ctx.http, "贵樣!").await {
                er("sending msg", why);
            }
        }

        // directMsg
        if msg.content == "kgrs!directMsg" {
            // dm
            let dm = msg.author.dm(&ctx, |m| m.content("Yoooo")).await;
            if let Err(why) = dm {
                er("when direct msging user", why);
            }
        }

        // MessageBuilder
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

        // MessageBuilder2
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

        // embed&img
        if msg.content == "kgrs!embed&img" {
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
                            e.timestamp(chrono::Utc::now());

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

            if let Err(why) = content {
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
