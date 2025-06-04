use git2::{Config, Repository};
use super::git_config::GitConfig;

pub struct Git2Config {
    config: Config,
}

impl Git2Config {
    pub fn open_global() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Git2Config {
            config: Config::open_default()?,
        })
    }
    pub fn open_local() -> Result<Self, Box<dyn std::error::Error>> {
        let repo = Repository::open(".")?;
        Ok(Git2Config {
            config: repo.config()?,
        })
    }
}

impl GitConfig for Git2Config {
    fn set_include_path(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.config.set_str("include.path", path)?;
        Ok(())
    }
}
