use crate::profile::git_config::GitConfig;

pub fn switch<T: GitConfig>(
    profile_name: &str,
    global: bool,
    profile_dir: &str,
    config: &mut T,
) -> anyhow::Result<()> {
    // Validate profile name doesn't contain path separators or other invalid characters
    if profile_name.contains('/') || profile_name.contains('\\') || 
       profile_name.contains('\0') || profile_name == "." || profile_name == ".." {
        return Err(crate::error::GitProfileError::ProfilePath { 
            path: profile_name.to_string() 
        }.into());
    }
    let profile_path = format!("{}/{}.gitconfig", profile_dir, profile_name);
    config.set_include_path(&profile_path)?;
    if global {
        println!("Global git profile switched to: {}", profile_name);
    } else {
        println!("Local git profile switched to: {}", profile_name);
    }
    Ok(())
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
        fn set_include_path(&mut self, path: &str) -> Result<(), crate::error::GitProfileError> {
            self.config
                .insert("include.path".to_string(), path.to_string());
            Ok(())
        }
    }

    #[test]
    fn test_switch_with_mock_config() {
        let mut mock_config = MockGitConfig::new();
        let result = switch(
            "testprofile",
            false,
            "/test/config/git-profile",
            &mut mock_config,
        );
        assert!(result.is_ok());
        assert_eq!(
            mock_config.get("include.path"),
            Some(&"/test/config/git-profile/testprofile.gitconfig".to_string())
        );
    }

    #[test]
    fn test_switch_global_flag() {
        let mut mock_config = MockGitConfig::new();
        let result = switch(
            "globalprofile",
            true,
            "/test/config/git-profile",
            &mut mock_config,
        );
        assert!(result.is_ok());
        assert_eq!(
            mock_config.get("include.path"),
            Some(&"/test/config/git-profile/globalprofile.gitconfig".to_string())
        );
    }

    #[test]
    fn test_invalid_profile_names() {
        let mut mock_config = MockGitConfig::new();
        
        // Test profile name with forward slash
        let result = switch("invalid/profile", false, "/test/config", &mut mock_config);
        assert!(result.is_err());
        
        // Test profile name with backslash
        let result = switch("invalid\\profile", false, "/test/config", &mut mock_config);
        assert!(result.is_err());
        
        // Test profile name with null character
        let result = switch("invalid\0profile", false, "/test/config", &mut mock_config);
        assert!(result.is_err());
        
        // Test "." as profile name
        let result = switch(".", false, "/test/config", &mut mock_config);
        assert!(result.is_err());
        
        // Test ".." as profile name
        let result = switch("..", false, "/test/config", &mut mock_config);
        assert!(result.is_err());
    }
}
