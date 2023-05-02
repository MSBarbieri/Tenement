mod cli;
mod config;
mod crd;
mod logger;
mod routes;
mod server;
mod tracer;

use clap::Parser;
use server::ServerError;
use thiserror::Error;
use tracer::TracingError;

use crate::cli::{Cli, CliError};
#[derive(Error, Debug)]
pub enum StartError {
    #[error("CliError Invalid with error: '{0}")]
    CliError(#[from] CliError),
    #[error("start server error '{0}")]
    ServerError(#[from] ServerError),
    #[error("start tracer error '{0}'")]
    TracerError(#[from] TracingError),
}

#[tokio::main]
async fn main() -> Result<(), StartError> {
    match Cli::parse() {
        Cli::Start(cli) => {
            let mut config = cli.into();
            tracer::setup_tracing(&mut config)?;

            tracing::debug!("Cli Validated, starting server");
            server::create_server(config).await?;
        }
        Cli::Create(commands) => {}
    };
    Ok(())
}
