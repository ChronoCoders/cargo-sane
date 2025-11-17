//! Configuration file handling

use crate::Result;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration for cargo-sane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Automatically update patch versions without prompting
    #[serde(default)]
    pub auto_update_patch: bool,

    /// Automatically update minor versions without prompting
    #[serde(default)]
    pub auto_update_minor: bool,

    /// List of crate names to ignore during checks
    #[serde(default)]
    pub ignore_crates: Vec<String>,

    /// Show verbose output by default
    #[serde(default)]
    pub verbose: bool,

    /// Create backups before modifying Cargo.toml
    #[serde(default = "default_true")]
    pub create_backups: bool,

    /// Check for security vulnerabilities during health checks
    #[serde(default = "default_true")]
    pub check_security: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_update_patch: false,
            auto_update_minor: false,
            ignore_crates: Vec::new(),
            verbose: false,
            create_backups: true,
            check_security: true,
        }
    }
}

impl Config {
    /// Find and load configuration file
    /// Search order:
    /// 1. .cargo-sane.toml in current directory
    /// 2. .cargo-sane.toml in project root (where Cargo.toml is)
    /// 3. ~/.config/cargo-sane/config.toml
    /// 4. ~/.cargo-sane.toml
    pub fn load() -> Result<Self> {
        // Try current directory
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let local_config = current_dir.join(".cargo-sane.toml");
        if local_config.exists() {
            return Self::from_file(&local_config);
        }

        // Try home directory config
        if let Some(home) = dirs::home_dir() {
            // Try ~/.config/cargo-sane/config.toml
            let config_dir = home.join(".config").join("cargo-sane").join("config.toml");
            if config_dir.exists() {
                return Self::from_file(&config_dir);
            }

            // Try ~/.cargo-sane.toml
            let home_config = home.join(".cargo-sane.toml");
            if home_config.exists() {
                return Self::from_file(&home_config);
            }
        }

        // No config file found, return defaults
        Ok(Self::default())
    }

    /// Load configuration from a specific file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read config file: {}", path.display()))?;

        let config: Config =
            toml::from_str(&content).context("Failed to parse configuration file")?;

        Ok(config)
    }

    /// Save configuration to the default location (~/.config/cargo-sane/config.toml)
    pub fn save(&self) -> Result<PathBuf> {
        let config_dir = self.get_config_dir()?;
        let config_path = config_dir.join("config.toml");

        self.save_to(&config_path)?;
        Ok(config_path)
    }

    /// Save configuration to a specific file
    pub fn save_to(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {}", parent.display()))?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(path, content)
            .context(format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Get the default configuration directory
    fn get_config_dir(&self) -> Result<PathBuf> {
        let home = dirs::home_dir().context("Failed to get home directory")?;
        Ok(home.join(".config").join("cargo-sane"))
    }

    /// Initialize a new configuration file in the current directory
    pub fn init_local() -> Result<PathBuf> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let config_path = current_dir.join(".cargo-sane.toml");

        let default_config = Self::default();
        default_config.save_to(&config_path)?;

        Ok(config_path)
    }

    /// Check if a crate should be ignored
    pub fn should_ignore(&self, crate_name: &str) -> bool {
        self.ignore_crates.iter().any(|name| name == crate_name)
    }

    /// Generate a sample configuration file content
    pub fn sample() -> String {
        r#"# cargo-sane configuration file
# Place this file at:
#   - .cargo-sane.toml (project-specific)
#   - ~/.config/cargo-sane/config.toml (global)
#   - ~/.cargo-sane.toml (global, alternative)

# Automatically update patch versions (0.0.X) without prompting
auto_update_patch = false

# Automatically update minor versions (0.X.0) without prompting
auto_update_minor = false

# Crates to ignore during dependency checks
ignore_crates = [
    # "some-internal-crate",
]

# Show verbose output by default
verbose = false

# Create backups before modifying Cargo.toml
create_backups = true

# Check for security vulnerabilities during health checks
check_security = true
"#
        .to_string()
    }
}

/// Simple home directory detection (fallback if dirs crate not available)
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| std::env::var("USERPROFILE").ok().map(PathBuf::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.auto_update_patch);
        assert!(!config.auto_update_minor);
        assert!(config.ignore_crates.is_empty());
        assert!(!config.verbose);
        assert!(config.create_backups);
        assert!(config.check_security);
    }

    #[test]
    fn test_should_ignore() {
        let mut config = Config::default();
        config.ignore_crates = vec!["test-crate".to_string()];

        assert!(config.should_ignore("test-crate"));
        assert!(!config.should_ignore("other-crate"));
    }
}
