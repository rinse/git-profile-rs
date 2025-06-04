mod cli;
mod error;
mod profile;

use anyhow::Context;
use clap::Parser;
use cli::{Cli, Commands};
use crate::profile::git_config_git2::Git2Config;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Switch { profile_name, global } => {
            let open_config = if global { Git2Config::open_global } else { Git2Config::open_local };
            let mut config = open_config()
                .with_context(|| format!("Failed to open {} git configuration", if global { "global" } else { "local" }))?;
            let profile_dir = get_profile_dir()?;
            profile::switch::switch(&profile_name, global, &profile_dir, &mut config)
                .with_context(|| format!("Failed to switch to profile '{}'", profile_name))?;
        }
    }
    Ok(())
}

fn get_profile_dir() -> anyhow::Result<String> {
    let xdg_config = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        xdg_config
    } else {
        let home = std::env::var("HOME")
            .with_context(|| "HOME environment variable is not set")?;
        format!("{}/.config", home)
    };
    Ok(format!("{}/git-profile", xdg_config))
}
