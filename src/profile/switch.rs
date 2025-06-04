use git2::{Config, Repository};
use std::env;

pub fn switch(profile_name: &str, global: bool) -> Result<(), Box<dyn std::error::Error>> {
    let xdg_config = get_xdg_config_dir()?;
    let profile_path = format!("{}/git-profile/{}.gitconfig", xdg_config, profile_name);
    if global {
        let mut config = Config::open_default()?;
        config.set_str("include.path", &profile_path)?;
        println!("Global git profile switched to: {}", profile_name);
    } else {
        let repo = Repository::open(".")?;
        let mut config = repo.config()?;
        config.set_str("include.path", &profile_path)?;
        println!("Local git profile switched to: {}", profile_name);
    }
    Ok(())
}

fn get_xdg_config_dir() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config)
    } else {
        let home = env::var("HOME")?;
        Ok(format!("{}/.config", home))
    }
}
