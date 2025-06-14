mod cli;
mod config_dir;
mod git_config;
mod profile;

use crate::config_dir::ConfigDir;
use crate::profile::config_dir_git_profile::ConfigDirGitProfile;
use crate::profile::git_config_git2::GitConfigGit2;
use anyhow::Context;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let profile_dir = ConfigDirGitProfile::new()?;
    match cli.command {
        Commands::Switch { profile_name } => switch(&profile_name, &profile_dir)?,
        Commands::List { verbose } => list(verbose, &profile_dir)?,
    }
    Ok(())
}

fn switch(profile_name: &str, profile_dir: &impl ConfigDir) -> anyhow::Result<()> {
    let mut config =
        GitConfigGit2::open().with_context(|| "Failed to open local git configuration")?;
    profile::switch::switch(profile_name, profile_dir, &mut config)
        .with_context(|| format!("Failed to switch to profile '{}'", profile_name))?;
    Ok(())
}

fn list(verbose: bool, profile_dir: &impl ConfigDir) -> anyhow::Result<()> {
    let config = GitConfigGit2::open().with_context(|| "Failed to open git configuration")?;
    let profiles = profile::list::list_profiles(&profile_dir.path(), &config)
        .with_context(|| "Failed to list profiles")?;
    for (name, path, is_current) in profiles {
        let marker = if is_current { "* " } else { "  " };
        if verbose {
            println!("{}{} -> {}", marker, name, path);
        } else {
            println!("{}{}", marker, name);
        }
    }
    Ok(())
}
