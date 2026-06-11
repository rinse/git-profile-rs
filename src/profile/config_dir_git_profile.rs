use crate::config_dir::ConfigDir;
use crate::profile::error::GitProfileError;
use std::path::PathBuf;

pub struct ConfigDirGitProfile {
    path: PathBuf,
}

impl ConfigDirGitProfile {
    pub fn new() -> Result<Self, GitProfileError> {
        let xdg_config_home = std::env::var("XDG_CONFIG_HOME").ok();
        let home = std::env::var("HOME").ok();
        let path = resolve_profile_dir(xdg_config_home, home)?;
        Ok(ConfigDirGitProfile { path })
    }
}

impl ConfigDir for ConfigDirGitProfile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

fn resolve_profile_dir(
    xdg_config_home: Option<String>,
    home: Option<String>,
) -> Result<PathBuf, GitProfileError> {
    let config_dir = match xdg_config_home {
        Some(xdg_config_home) if !xdg_config_home.is_empty() => PathBuf::from(xdg_config_home),
        _ => {
            let home = home.ok_or_else(|| GitProfileError::Environment {
                variable: "HOME".to_string(),
            })?;
            PathBuf::from(home).join(".config")
        }
    };
    Ok(config_dir.join("git-profile"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xdg_config_home_set_and_non_empty_is_used_as_is() {
        let path = resolve_profile_dir(
            Some("/custom/xdg".to_string()),
            Some("/home/user".to_string()),
        )
        .unwrap();
        assert_eq!(path, PathBuf::from("/custom/xdg/git-profile"));
    }

    #[test]
    fn xdg_config_home_unset_falls_back_to_home_config() {
        let path = resolve_profile_dir(None, Some("/home/user".to_string())).unwrap();
        assert_eq!(path, PathBuf::from("/home/user/.config/git-profile"));
    }

    #[test]
    fn xdg_config_home_empty_falls_back_to_home_config() {
        let path =
            resolve_profile_dir(Some(String::new()), Some("/home/user".to_string())).unwrap();
        assert_eq!(path, PathBuf::from("/home/user/.config/git-profile"));
    }

    #[test]
    fn both_unset_returns_environment_error() {
        let result = resolve_profile_dir(None, None);
        assert!(matches!(
            result,
            Err(GitProfileError::Environment { variable }) if variable == "HOME"
        ));
    }
}
