//! These are utilities for kagurin-rs

use std::{env, process};

/// Get the current memory usage by this bot. (MiB)
pub fn get_memory_usage() -> f64 {
    let stdout = process::Command::new("sh")
        .args([
            format!("{}/../assets/shells/pmap.sh", env!("CARGO_MANIFEST_DIR")),
            process::id().to_string(),
        ])
        .output()
        .unwrap()
        .stdout;
    String::from_utf8_lossy(&stdout)
        .to_string()
        .replace('\n', "")
        .parse::<f64>()
        .unwrap()
        / 1024.0
}

/// Rounds the passed `f64` halfway.
/// Leaves up to the position specified by `decimal_places`argument.
///
/// # Examples:
///
/// ```
/// assert_eq!(round_mid(3.1415, 3), 3.142);
/// ```
pub fn round_mid(num: f64, decimal_places: u32) -> f64 {
    let n = 10_u64.pow(decimal_places) as f64;
    (num * n).round() / n
}

/// Get the current uptime of this bot.
pub fn get_uptime() -> String {
    let stdout = process::Command::new("sh")
        .args([
            format!("{}/../assets/shells/ps.sh", env!("CARGO_MANIFEST_DIR")),
            process::id().to_string(),
        ])
        .output()
        .unwrap()
        .stdout;
    String::from_utf8_lossy(&stdout)
        .to_string()
        .replace('\n', "")
}

pub mod fmt {
    //! This is a module for formatting texts.

    use std::fmt::Display;

    /// Formats to a codeblock.
    ///
    /// We can also specify language.
    ///
    /// # Examples:
    ///
    /// ```
    /// assert_eq!(
    ///     cb("print(Hello, World!)", "py"),
    ///     "```py\nprint(Hello, World!)\n```"
    /// );
    /// ```
    pub fn cb<T: Display>(content: T, lang: &str) -> String {
        format!("```{}\n{}\n```", lang, content)
    }

    /// Formats to inlined monospaced text.
    ///
    /// # Examples:
    ///
    /// ```
    /// assert_eq!(mn("Hello, World!"), "`Hello, World!`");
    /// ```
    pub fn mn<T: Display>(content: T) -> String {
        format!("`{}`", content)
    }
}
