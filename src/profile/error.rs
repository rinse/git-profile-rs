use thiserror::Error;

/// Errors that can occur in git-profile-rs
#[derive(Error, Debug)]
pub enum GitProfileError {
    #[error("Failed to open git repository")]
    RepositoryOpen(#[source] git2::Error),

    #[error("Failed to access git configuration")]
    ConfigAccess(#[source] git2::Error),

    #[error("Environment variable error: {variable}")]
    Environment { variable: String },

    #[error("Invalid profile name: {path}")]
    ProfilePath { path: String },

    #[error("Profile '{name}' not found at {path}")]
    ProfileNotFound { name: String, path: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
