use crate::error::GitProfileError;
use std::fs;
use std::path::Path;

pub fn list_profiles(profile_dir: &Path) -> Result<Vec<(String, String)>, GitProfileError> {
    if !profile_dir.exists() {
        return Ok(Vec::new());
    }
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
                            profiles.push((name.to_string(), path.to_string_lossy().to_string()));
                        }
                    }
                }
            }
        }
    }
    profiles.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(profiles)
}
