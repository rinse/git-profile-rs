use crate::error::GitProfileError;
use crate::profile::git_config::GitConfig;
use std::fs;
use std::path::Path;

pub fn list_profiles(
    profile_dir: &Path,
    config: &impl GitConfig,
) -> Result<Vec<(String, String, bool)>, GitProfileError> {
    if !profile_dir.exists() {
        return Ok(Vec::new());
    }
    let current_include_paths = config.get_include_paths()?;
    let entries = fs::read_dir(profile_dir).map_err(|e| {
        GitProfileError::ConfigError(format!("Failed to read profile directory: {}", e))
    })?;
    let mut profiles = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| {
            GitProfileError::ConfigError(format!("Failed to read directory entry: {}", e))
        })?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "gitconfig" {
                    if let Some(stem) = path.file_stem() {
                        if let Some(name) = stem.to_str() {
                            let path_string = path.to_string_lossy().to_string();
                            let is_current = current_include_paths.contains(&path_string);
                            profiles.push((name.to_string(), path_string, is_current));
                        }
                    }
                }
            }
        }
    }
    profiles.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(profiles)
}
