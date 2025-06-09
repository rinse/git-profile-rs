use crate::error::GitProfileError;
use crate::profile::git_config::GitConfig;
use std::fs;
use std::path::{Path, PathBuf};

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
    let is_current = current_include_paths.contains(&path_string);
    Some((name, path_string, is_current))
}
