use std::path::PathBuf;

pub trait ConfigDir {
    fn path(&self) -> PathBuf;
}
