use super::git_profile_dir::GitProfileDir;
use crate::error::GitProfileError;

pub trait GitConfig {
    fn set_include_path(
        &mut self,
        path: &str,
        profile_dir: &impl GitProfileDir,
    ) -> Result<(), GitProfileError>;
    fn get_include_paths(&self) -> Result<Vec<String>, GitProfileError>;
}
