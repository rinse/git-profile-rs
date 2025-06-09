use crate::config_dir::ConfigDir;
use crate::profile::error::GitProfileError;
use std::path::PathBuf;

pub struct ConfigDirGitProfile {
    path: PathBuf,
}

impl ConfigDirGitProfile {
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
        Ok(ConfigDirGitProfile { path })
    }
}

impl ConfigDir for ConfigDirGitProfile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
