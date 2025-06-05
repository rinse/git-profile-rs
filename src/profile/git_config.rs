use crate::error::GitProfileError;

pub trait GitConfig {
    fn set_include_path(&mut self, path: &str) -> Result<(), GitProfileError>;
}
