use thiserror::Error;

/// Errors that can occur in git-profile-rs
#[derive(Error, Debug)]
pub enum GitProfileError {
    #[error("Failed to open git repository: {0}")]
    RepositoryOpen(#[from] git2::Error),
    
    #[error("Failed to access git configuration")]
    ConfigAccess(#[source] git2::Error),
    
    #[error("Environment variable error: {variable}")]
    Environment { variable: String },
    
    #[error("Profile path error: {path}")]
    ProfilePath { path: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}