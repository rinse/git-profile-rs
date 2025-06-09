use crate::git_config::GitConfig;
use crate::profile::error::GitProfileError;
use git2::{Config, Repository};

pub struct GitConfigGit2 {
    config: Config,
}

impl GitConfigGit2 {
    pub fn open_global() -> Result<Self, GitProfileError> {
        let config = Config::open_default().map_err(GitProfileError::ConfigAccess)?;
        Ok(GitConfigGit2 { config })
    }
    pub fn open_local() -> Result<Self, GitProfileError> {
        let repo = Repository::open(".")?;
        let config = repo.config().map_err(GitProfileError::ConfigAccess)?;
        Ok(GitConfigGit2 { config })
    }
}

impl GitConfig for GitConfigGit2 {
    fn add_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
        self.config
            .set_multivar("include.path", "^$", path)
            .map_err(GitProfileError::ConfigAccess)
    }

    fn remove_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
        self.config
            .remove_multivar("include.path", path)
            .map_err(GitProfileError::ConfigAccess)
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
