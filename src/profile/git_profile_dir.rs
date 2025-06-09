use crate::profile::error::GitProfileError;
use std::path::{Path, PathBuf};

pub trait GitProfileDir {
    fn path(&self) -> &Path;
}

pub struct DefaultGitProfileDir {
    path: PathBuf,
}

impl DefaultGitProfileDir {
    pub fn new() -> Result<Self, GitProfileError> {
        let xdg_config = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            xdg_config
        } else {
            let home = std::env::var("HOME").map_err(|_| GitProfileError::Environment {
                variable: "HOME".to_string(),
            })?;
            format!("{}/.config", home)
        };
        let path = PathBuf::from(format!("{}/git-profile", xdg_config));
        Ok(DefaultGitProfileDir { path })
    }
}

impl GitProfileDir for DefaultGitProfileDir {
    fn path(&self) -> &Path {
        &self.path
    }
}
