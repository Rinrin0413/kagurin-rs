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
        d
    }

    pub fn help_fun() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Entertainment commands".to_string(), 
            "娯楽系コマンド一覧".to_string()
        ));
        d
    }

    pub fn help_tetrio() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "TETR.IO related commands".to_string(), 
            "TETR.IO関連のコマンド一覧".to_string()
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
        d
    }
}
