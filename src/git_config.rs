use crate::config_dir::ConfigDir;
use crate::profile::error::GitProfileError;

pub trait GitConfig {
    fn set_include_path(
        &mut self,
        path: &str,
        profile_dir: &impl ConfigDir,
    ) -> Result<(), GitProfileError>;
    fn get_include_paths(&self) -> Result<Vec<String>, GitProfileError>;
}
