use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "git-profile")]
#[command(about = "A Rust implementation for Git Profile management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Switch the git profile
    Switch {
        /// Profile name to switch to
        profile_name: String,
        /// Apply globally instead of locally
        #[arg(long, short)]
        global: bool,
    },
    /// List available git profiles
    List {
        /// Show verbose output with file paths
        #[arg(long, short)]
        verbose: bool,
    },
}
