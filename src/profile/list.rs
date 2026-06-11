use crate::git_config::GitConfig;
use crate::profile::error::GitProfileError;
use std::fs;
use std::path::{Path, PathBuf};

pub fn list_profiles(
    profile_dir: &Path,
    config: Option<&impl GitConfig>,
) -> Result<Vec<(String, String, bool)>, GitProfileError> {
    if !profile_dir.exists() {
        return Ok(Vec::new());
    }
    let current_include_paths = config
        .map(|c| c.get_include_paths())
        .transpose()?
        .unwrap_or_default();
    let entries = fs::read_dir(profile_dir).map_err(|e| {
        GitProfileError::ConfigError(format!("Failed to read profile directory: {e}"))
    })?;
    let mut profiles = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| {
            GitProfileError::ConfigError(format!("Failed to read directory entry: {e}"))
        })?;
        if let Some(profile) = process_profile_entry(entry.path(), &current_include_paths) {
            profiles.push(profile);
        }
    }
    profiles.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(profiles)
}

fn process_profile_entry(
    path: PathBuf,
    current_include_paths: &[String],
) -> Option<(String, String, bool)> {
    if !path.is_file() {
        return None;
    }
    if path.extension()? != "gitconfig" {
        return None;
    }
    let name = path.file_stem()?.to_str()?.to_string();
    let path_string = path.to_string_lossy().to_string();
    let normalized_path = path_string.replace('\\', "/");
    let is_current = current_include_paths
        .iter()
        .any(|include_path| include_path.replace('\\', "/") == normalized_path);
    Some((name, path_string, is_current))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockGitConfig {
        include_paths: Vec<String>,
    }

    impl GitConfig for MockGitConfig {
        fn add_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
            self.include_paths.push(path.to_string());
            Ok(())
        }

        fn remove_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
            self.include_paths.retain(|p| p != path);
            Ok(())
        }

        fn get_include_paths(&self) -> Result<Vec<String>, GitProfileError> {
            Ok(self.include_paths.clone())
        }
    }

    #[test]
    fn test_list_profiles_returns_sorted_gitconfig_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("beta.gitconfig"), "").unwrap();
        fs::write(dir.path().join("alpha.gitconfig"), "").unwrap();
        fs::write(dir.path().join("notes.txt"), "").unwrap();
        fs::create_dir(dir.path().join("sub.gitconfig")).unwrap();
        let profiles = list_profiles(dir.path(), None::<&MockGitConfig>).unwrap();
        let names: Vec<&str> = profiles.iter().map(|p| p.0.as_str()).collect();
        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn test_is_current_matches_include_path() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("alpha.gitconfig"), "").unwrap();
        fs::write(dir.path().join("beta.gitconfig"), "").unwrap();
        let alpha_path = dir.path().join("alpha.gitconfig");
        let config = MockGitConfig {
            include_paths: vec![alpha_path.to_string_lossy().to_string()],
        };
        let profiles = list_profiles(dir.path(), Some(&config)).unwrap();
        let alpha = profiles.iter().find(|p| p.0 == "alpha").unwrap();
        let beta = profiles.iter().find(|p| p.0 == "beta").unwrap();
        assert!(alpha.2);
        assert!(!beta.2);
    }

    #[test]
    fn test_list_profiles_with_no_config_marks_none_current() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("alpha.gitconfig"), "").unwrap();
        fs::write(dir.path().join("beta.gitconfig"), "").unwrap();
        let profiles = list_profiles(dir.path(), None::<&MockGitConfig>).unwrap();
        assert!(profiles.iter().all(|p| !p.2));
    }

    #[test]
    fn test_list_profiles_returns_empty_for_missing_directory() {
        let dir = tempfile::tempdir().unwrap();
        let missing_dir = dir.path().join("does-not-exist");
        let profiles = list_profiles(&missing_dir, None::<&MockGitConfig>).unwrap();
        assert!(profiles.is_empty());
    }

    #[test]
    fn test_is_current_normalizes_backslashes() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("alpha.gitconfig"), "").unwrap();
        let alpha_path = dir.path().join("alpha.gitconfig");
        let forward_slash_path = alpha_path.to_string_lossy().replace('\\', "/");
        let config = MockGitConfig {
            include_paths: vec![forward_slash_path],
        };
        let profiles = list_profiles(dir.path(), Some(&config)).unwrap();
        let alpha = profiles.iter().find(|p| p.0 == "alpha").unwrap();
        assert!(alpha.2);
    }
}
