mod parser;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tilth")]
#[command(about = "A safe and interactive wrapper for Terraform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Inspect {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Inspect { path } => {
            println!("Inspecting directory: {}", path.display());

            let variables = parser::discover_variables(path);

            println!("Discovered Variables: {:#?}", variables);
        }
    }
}
