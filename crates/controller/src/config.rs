use crate::logger::LogLevel;
use serde::Deserialize;

fn default_treads() -> usize {
    4
}

fn default_log_level() -> LogLevel {
    LogLevel::default()
}
fn default_address() -> String {
    "0.0.0.0:3000".to_string()
}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(default = "default_treads")]
    pub(crate) num_threads: usize,
    #[serde(default = "default_log_level")]
    pub(crate) log_level: LogLevel,
    #[serde(default = "default_address")]
    pub(crate) address: String,
    pub(crate) otlp_url: Option<String>,
}
