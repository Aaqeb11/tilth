mod executor;
mod parser;
mod prompter;

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
    Prompt {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Plan {
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Extra arguments to pass directly to terraform (e.g., -- -target=...)
        #[arg(last = true)]
        extra_args: Vec<String>,
    },
    Apply {
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Extra arguments to pass directly to terraform (e.g., -- -target=...)
        #[arg(last = true)]
        extra_args: Vec<String>,
    },
    Destroy {
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Extra arguments to pass directly to terraform (e.g., -- -target=...)
        #[arg(last = true)]
        extra_args: Vec<String>,
    },
}

fn execute_command(command: &str, path: &PathBuf, extra_args: &[String]) {
    let variables = parser::discover_variables(path);
    
    let mut answers = std::collections::HashMap::new();
    
    // Only prompt if we actually discovered any variables
    if !variables.is_empty() {
        answers = prompter::prompt_for_variables(variables);
    }

    if let Err(e) = executor::run_terraform(command, path, &answers, extra_args) {
        eprintln!("Failed to execute Terraform {}: {}", command, e);
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Inspect { path } => {
            println!("Inspecting directory: {}", path.display());

            let variables = parser::discover_variables(path);

            println!("Discovered Variables: {:#?}", variables);
        }
        Commands::Prompt { path } => {
            let variables = parser::discover_variables(path);
            if variables.is_empty() {
                println!("No variables found in this directory.");
                return;
            }

            let answers = prompter::prompt_for_variables(variables);
            println!("Final Answers: {:#?}", answers);
        }
        Commands::Plan { path, extra_args } => {
            execute_command("plan", path, extra_args);
        }
        Commands::Apply { path, extra_args } => {
            execute_command("apply", path, extra_args);
        }
        Commands::Destroy { path, extra_args } => {
            // For destroy, we'll eventually want to add an extra confirmation gate here
            // before we even prompt for variables or run the command.
            execute_command("destroy", path, extra_args);
        }
    }
}
