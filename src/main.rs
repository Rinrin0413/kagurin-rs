use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Enter the path to the file containing the bot token (ex: TOKEN)
    // If the corresponding file is not found in the corresponding path,
    // it will be created in that path...
    let token = env::var(open_file("TOKEN")).expect("token");
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
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

fn open_file(path: &str) -> String {
    let mut fh = match File::open(path) {
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("./TOKEN") {
                Ok(file) => panic!("ファイル `{}` を作成しました。\nそのファイルに bot のトークンを記入してください。\nハンドル: {:?}", path, file),
                Err(err) =>  panic!("ファイル `{}` の作成を試みましたが問題が発生: {:?}", path, err),
            }
        },
        Err(err) => panic!("ファイルを開く時に問題が発生: {:?}", err),
        Ok(file) => file,
    };
    let mut tkn = String::new();
    let _ = fh.read_to_string(&mut tkn);
    tkn.to_string()
}