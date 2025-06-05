use super::git_config::GitConfig;
use crate::error::GitProfileError;
use git2::{Config, Repository};

pub struct Git2Config {
    config: Config,
}

impl Git2Config {
    pub fn open_global() -> Result<Self, GitProfileError> {
        let config = Config::open_default()
            .map_err(GitProfileError::ConfigAccess)?;
        Ok(Git2Config { config })
    }
    pub fn open_local() -> Result<Self, GitProfileError> {
        let repo = Repository::open(".")?;
        let config = repo.config()
            .map_err(GitProfileError::ConfigAccess)?;
        Ok(Git2Config { config })
    }
}

impl GitConfig for Git2Config {
    fn set_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
        self.config.set_str("include.path", path)
            .map_err(GitProfileError::ConfigAccess)?;
        Ok(())
    }
}
