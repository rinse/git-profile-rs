use crate::error::GitProfileError;

pub trait GitConfig {
    fn set_include_path(&mut self, path: &str) -> Result<(), GitProfileError>;
    fn get_include_paths(&self) -> Result<Vec<String>, GitProfileError>;
}
