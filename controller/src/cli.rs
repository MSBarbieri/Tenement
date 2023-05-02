use crate::{config::Config, logger::LogLevel};
use clap::{Parser, Subcommand};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Limit of threads exeeded: '{0}', limit is 8")]
    ThreadLimit(usize),
    #[error("Invalid type of address '{0}'")]
    InvaidAddress(String),
}

#[derive(Debug, Subcommand)]
pub enum CreateCommands {
    CRD,
    Application,
}

#[derive(Debug, clap::Args)]
pub struct StartCommandArgs {
    ///async treadpool size
    #[arg(short, long, default_value_t = 8)]
    pub num_threads: usize,

    /// Log/Tracing level
    #[arg(value_enum, short, long, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    /// server address
    #[arg(short, long, default_value_t = String::from("0.0.0.0:3000"))]
    pub address: String,

    #[arg(short, long)]
    pub otlp_url: Option<String>,
}

impl Into<Config> for StartCommandArgs {
    fn into(self) -> Config {
        Config::new(
            Some(self.num_threads),
            Some(self.log_level),
            Some(self.address),
            self.otlp_url,
        )
    }
}

#[derive(Debug, Parser)]
#[command(author,version,about,long_about = None)]
pub enum Cli {
    Start(StartCommandArgs),

    #[clap(subcommand)]
    Create(CreateCommands),
}
