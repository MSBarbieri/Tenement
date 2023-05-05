///! Hubtify Cli
///
use clap::{Args, Parser, Subcommand, ValueEnum};
use kube::CustomResourceExt;

#[derive(Debug, Parser)]
#[command(author,version,about,long_about = None)]
pub enum Cli {
    #[clap(subcommand, about = "Create a new resource")]
    Create(CreateCommands),
}

#[derive(Debug, Subcommand)]
pub enum CreateCommands {
    CRD,
    Application,
}

fn main() {
    match Cli::parse() {
        Cli::Create(cmds) => {
            match cmds {
                CreateCommands::CRD => {
                    let text = serde_yaml::to_string(&vec![
                        common::crd::Application::crd(),
                        common::crd::Command::crd(),
                    ])
                    .unwrap();
                    println!("{text}");
                }
                CreateCommands::Application => todo!(),
            };
        }
    };
}
