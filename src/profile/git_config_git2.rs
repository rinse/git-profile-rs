use git2::{Config, Repository};
use super::git_config::GitConfig;
use crate::error::{GitProfileError, Result};

pub struct Git2Config {
    config: Config,
}

impl Git2Config {
    pub fn open_global() -> Result<Self> {
        let config = Config::open_default()
            .map_err(|e| GitProfileError::ConfigAccess(e))?;
        Ok(Git2Config { config })
    }
    pub fn open_local() -> Result<Self> {
        let repo = Repository::open(".")?;
        let config = repo.config()
            .map_err(|e| GitProfileError::ConfigAccess(e))?;
        Ok(Git2Config { config })
    }
}

impl GitConfig for Git2Config {
    fn set_include_path(&mut self, path: &str) -> Result<()> {
        self.config.set_str("include.path", path)
            .map_err(|e| GitProfileError::ConfigAccess(e))?;
        Ok(())
    }
}
