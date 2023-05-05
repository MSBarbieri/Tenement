///! Hubify Controller
///
mod config;
mod logger;
mod routes;
mod server;
mod tracer;

use config::Config;
use envy::Error as EnvyError;
use server::ServerError;
use thiserror::Error;
use tracer::TracingError;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("start server error '{0}")]
    ServerError(#[from] ServerError),
    #[error("start tracer error '{0}'")]
    TracerError(#[from] TracingError),
    #[error("failed to start runtime'{0}'")]
    IoError(#[from] std::io::Error),
    #[error("yaml parse error'{0}'")]
    YamlParseError(#[from] serde_yaml::Error),
    #[error("load env config error'{0}'")]
    LoadEnvError(#[from] EnvyError),
}

async fn start(mut config: Config) -> Result<(), StartError> {
    tracer::setup_tracing(&mut config)?;
    server::create_server(config).await?;
    Ok(())
}

fn main() -> Result<(), StartError> {
    let config = envy::from_env::<Config>()?;
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.worker_threads(config.num_threads);
    builder.thread_name("hubify_controller");
    builder.enable_all();
    let rt = builder.build()?;
    rt.block_on(start(config))?;
    Ok(())
}
