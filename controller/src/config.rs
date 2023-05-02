use crate::logger::LogLevel;

pub struct Config {
    pub(crate) _num_threads: usize,
    pub(crate) log_level: LogLevel,
    pub(crate) address: String,
    pub(crate) otlp_url: Option<String>,
}

impl Config {
    pub fn new(
        num_threads: Option<usize>,
        log_level: Option<LogLevel>,
        address: Option<String>,
        otlp_url: Option<String>,
    ) -> Self {
        Self {
            _num_threads: num_threads.unwrap_or(8),
            log_level: log_level.unwrap_or(LogLevel::Info),
            address: address.unwrap_or(String::from("0.0.0.0:3000")),
            otlp_url,
        }
    }
}
