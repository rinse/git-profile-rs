use crate::profile::error::GitProfileError;
use crate::git_config::GitConfig;
use crate::config_dir::ConfigDir;

pub fn switch<T: GitConfig, U: ConfigDir>(
    profile_name: &str,
    global: bool,
    profile_dir: &U,
    config: &mut T,
) -> anyhow::Result<()> {
    validate_profile_name(profile_name)?;
    let profile_path = format!(
        "{}/{}.gitconfig",
        profile_dir.path().display(),
        profile_name
    );
    let existing_paths = config.get_include_paths()?;
    for path in &existing_paths {
        if std::path::Path::new(path).starts_with(profile_dir.path()) {
            config.remove_include_path(path)?;
        }
    }
    config.add_include_path(&profile_path)?;
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

    struct MockGitConfig {
        include_paths: Vec<String>,
    }

    struct MockGitProfileDir {
        path: std::path::PathBuf,
    }

    impl MockGitProfileDir {
        fn new(path: &str) -> Self {
            MockGitProfileDir {
                path: std::path::PathBuf::from(path),
            }
        }
    }

    impl ConfigDir for MockGitProfileDir {
        fn path(&self) -> std::path::PathBuf {
            self.path.clone()
        }
    }

    impl MockGitConfig {
        fn new() -> Self {
            MockGitConfig {
                include_paths: Vec::new(),
            }
        }

        fn get(&self, key: &str) -> Option<&String> {
            if key == "include.path" && !self.include_paths.is_empty() {
                self.include_paths.last()
            } else {
                None
            }
        }
    }

    impl GitConfig for MockGitConfig {
        fn add_include_path(&mut self, path: &str) -> Result<(), crate::profile::error::GitProfileError> {
            self.include_paths.push(path.to_string());
            Ok(())
        }

        fn remove_include_path(&mut self, path: &str) -> Result<(), crate::profile::error::GitProfileError> {
            self.include_paths.retain(|p| p != path);
            Ok(())
        }

        fn get_include_paths(&self) -> Result<Vec<String>, crate::profile::error::GitProfileError> {
            Ok(self.include_paths.clone())
        }
    }

    #[test]
    fn test_switch_with_mock_config() {
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new("/test/config/git-profile");
        let result = switch("testprofile", false, &mock_profile_dir, &mut mock_config);
        assert!(result.is_ok());
        assert_eq!(
            mock_config.get("include.path"),
            Some(&"/test/config/git-profile/testprofile.gitconfig".to_string())
        );
    }

    #[test]
    fn test_switch_global_flag() {
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new("/test/config/git-profile");
        let result = switch("globalprofile", true, &mock_profile_dir, &mut mock_config);
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
    fn test_switch_preserves_other_includes() {
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new("/home/user/.config/git-profile");
        // Set up existing includes
        mock_config
            .include_paths
            .push("/path/to/delta.gitconfig".to_string());
        mock_config
            .include_paths
            .push("/another/config.gitconfig".to_string());
        let result = switch("work", false, &mock_profile_dir, &mut mock_config);
        assert!(result.is_ok());
        // Check that other includes are preserved
        let paths = mock_config.get_include_paths().unwrap();
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], "/path/to/delta.gitconfig");
        assert_eq!(paths[1], "/another/config.gitconfig");
        assert_eq!(paths[2], "/home/user/.config/git-profile/work.gitconfig");
    }

    #[test]
    fn test_switch_replaces_previous_git_profile() {
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new("/home/user/.config/git-profile");
        // Set up existing includes including a git-profile one
        mock_config
            .include_paths
            .push("/path/to/delta.gitconfig".to_string());
        mock_config
            .include_paths
            .push("/home/user/.config/git-profile/personal.gitconfig".to_string());
        let result = switch("work", false, &mock_profile_dir, &mut mock_config);
        assert!(result.is_ok());
        // Check that the old git-profile include is replaced
        let paths = mock_config.get_include_paths().unwrap();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], "/path/to/delta.gitconfig");
        assert_eq!(paths[1], "/home/user/.config/git-profile/work.gitconfig");
    }

    #[test]
    fn test_switch_with_invalid_profile_names() {
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new("/test/config");

        // Test empty profile name
        let result = switch("", false, &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());

        // Test profile name with forward slash
        let result = switch(
            "invalid/profile",
            false,
            &mock_profile_dir,
            &mut mock_config,
        );
        assert!(result.is_err());

        // Test profile name with backslash
        let result = switch(
            "invalid\\profile",
            false,
            &mock_profile_dir,
            &mut mock_config,
        );
        assert!(result.is_err());

        // Test profile name with null character
        let result = switch(
            "invalid\0profile",
            false,
            &mock_profile_dir,
            &mut mock_config,
        );
        assert!(result.is_err());

        // Test "." as profile name
        let result = switch(".", false, &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());

        // Test ".." as profile name
        let result = switch("..", false, &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());
    }
}
