use crate::error::Result;

pub trait GitConfig {
    fn set_include_path(&mut self, path: &str) -> Result<()>;
}
