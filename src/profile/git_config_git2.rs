use crate::git_config::GitConfig;
use crate::profile::error::GitProfileError;
use git2::{Config, ConfigLevel, Repository};

pub struct GitConfigGit2 {
    config: Config,
}

impl GitConfigGit2 {
    pub fn open() -> Result<Self, GitProfileError> {
        let repo = Repository::discover(".").map_err(GitProfileError::RepositoryOpen)?;
        let config = repo.config().map_err(GitProfileError::ConfigAccess)?;
        let config = config
            .open_level(ConfigLevel::Local)
            .map_err(GitProfileError::ConfigAccess)?;
        Ok(GitConfigGit2 { config })
    }

    pub fn open_optional() -> Option<Self> {
        let repo = Repository::discover(".").ok()?;
        let config = repo.config().ok()?;
        let config = config.open_level(ConfigLevel::Local).ok()?;
        Some(GitConfigGit2 { config })
    }
}

impl GitConfig for GitConfigGit2 {
    fn add_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
        self.config
            .set_multivar("include.path", "^$", path)
            .map_err(GitProfileError::ConfigAccess)
    }

    fn remove_include_path(&mut self, path: &str) -> Result<(), GitProfileError> {
        let pattern = format!("^{}$", escape_regex(path));
        self.config
            .remove_multivar("include.path", &pattern)
            .map_err(GitProfileError::ConfigAccess)
    }

    fn get_include_paths(&self) -> Result<Vec<String>, GitProfileError> {
        let mut paths = Vec::new();
        let mut entries = match self.config.entries(Some("include.path")) {
            Ok(entries) => entries,
            Err(e) if e.code() == git2::ErrorCode::NotFound => return Ok(paths),
            Err(e) => return Err(GitProfileError::ConfigAccess(e)),
        };
        while let Some(entry) = entries.next() {
            match entry {
                Ok(entry) => {
                    if let Some(value) = entry.value() {
                        paths.push(value.to_string());
                    }
                }
                Err(e) => return Err(GitProfileError::ConfigAccess(e)),
            }
        }
        Ok(paths)
    }
}

fn escape_regex(path: &str) -> String {
    let mut escaped = String::with_capacity(path.len());
    for c in path.chars() {
        if matches!(
            c,
            '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$'
        ) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_test_config() -> (tempfile::TempDir, GitConfigGit2) {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let repo = git2::Repository::init(dir.path()).expect("failed to init repo");
        let config = repo.config().expect("failed to get repo config");
        let config = config
            .open_level(ConfigLevel::Local)
            .expect("failed to open local config");
        (dir, GitConfigGit2 { config })
    }

    #[test]
    fn add_and_get_include_path_round_trip() {
        let (_dir, mut config) = new_test_config();
        config.add_include_path("/a/b/work.gitconfig").unwrap();
        let paths = config.get_include_paths().unwrap();
        assert_eq!(paths, vec!["/a/b/work.gitconfig".to_string()]);
    }

    #[test]
    fn remove_include_path_removes_added_path() {
        let (_dir, mut config) = new_test_config();
        config.add_include_path("/a/b/work.gitconfig").unwrap();
        config.remove_include_path("/a/b/work.gitconfig").unwrap();
        let paths = config.get_include_paths().unwrap();
        assert!(paths.is_empty());
    }

    #[test]
    fn remove_include_path_does_not_remove_unrelated_substring_match() {
        let (_dir, mut config) = new_test_config();
        config
            .add_include_path("/backup/a/b/work.gitconfig")
            .unwrap();
        config.add_include_path("/a/b/work.gitconfig").unwrap();
        config.remove_include_path("/a/b/work.gitconfig").unwrap();
        let paths = config.get_include_paths().unwrap();
        assert_eq!(paths, vec!["/backup/a/b/work.gitconfig".to_string()]);
    }

    #[test]
    fn get_include_paths_returns_empty_when_none_set() {
        let (_dir, config) = new_test_config();
        let paths = config.get_include_paths().unwrap();
        assert!(paths.is_empty());
    }

    #[test]
    fn remove_include_path_with_regex_metacharacters() {
        let (_dir, mut config) = new_test_config();
        config
            .add_include_path("/a/dir+name/pro(file).gitconfig")
            .unwrap();
        config
            .remove_include_path("/a/dir+name/pro(file).gitconfig")
            .unwrap();
        let paths = config.get_include_paths().unwrap();
        assert!(paths.is_empty());
    }
}
