//! Hubify Cli
//
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
mod k8s;

#[derive(Debug, Parser)]
#[command(author,version,about,long_about = None)]
pub enum Cli {
    #[clap(subcommand, about = "Create a new resource")]
    Create(CreateCommands),
}

#[derive(Args, Debug)]
pub struct ApplicationArgs {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    url: url::Url,
}

// TODO: generate application
// impl Into<Application> for ApplicationArgs {
//     fn into(self) -> Application {
//         let spec = ApplicationSpec {
//             name: self.name.clone(),
//             version: None,
//             description: None,
//             icon: None,
//             url: self.url,
//             categories: None,
//             openapi_endpoint: None,
//             repository: None,
//         };
//         Application::new(&self.name, spec)
//     }
// }

#[derive(Debug, Subcommand)]
pub enum CreateCommands {
    CRD,
    Application(ApplicationArgs),
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    match Cli::parse() {
        Cli::Create(cmds) => {
            match cmds {
                CreateCommands::CRD => {
                    let text = crate::k8s::generate_resources(None)?;
                    println!("{text}");
                }
                CreateCommands::Application(_args) => {
                    // let mut text = serde_yaml::to_string(&Into::<Application>::into(args)).unwrap();
                    // println!("{text}");
                }
            };
        }
    };
    Ok(())
}
