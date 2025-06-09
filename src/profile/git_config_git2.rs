use super::git_config::GitConfig;
use crate::error::GitProfileError;
use git2::{Config, Repository};

pub struct Git2Config {
    config: Config,
}

impl Git2Config {
    pub fn open_global() -> Result<Self, GitProfileError> {
        let config = Config::open_default().map_err(GitProfileError::ConfigAccess)?;
        Ok(Git2Config { config })
    }
    pub fn open_local() -> Result<Self, GitProfileError> {
        let repo = Repository::open(".")?;
        let config = repo.config().map_err(GitProfileError::ConfigAccess)?;
        Ok(Git2Config { config })
    }
}

impl GitConfig for Git2Config {
    fn set_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
        self.config
            .set_str("include.path", path)
            .map_err(GitProfileError::ConfigAccess)?;
        Ok(())
    }
    fn get_include_paths(&self) -> Result<Vec<String>, GitProfileError> {
        let mut paths = Vec::new();
        let mut entries = self
            .config
            .entries(Some("include.path"))
            .map_err(GitProfileError::ConfigAccess)?;
        while let Some(entry) = entries.next() {
            match entry {
                Ok(entry) => {
                    if let Some(value) = entry.value() {
                        paths.push(value.to_string());
                    }
                }
                Err(e) => return Err(GitProfileError::ConfigAccess(e)),
            }
        }
        Ok(paths)
    }
}
