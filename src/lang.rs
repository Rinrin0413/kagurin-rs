#[rustfmt::skip]
pub mod dict {
    use std::collections::HashMap;

    pub fn help_cmd() -> HashMap<String, (String, String)> {
        let mut d = HashMap::new();
        d.insert("implSlashCmds".to_string(), (
            "Slash commands is now implemented! (in the middle)".to_string(), 
            "スラッシュコマンドが実装されました! (実装途中)".to_string()
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
            "Show help for Rinrin.".to_string(), 
            "Rinrin用コマンドのヘルプを表示します。".to_string()
        ));
        d.insert("help.trust".to_string(), (
            "Show help for commands for users trusted by Rinrin.".to_string(), 
            "Rinrinに信頼されてるユーザー向けコマンドのヘルプを表示します。".to_string()
        ));
        d
    }
}
