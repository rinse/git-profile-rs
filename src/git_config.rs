use crate::profile::error::GitProfileError;

pub trait GitConfig {
    fn add_include_path(&mut self, path: &str) -> Result<(), GitProfileError>;
    fn remove_include_path(&mut self, path: &str) -> Result<(), GitProfileError>;
    fn get_include_paths(&self) -> Result<Vec<String>, GitProfileError>;
}
