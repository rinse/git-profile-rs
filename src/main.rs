mod cli;
mod profile;

use crate::profile::git_config_git2::Git2Config;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Switch {
            profile_name,
            global,
        } => {
            let open_config = if global {
                Git2Config::open_global
            } else {
                Git2Config::open_local
            };
            let mut config = match open_config() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error opening global config: {}", e);
                    std::process::exit(1);
                }
            };
            let profile_dir = get_profile_dir();
            if let Err(e) =
                profile::switch::switch(&profile_name, global, &profile_dir, &mut config)
            {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn get_profile_dir() -> String {
    let xdg_config = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        xdg_config
    } else {
        format!("{}/.config", std::env::var("HOME").unwrap())
    };
    format!("{}/git-profile", xdg_config)
}
