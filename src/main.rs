use clap::{Parser, Subcommand};
use std::{error::Error, println};

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
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let user_config = match cli.command {
        Commands::New { project_name } => ask_config(project_name)?,
    };

    println!("{:-^50}", "Project Configuration");
    println!("{:#?}", user_config);
    generate(&user_config)?;
    Ok(())
}
