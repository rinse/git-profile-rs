mod cli;
mod profile;

use clap::Parser;
use cli::{Cli, Commands};
use crate::profile::git_config_git2::Git2Config;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Switch { profile_name, global } => {
            let open_config = if global { Git2Config::open_global } else { Git2Config::open_local };
            let mut config = match open_config() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error opening global config: {}", e);
                    std::process::exit(1);
                }
            };
            if let Err(e) = profile::switch::switch(&profile_name, global, &mut config) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
