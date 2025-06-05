mod cli;
mod error;
mod profile;

use anyhow::Context;
use crate::profile::git_config_git2::Git2Config;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
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
            let mut config = open_config()
                .with_context(|| format!("Failed to open {} git configuration", if global { "global" } else { "local" }))?;
            let profile_dir = get_profile_dir();
            profile::switch::switch(&profile_name, global, &profile_dir, &mut config)
                .with_context(|| format!("Failed to switch to profile '{}'", profile_name))?;
        }
    }
    Ok(())
}

fn get_profile_dir() -> String {
    let xdg_config = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        xdg_config
    } else {
        format!("{}/.config", std::env::var("HOME").unwrap())
    };
    format!("{}/git-profile", xdg_config)
}
