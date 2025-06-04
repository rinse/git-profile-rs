mod cli;
mod profile;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Switch { profile_name, global } => {
            if let Err(e) = profile::switch::switch(&profile_name, global) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
