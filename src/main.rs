mod cli;
mod error;
mod profile;

use crate::profile::git_config_git2::Git2Config;
use anyhow::Context;
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
            let mut config = open_config().with_context(|| {
                format!(
                    "Failed to open {} git configuration",
                    if global { "global" } else { "local" }
                )
            })?;
            let profile_dir = get_profile_dir()?;
            profile::switch::switch(&profile_name, global, &profile_dir, &mut config)
                .with_context(|| format!("Failed to switch to profile '{}'", profile_name))?;
        }
        Commands::List { verbose } => {
            let profile_dir = get_profile_dir()?;
            let config = Git2Config::open_local()
                .or_else(|_| Git2Config::open_global())
                .with_context(|| "Failed to open git configuration")?;
            let profiles =
                profile::list::list_profiles(std::path::Path::new(&profile_dir), &config)
                    .with_context(|| "Failed to list profiles")?;
            for (name, path, is_current) in profiles {
                let marker = if is_current { "* " } else { "  " };
                if verbose {
                    println!("{}{} -> {}", marker, name, path);
                } else {
                    println!("{}{}", marker, name);
                }
            }
        }
    }
    Ok(())
}

fn get_profile_dir() -> Result<String, crate::error::GitProfileError> {
    let xdg_config = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        xdg_config
    } else {
        let home =
            std::env::var("HOME").map_err(|_| crate::error::GitProfileError::Environment {
                variable: "HOME".to_string(),
            })?;
        format!("{}/.config", home)
    };
    Ok(format!("{}/git-profile", xdg_config))
}
