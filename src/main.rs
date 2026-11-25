use clap::{Parser, Subcommand};
use std::{eprintln, error::Error, format};
use cliclack::{confirm, intro,note, outro, outro_cancel};

mod config;
mod prompt;
mod generate;
use prompt::ask_config;
use generate::generate;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { project_name: Option<String> },
}

fn main() {
    let cli: Cli = Cli::parse();
    if let Err(e) = run(cli) {
        match outro_cancel(format!("Error: {}", e)) {
            Ok(()) => {},
            Err(e) => {
                eprintln!("{e}")
            }
        }
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    intro("Let's create your project")?;
    let user_config = match cli.command {
        Commands::New { project_name } => ask_config(project_name)? 
    };

    note("Project Configuration",format!("{:#?}", user_config))?;
    let cf = confirm("Create the project?").initial_value(true).interact()?;
    if cf {
        generate(&user_config)?;
        outro("Done!")?;
    } else {
        outro("Canceled configuration")?;
    }
    Ok(())
}
