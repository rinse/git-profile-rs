use crate::error::GitProfileError;
use crate::profile::git_config::GitConfig;

pub fn switch<T: GitConfig>(
    profile_name: &str,
    global: bool,
    profile_dir: &str,
    config: &mut T,
) -> anyhow::Result<()> {
    validate_profile_name(profile_name)?;
    let profile_path = format!("{}/{}.gitconfig", profile_dir, profile_name);
    config.set_include_path(&profile_path)?;
    if global {
        println!("Global git profile switched to: {}", profile_name);
    } else {
        println!("Local git profile switched to: {}", profile_name);
    }
    Ok(())
}

fn validate_profile_name(profile_name: &str) -> Result<(), GitProfileError> {
    if profile_name.is_empty()
        || profile_name.contains('/')
        || profile_name.contains('\\')
        || profile_name.contains('\0')
        || profile_name == "."
        || profile_name == ".."
    {
        return Err(GitProfileError::ProfilePath {
            path: profile_name.to_string(),
        });
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
        fn get_include_paths(&self) -> Result<Vec<String>, crate::error::GitProfileError> {
            Ok(self
                .config
                .get("include.path")
                .map(|p| vec![p.clone()])
                .unwrap_or_default())
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
    fn test_validate_profile_name() {
        // Valid profile names
        assert!(validate_profile_name("work").is_ok());
        assert!(validate_profile_name("personal").is_ok());
        assert!(validate_profile_name("project-123").is_ok());
        assert!(validate_profile_name("my_profile").is_ok());

        // Invalid profile names
        assert!(validate_profile_name("").is_err());
        assert!(validate_profile_name("invalid/profile").is_err());
        assert!(validate_profile_name("invalid\\profile").is_err());
        assert!(validate_profile_name("invalid\0profile").is_err());
        assert!(validate_profile_name(".").is_err());
        assert!(validate_profile_name("..").is_err());
    }

    #[test]
    fn test_switch_with_invalid_profile_names() {
        let mut mock_config = MockGitConfig::new();

        // Test empty profile name
        let result = switch("", false, "/test/config", &mut mock_config);
        assert!(result.is_err());

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
