#[rustfmt::skip]
pub mod dict {
    use std::collections::HashMap;

    pub fn help_cmd_general() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("implSlashCmds".to_string(), (
            "Slash commands is now implemented! (in the middle)".to_string(), 
            "スラッシュコマンドが実装されました! (実装途中)".to_string()
        ));
        d
    }

    pub fn help() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Help commands".to_string(), 
            "ヘルプコマンド一覧".to_string()
        ));
        d.insert("help".to_string(), (
            "Show help for help commands.".to_string(), 
            "ヘルプコマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.display".to_string(), (
            "Show help for display commands.".to_string(), 
            "表示系コマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.util".to_string(), (
            "Show help for utility commands.".to_string(), 
            "機能系コマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.fun".to_string(), (
            "Show help for entertainment commands.".to_string(), 
            "娯楽系コマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.tetrio".to_string(), (
            "Show help for [TETR.IO](https://tetr.io) related commands.".to_string(), 
            "[TETR.IO](https://tetr.io)関連のコマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.admin".to_string(), (
            "Show help for admin commands.".to_string(), 
            "管理者向けコマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.dev".to_string(), (
            "Show help for commands for Rinrin.".to_string(), 
            "Rinrin用コマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.trust".to_string(), (
            "Show help for commands for users trusted by Rinrin.".to_string(), 
            "Rinrinに信頼されてるユーザー向けコマンドのヘルプを表示します。".to_string()
        ));
        d
    }

    pub fn help_display() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Display commands".to_string(), 
            "表示系コマンド一覧".to_string()
        ));
        d.insert("info".to_string(), (
            "Show information about this bot.".to_string(), 
            "このボットに関する情報を表示します。".to_string()
        ));
        d.insert("neofetch".to_string(), (
            "Display information about this bot in an aesthetic and visually pleasing way.".to_string(), 
            "このボットに関する情報を美しく視覚的に楽しいカンジで表示します。".to_string()
        ));
        d
    }

    pub fn help_util() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Utility commands".to_string(), 
            "機能系コマンド一覧".to_string()
        ));
        d.insert("cjp".to_string(), (
            "Convert the string to 怪レい日本语(correct Japanese).".to_string(), 
            "渡された文字列を怪レい日本语に変換します。".to_string()
        ));
        d
    }

    pub fn help_fun() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Entertainment commands".to_string(), 
            "娯楽系コマンド一覧".to_string()
        ));
        d.insert("jsd".to_string(), (
            "Create a image with [Japanese Stable diffusion](https://github.com/rinnakk/japanese-stable-diffusion).".to_string(), 
            "[Japanese Stable diffusion](https://github.com/rinnakk/japanese-stable-diffusion) で画像を作成します。".to_string()
        ));
        d
    }

    pub fn help_tetrio() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "TETR.IO related commands".to_string(), 
            "TETR.IO関連のコマンド一覧".to_string()
        ));
        d.insert("tetr-user".to_string(), (
            "Display details of the target TETR.IO user.".to_string(), 
            "対象の TETR.IO のユーザーの詳細を表示します。".to_string()
        ));
        d
    }

    pub fn help_admin() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Admin commands".to_string(), 
            "管理者向けコマンド一覧".to_string()
        ));
        d
    }

    pub fn help_dev() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Commands for Rinrin".to_string(), 
            "Rinrin用コマンド一覧".to_string()
        ));
        d.insert("exit".to_string(), (
            "Kill the bot.".to_string(), 
            "Bot を強制終了します。".to_string()
        ));
        d
    }

    pub fn help_trust() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Commands for users trusted by Rinrin".to_string(), 
            "Rinrinに信頼されてるユーザー向けコマンド一覧".to_string()
        ));
        d
    }

    pub fn info() ->  HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Informations de Kagurin.rs".to_string(), 
            "かぐりん.rs の情報".to_string()
        ));
        d.insert("nameTitle".to_string(), (
            "Name:;".to_string(), 
            "名前:".to_string()
        ));
        d.insert("botVer".to_string(), (
            "Bot version:".to_string(), 
            "Botバージョン:".to_string()
        ));
        d.insert("createdAt".to_string(), (
            "Created at:".to_string(), 
            "生誕:".to_string()
        ));
        d.insert("guildsTitle".to_string(), (
            "Guilds:".to_string(), 
            "導入サーバー数:".to_string()
        ));
        d.insert("guildsTxt".to_string(), (
            " guilds".to_string(), 
            "個".to_string()
        ));
        d.insert("dev".to_string(), (
            "Developer:".to_string(), 
            "開発者:".to_string()
        ));
        d.insert("lang".to_string(), (
            "Language:".to_string(), 
            "言語:".to_string()
        ));
        d.insert("lib".to_string(), (
            "Library:".to_string(), 
            "ライブラリ:".to_string()
        ));
        d.insert("memory".to_string(), (
            "Memory:".to_string(),
            "メモリ:".to_string()
        ));
        d.insert("uptime".to_string(), (
            "Uptime:".to_string(),
            "稼働時間:".to_string()
        ));
        d
    }

    pub fn tetr_user() ->  HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("err.plzSendUserNameOrID".to_string(), (
            "Invalid user. Please specify a username or userID.".to_string(), 
            "無効なユーザーです。ユーザー名かユーザーIDを指定してください。".to_string()
        ));
        d
    }

    pub fn exit() ->  HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("unauthorized".to_string(), (
            "Sorry, you cannot call this command:(".to_string(), 
            "すみません、このコマンドはあなたには使用できません ; ;".to_string()
        ));
        d
    }

    pub fn cjp() ->  HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("err.strTooLong".to_string(), (
            "String too long.".to_string(), 
            "文字列が長すぎます。".to_string()
        ));
        d.insert("title".to_string(), (
            "Converted to 怪レい日本语(correct Japanese)".to_string(), 
            "変换 to 怪レい日本语.".to_string()
        ));
        d.insert("input".to_string(), (
            "Original:".to_string(), 
            "原文:".to_string()
        ));
        d.insert("output".to_string(), (
            "Correct:".to_string(), 
            "怪レい:".to_string()
        ));
        d
    }

    pub fn jsd() ->  HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("plzWait".to_string(), (
            "Please wait...".to_string(), 
            "お待ち下さい...".to_string()
        ));
        d.insert("err.plzRetry".to_string(), (
            "A problem occurred on the API side.\nSorry, please try again.".to_string(), 
            "API側で問題が発生しました。\n申し訳ありませんが、もう一度お試しください。".to_string()
        ));
        d.insert("subject".to_string(), (
            "Subject".to_string(), 
            "主題".to_string()
        ));
        d.insert("sensitiveFrag".to_string(), (
            "\n* This image has the NSFW content flag set.".to_string(), 
            "\n※ 不適切画像フラグが立っています。".to_string()
        ));
        d.insert("calledBy.before".to_string(), (
            "Called by ".to_string(), 
            String::new()
        ));
        d.insert("calledBy.after".to_string(), (
            String::new(), 
            " によって実行".to_string()
        ));
        d.insert("btn.retry".to_string(), (
            "Retry".to_string(), 
            "再試行".to_string()
        ));
        d.insert("retrying".to_string(), (
            "Retrying now...".to_string(), 
            "再試行中...".to_string()
        ));
        d.insert("imageBrokenFrag".to_string(), (
            "\n* This image may be corrupted.".to_string(), 
            "\n※ この画像は破損している可能性があります。".to_string()
        ));
        d
    }

    pub fn jsd_retry() ->  HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("retrying".to_string(), (
            "Retrying...\n* You will be said this interaction failed, but no problem".to_string(), 
            "お待ち下さい...\n※「インタラクションに失敗しました」と表示されますが 問題はありません".to_string()
        ));
        d
    }
}
