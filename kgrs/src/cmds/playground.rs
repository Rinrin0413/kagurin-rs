use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PostData {
    /// The code to run.
    pub code: String,
    /// Create type. (either `bin` or `lib`)
    #[serde(rename = "crateType")]
    pub crate_type: String,
    /// Optimization level. (either `debug` or `release`)
    pub mode: String,
    /// Rust version. (either `stable`, `beta`, or `nightly`)
    pub channel: String,
    /// Edition. (either `2015`, `2018` or `2021`)
    pub edition: String,
    /// Whether to enable backtrace.
    pub backtrace: bool,
    /// Whether to enable testings.
    pub tests: bool,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub stdout: String,
    pub stderr: String,
}
