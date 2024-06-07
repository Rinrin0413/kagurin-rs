# Code snippets for command management

## Usage

Enter these code snippets into methods of the struct [`CmdManager`](./kgrs/src/cmd_mng.rs) at "Main manager." in [`main.rs`](./kagurin_rs/src/main.rs).

### Example:

```rs
use kgrs::cmd_mng::{cmd_list, CmdManager};
use serenity::model::application::command::{
    // May need these.
    //CommandOptionType,
    //CommandType,
};

// Prints command list.
// You can you this function to check which commands are there.
cmd_list(&ctx.http).await;

// Main manager.
// ! WARNING: If manage multiple commands at once, Clone the variable `cmd`.
// !          Recommend always cloning to avoid mistakes.
let cmd = serenity::builder::CreateApplicationCommand::default();
CmdManager::new()

    // Creates a command.
    .create(cmd.clone()
        .name("ping").description("pong!")
        .description_localized("ja", "pong!")
    })

    // Edits a command.
    .edit(1014243185880465557, cmd.clone()
        .name("info").description("Show bot information")

    // Deletes a command.
    .delete(1014243185880465558)

    .run(&ctx.http)
    .await;
```

## Snippets

---

### `/ping` | `1014243185880465550`

```rs
cmd.clone().name("ping").description("pong!").description_localized("ja", "pong!")
```

---

### `/help [kind:string]` | `1014735729139662898`

```rs
cmd.clone()
    .name("help").description("Show command help")
    .description_localized("ja", "コマンドのヘルプを表示します")
    .create_option(|o| {
        o.name("kind").description("Input kind of help commands.")
            .name_localized("ja", "種類").description_localized("ja", "ヘルプコマンドの種類を入れてください。")
            .add_string_choice("display", "display")
            .add_string_choice("util", "util")
            .add_string_choice("fun", "fun")
            .add_string_choice("tetrio", "tetrio")
            .add_string_choice("admin", "admin")
            .add_string_choice("trust", "trust")
            .add_string_choice("dev", "dev")
            .kind(CommandOptionType::String)
    })
```

---

### `/info` | `1015567292022673449`

```rs
cmd.clone()
    .name("info").description("Show information about this bot")
    .description_localized("ja", "このボットに関する情報を表示します")
```

---

### `/neofetch` | `1015944810647011328`

```rs
cmd.clone()
    .name("neofetch").description("Display information about this bot in an aesthetic and visually pleasing way")
    .description_localized("ja", "このボットに関する情報を美しく視覚的に楽しいカンジで表示します")
```

---

### `/rust` | `1097501737994162228`

```rs
cmd.clone()
    .name("rust").description("Run Rust code in Rust playground")
    .description_localized("ja", "Rust のコードを Rust Playground で実行します")
```

---


### `/tetr-user <username/user-id:string>` | `1018530733314289737`

```rs
cmd.clone()
    .name("tetr-user").description("Display details of the target TETR.IO user")
    .description_localized("ja", "対象の TETR.IO のユーザーの詳細を表示します")
    .create_option(|o| {
        o.name("user").description("TETR.IO user name or ID")
            .name_localized("ja", "ユーザー").description_localized("ja", "TETR.IO のユーザー名またはID")
            .kind(CommandOptionType::String)
            .required(true)
    })
```

---

### `/exit` | `1019672344643522580`

```rs
cmd.clone()
    .name("exit").description("Kill the bot.")
    .description_localized("ja", "Bot を強制終了します。")
```

---

### `/cjp <sentence:string> [version:string]` | `1021847038545100810`

```rs
cmd.clone()
    .name("cjp").description("Convert the string to 怪レい日本语(Ayashī Nihongo)")
    .description_localized("ja", "渡された文字列を怪レい日本语に変換します")
    .create_option(|o| {
        o.name("string").description("Japanese sentence")
            .name_localized("ja", "文字列").description_localized("ja", "日本語の文章")
            .kind(CommandOptionType::String)
            .required(true)
    })
    .create_option(|o| {
        o.name("lib-v").description("Version of cjp.rs")
            .name_localized("ja", "バージョン").description_localized("ja", "cjp.rs のバージョン")
            .add_string_choice("latest (v1.0.0)", "latest")
            .add_string_choice("v0.1.0", "v0")
            .kind(CommandOptionType::String)
    })
```

---

### `/tetr-user-search <user>` | `1035478275910275093`

```rs
cmd.clone()
    .name("tetr-user-search").description("Search for a TETR.IO account by Discord account")
    .description_localized("ja", "DiscordアカウントからTETR.IOアカウントを調べます")
    .create_option(|o| {
        o.name("user").description("Discord user to look up")
            .name_localized("ja", "ユーザー").description_localized("ja", "調べるユーザー")
            .kind(CommandOptionType::User)
            .required(true)
    })
```

---

### `/now` | `1040285205874888787`

```rs
cmd.clone()
    .name("now")
    .description("Get current UNIX timestamp")
    .description_localized("ja", "現在の UNIXタイムスタンプを取得します",)
```

---

### `/ts <year:integer> <month:integer> <day:integer> [hour:integer] [minute:integer] [second:integer] [millisecond:integer]` | `1040293233839845396`

```rs
cmd.clone()
    .name("ts").description("Get UNIX timestamp of the specified datetime(UTC)")
    .description_localized("ja", "指定した日時(UTC)の UNIXタイムスタンプを取得します")
    .create_option(|o| {
        o.name("year").description("Year")
            .name_localized("ja", "年").description_localized("ja", "年")
            .kind(CommandOptionType::Integer)
            .required(true)
    })
    .create_option(|o| {
        o.name("month").description("Month")
            .name_localized("ja", "月").description_localized("ja", "月")
            .kind(CommandOptionType::Integer)
            .required(true)
    })
    .create_option(|o| {
        o.name("day").description("Day")
            .name_localized("ja", "日").description_localized("ja", "日")
            .kind(CommandOptionType::Integer)
            .required(true)
    })
    .create_option(|o| {
        o.name("hour").description("Hour (24-hour)")
            .name_localized("ja", "時").description_localized("ja", "時 (24時間)")
            .kind(CommandOptionType::Integer)
    })
    .create_option(|o| {
        o.name("minute").description("Minute")
            .name_localized("ja", "分").description_localized("ja", "分")
            .kind(CommandOptionType::Integer)
    })
    .create_option(|o| {
        o.name("second").description("Second")
            .name_localized("ja", "秒").description_localized("ja", "秒")
            .kind(CommandOptionType::Integer)
    })
    .create_option(|o| {
        o.name("millisecond").description("Millisecond")
            .name_localized("ja", "ミリ秒").description_localized("ja", "ミリ秒")
            .kind(CommandOptionType::Integer)
    })
```

---

### `/sfinder-path <field:string> [next:string]` | `1072236238574190754`

```rs
cmd.clone()
    .name("sfinder-path")
    .description("Get the solution to Perfect Clear from a specified field with solution-finder")
    .description_localized(
        "ja",
        "指定されたフィールドからのパフェルートをsolution-finderで求めます",
    )
    .create_option(|o| {
        o.name("field").description("Fumen data")
            .name_localized("ja", "フィールド").description_localized("ja", "テト譜データ")
            .kind(CommandOptionType::String)
            .required(true)
    })
    .create_option(|o| {
        o.name("next").description("Next pieces pattern")
            .name_localized("ja", "ネクスト").description_localized("ja", "ネクストのパターン")
            .kind(CommandOptionType::String)
            .required(false)
    })
```

---

<!-- ### `/jsd <prompts:string> [scale:integer]` | `1078586252393197659`

```rs
cmd.clone()
    .name("jsd").description("Generate a image with Japanese Stable diffusion")
    .description_localized("ja", "Japanese Stable diffusion で画像を生成します")
    .create_option(|o| {
        o.name("prompts").description("Prompts for image generation")
            .name_localized("ja", "プロンプト").description_localized("ja", "画像を生成するための主題")
            .kind(CommandOptionType::String)
            .required(true)
    })
    .create_option(|o| {
        o.name("scale").description("How much to follow the prompts [0-20] (default:10)")
            .name_localized("ja", "スケール").description_localized("ja", "プロンプトに従う程度 [0-20] (デフォルト:10)")
            .kind(CommandOptionType::Integer)
    })
```

--- -->
