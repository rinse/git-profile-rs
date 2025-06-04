pub trait GitConfig {
    fn set_include_path(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
}
