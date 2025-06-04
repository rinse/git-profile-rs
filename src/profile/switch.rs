use std::env;
use crate::profile::git_config::GitConfig;

pub fn switch<T: GitConfig>(profile_name: &str, global: bool, config: &mut T) -> Result<(), Box<dyn std::error::Error>> {
    let xdg_config = get_xdg_config_dir()?;
    let profile_path = format!("{}/git-profile/{}.gitconfig", xdg_config, profile_name);
    config.set_include_path(&profile_path)?;
    if global {
        println!("Global git profile switched to: {}", profile_name);
    } else {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct MockGitConfig {
        config: HashMap<String, String>,
    }

    impl MockGitConfig {
        fn new() -> Self {
            MockGitConfig {
                config: HashMap::new(),
            }
        }

        fn get(&self, key: &str) -> Option<&String> {
            self.config.get(key)
        }
    }

    impl GitConfig for MockGitConfig {
        fn set_include_path(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
            self.config.insert("include.path".to_string(), path.to_string());
            Ok(())
        }
    }

    #[test]
    fn test_switch_with_mock_config() {
        env::remove_var("XDG_CONFIG_HOME");
        let mut mock_config = MockGitConfig::new();
        let result = switch("testprofile", false, &mut mock_config);
        assert!(result.is_ok());
        let expected_path = format!("{}/.config/git-profile/testprofile.gitconfig", env::var("HOME").unwrap());
        assert_eq!(mock_config.get("include.path"), Some(&expected_path));
    }

    #[test]
    fn test_switch_global_flag() {
        env::remove_var("XDG_CONFIG_HOME");
        let mut mock_config = MockGitConfig::new();
        let result = switch("globalprofile", true, &mut mock_config);
        assert!(result.is_ok());
        let expected_path = format!("{}/.config/git-profile/globalprofile.gitconfig", env::var("HOME").unwrap());
        assert_eq!(mock_config.get("include.path"), Some(&expected_path));
    }

    #[test]
    fn test_switch_with_xdg_config_home() {
        env::set_var("XDG_CONFIG_HOME", "/custom/config");
        let mut mock_config = MockGitConfig::new();
        let result = switch("xdgprofile", false, &mut mock_config);
        assert!(result.is_ok());
        assert_eq!(mock_config.get("include.path"), Some(&"/custom/config/git-profile/xdgprofile.gitconfig".to_string()));
        env::remove_var("XDG_CONFIG_HOME");
    }
}
