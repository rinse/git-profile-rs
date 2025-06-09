use crate::git_config::GitConfig;
use crate::profile::error::GitProfileError;
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
    fn set_include_path(
        &mut self,
        path: &str,
        profile_dir: &impl crate::config_dir::ConfigDir,
    ) -> Result<(), GitProfileError> {
        // Get all existing include paths
        let mut existing_paths = self.get_include_paths()?;
        // Remove any git-profile related paths (those under the profile directory)
        existing_paths.retain(|p| !std::path::Path::new(p).starts_with(profile_dir.path()));
        // Add the new path
        existing_paths.push(path.to_string());
        // Remove all include.path entries first
        self.config
            .remove_multivar("include.path", ".*")
            .map_err(GitProfileError::ConfigAccess)?;
        // Add all paths back as multivars
        for include_path in existing_paths {
            self.config
                .set_multivar("include.path", "^$", &include_path)
                .map_err(GitProfileError::ConfigAccess)?;
        }
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
