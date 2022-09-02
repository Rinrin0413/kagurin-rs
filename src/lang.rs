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

    pub fn help_cmd() -> HashMap<String, (String, String)> {
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

    pub fn help_cmd_display() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Display commands".to_string(), 
            "表示系コマンド一覧".to_string()
        ));
        d
    }

    pub fn help_cmd_util() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Utility commands".to_string(), 
            "機能系コマンド一覧".to_string()
        ));
        d
    }

    pub fn help_cmd_fun() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Entertainment commands".to_string(), 
            "娯楽系コマンド一覧".to_string()
        ));
        d
    }

    pub fn help_cmd_tetrio() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "TETR.IO related commands".to_string(), 
            "TETR.IO関連のコマンド一覧".to_string()
        ));
        d
    }

    pub fn help_cmd_admin() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Admin commands".to_string(), 
            "管理者向けコマンド一覧".to_string()
        ));
        d
    }

    pub fn help_cmd_dev() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Commands for Rinrin".to_string(), 
            "Rinrin用コマンド一覧".to_string()
        ));
        d
    }

    pub fn help_cmd_trust() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("title".to_string(), (
            "Commands for users trusted by Rinrin".to_string(), 
            "Rinrinに信頼されてるユーザー向けコマンド一覧".to_string()
        ));
        d
    }
}
