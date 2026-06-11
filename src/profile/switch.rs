use crate::config_dir::ConfigDir;
use crate::git_config::GitConfig;
use crate::profile::error::GitProfileError;

pub fn switch<T: GitConfig, U: ConfigDir>(
    profile_name: &str,
    profile_dir: &U,
    config: &mut T,
) -> anyhow::Result<()> {
    validate_profile_name(profile_name)?;
    let profile_path_buf = profile_dir
        .path()
        .join(format!("{}.gitconfig", profile_name));
    if !profile_path_buf.exists() {
        return Err(GitProfileError::ProfileNotFound {
            name: profile_name.to_string(),
            path: profile_path_buf.display().to_string(),
        }
        .into());
    }
    // Convert Windows backslashes to forward slashes for Git include path compatibility
    let profile_path = profile_path_buf.display().to_string().replace("\\", "/");
    let existing_paths = config.get_include_paths()?;
    for path in &existing_paths {
        if std::path::Path::new(path).starts_with(profile_dir.path()) {
            config.remove_include_path(path)?;
        }
    }
    config.add_include_path(&profile_path)?;
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
        fn new(path: &std::path::Path) -> Self {
            MockGitProfileDir {
                path: path.to_path_buf(),
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
        fn add_include_path(
            &mut self,
            path: &str,
        ) -> Result<(), crate::profile::error::GitProfileError> {
            self.include_paths.push(path.to_string());
            Ok(())
        }

        fn remove_include_path(
            &mut self,
            path: &str,
        ) -> Result<(), crate::profile::error::GitProfileError> {
            self.include_paths.retain(|p| p != path);
            Ok(())
        }

        fn get_include_paths(&self) -> Result<Vec<String>, crate::profile::error::GitProfileError> {
            Ok(self.include_paths.clone())
        }
    }

    #[test]
    fn test_switch_with_mock_config() {
        let temp_dir = tempfile::tempdir().unwrap();
        std::fs::write(temp_dir.path().join("testprofile.gitconfig"), "").unwrap();
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new(temp_dir.path());
        let result = switch("testprofile", &mock_profile_dir, &mut mock_config);
        assert!(result.is_ok());
        assert_eq!(
            mock_config.get("include.path"),
            Some(
                &temp_dir
                    .path()
                    .join("testprofile.gitconfig")
                    .display()
                    .to_string()
                    .replace("\\", "/")
            )
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
        let temp_dir = tempfile::tempdir().unwrap();
        std::fs::write(temp_dir.path().join("work.gitconfig"), "").unwrap();
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new(temp_dir.path());
        // Set up existing includes
        mock_config
            .include_paths
            .push("/path/to/delta.gitconfig".to_string());
        mock_config
            .include_paths
            .push("/another/config.gitconfig".to_string());
        let result = switch("work", &mock_profile_dir, &mut mock_config);
        assert!(result.is_ok());
        // Check that other includes are preserved
        let paths = mock_config.get_include_paths().unwrap();
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], "/path/to/delta.gitconfig");
        assert_eq!(paths[1], "/another/config.gitconfig");
        assert_eq!(
            paths[2],
            temp_dir
                .path()
                .join("work.gitconfig")
                .display()
                .to_string()
                .replace("\\", "/")
        );
    }

    #[test]
    fn test_switch_replaces_previous_git_profile() {
        let temp_dir = tempfile::tempdir().unwrap();
        std::fs::write(temp_dir.path().join("work.gitconfig"), "").unwrap();
        std::fs::write(temp_dir.path().join("personal.gitconfig"), "").unwrap();
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new(temp_dir.path());
        // Set up existing includes including a git-profile one
        mock_config
            .include_paths
            .push("/path/to/delta.gitconfig".to_string());
        mock_config.include_paths.push(
            temp_dir
                .path()
                .join("personal.gitconfig")
                .display()
                .to_string()
                .replace("\\", "/"),
        );
        let result = switch("work", &mock_profile_dir, &mut mock_config);
        assert!(result.is_ok());
        // Check that the old git-profile include is replaced
        let paths = mock_config.get_include_paths().unwrap();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], "/path/to/delta.gitconfig");
        assert_eq!(
            paths[1],
            temp_dir
                .path()
                .join("work.gitconfig")
                .display()
                .to_string()
                .replace("\\", "/")
        );
    }

    #[test]
    fn test_switch_with_invalid_profile_names() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new(temp_dir.path());

        // Test empty profile name
        let result = switch("", &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());

        // Test profile name with forward slash
        let result = switch("invalid/profile", &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());

        // Test profile name with backslash
        let result = switch("invalid\\profile", &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());

        // Test profile name with null character
        let result = switch("invalid\0profile", &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());

        // Test "." as profile name
        let result = switch(".", &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());

        // Test ".." as profile name
        let result = switch("..", &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_with_nonexistent_profile_returns_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut mock_config = MockGitConfig::new();
        let mock_profile_dir = MockGitProfileDir::new(temp_dir.path());
        mock_config
            .include_paths
            .push("/path/to/delta.gitconfig".to_string());
        let result = switch("missing", &mock_profile_dir, &mut mock_config);
        assert!(result.is_err());
        // Include paths must remain unchanged
        let paths = mock_config.get_include_paths().unwrap();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "/path/to/delta.gitconfig");
    }
}
