use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Configuration for cwrdd-make
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the cwrdd repository root
    pub repo_path: PathBuf,
}

impl Config {
    /// Default repository path
    const DEFAULT_REPO_PATH: &'static str = "~/Projects/cwrdd";

    /// Config file name (placed alongside the cwrdd-make binary)
    const CONFIG_FILE_NAME: &'static str = "cwrdd-make-config.toml";

    /// Load configuration from file or use defaults
    pub fn load() -> Result<Self> {
        // Try to load from config file next to binary
        if let Some(config_path) = Self::config_file_path() {
            if config_path.exists() {
                return Self::load_from_file(&config_path);
            }
        }

        // Fall back to default
        Ok(Self::default())
    }

    /// Load configuration from a specific file
    fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    /// Get the path to the config file (next to the binary)
    fn config_file_path() -> Option<PathBuf> {
        let exe_path = std::env::current_exe().ok()?;
        let exe_dir = exe_path.parent()?;
        Some(exe_dir.join(Self::CONFIG_FILE_NAME))
    }

    /// Get the path to the app directory
    pub fn app_path(&self) -> PathBuf {
        self.repo_path.join("app")
    }

    /// Get the path to the db directory
    pub fn db_path(&self) -> PathBuf {
        self.repo_path.join("db")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_path: expand_tilde(Self::DEFAULT_REPO_PATH),
        }
    }
}

/// Expand ~ to home directory
fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.repo_path.to_string_lossy().contains("Projects/cwrdd"));
    }

    #[test]
    fn test_expand_tilde() {
        let path = expand_tilde("~/test/path");
        assert!(!path.to_string_lossy().starts_with("~"));
    }

    #[test]
    fn test_app_path() {
        let config = Config::default();
        let app_path = config.app_path();
        assert!(app_path.ends_with("app"));
    }
}
