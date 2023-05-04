///! Xtask
///
mod scripts;
mod utils;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use scripts::Scripts;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Subcommand)]
pub enum WatchArgs {
    Tilt,
    Web,
}

#[derive(Debug, Parser)]
pub enum Cli {
    #[clap(name = "watch", about = "Watch the project")]
    Watch {
        #[clap(subcommand)]
        args: WatchArgs,
    },

    #[clap(name = "build", about = "Build the project")]
    Build(BuildArgs),
    Deploy(DeployArgs),
    #[clap(name = "scripts", about = "Run scripts")]
    Scripts {
        #[clap(subcommand)]
        args: Scripts,
    },
}

#[derive(Debug, Args)]
pub struct BuildArgs {}

#[derive(Debug, Args)]
pub struct DeployArgs {}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Build(BuildArgs),
    Deploy(DeployArgs),
}

pub fn watch(args: WatchArgs) -> Result<()> {
    match args {
        WatchArgs::Tilt => {
            if !crate::utils::has_cluster("hubify")? {
                log::warn!("cluster not found, creating one");
                print!("---------------");
                crate::scripts::create_cluster("hubify", "registry", "5000")?;
                log::info!("End of cluster creation");
                print!("---------------");
            } else {
                log::info!("Cluster found");
            }
            // check if Tiltfile file exists rollback one directory and check again
            let tiltfile = Path::new("./Tiltfile");
            if !tiltfile.exists() {
                return Err(anyhow::anyhow!("Tiltfile not found"));
            }
            log::info!("Tiltfile found");

            // start tilt command with tilt up
            let mut tilt = Command::new("tilt")
                .arg("up")
                .arg("-f")
                .arg(tiltfile)
                .spawn()
                .expect("failed to execute child");
            tilt.wait()?;
        }

        WatchArgs::Web => todo!(),
    };
    Ok(())
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let cli = Cli::parse();
    match cli {
        Cli::Watch { args } => watch(args)?,
        Cli::Build(_) => log::warn!("build is not implemented yet"),
        Cli::Deploy(_) => log::warn!("deploy is not implemented yet"),
        Cli::Scripts { args } => scripts::parse(args)?,
    };
    Ok(())
}
