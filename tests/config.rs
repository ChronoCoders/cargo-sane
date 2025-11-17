//! Integration tests for the configuration system

mod common;

use cargo_sane::core::config::Config;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default_values() {
    let config = Config::default();

    assert!(!config.auto_update_patch);
    assert!(!config.auto_update_minor);
    assert!(config.ignore_crates.is_empty());
    assert!(!config.verbose);
    assert!(config.create_backups);
    assert!(config.check_security);
}

#[test]
fn test_config_load_from_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");

    let content = r#"
auto_update_patch = true
auto_update_minor = true
ignore_crates = ["test-crate", "another-crate"]
verbose = true
create_backups = false
check_security = false
"#;

    fs::write(&config_path, content).expect("Failed to write config file");

    let config = Config::from_file(&config_path).expect("Failed to load config");

    assert!(config.auto_update_patch);
    assert!(config.auto_update_minor);
    assert_eq!(config.ignore_crates.len(), 2);
    assert!(config.ignore_crates.contains(&"test-crate".to_string()));
    assert!(config.verbose);
    assert!(!config.create_backups);
    assert!(!config.check_security);
}

#[test]
fn test_config_save_to_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");

    let mut config = Config::default();
    config.auto_update_patch = true;
    config.ignore_crates = vec!["test-crate".to_string()];

    config
        .save_to(&config_path)
        .expect("Failed to save config");

    // Verify the file was created
    assert!(config_path.exists());

    // Reload and verify
    let loaded = Config::from_file(&config_path).expect("Failed to reload config");
    assert!(loaded.auto_update_patch);
    assert_eq!(loaded.ignore_crates.len(), 1);
}

#[test]
fn test_config_should_ignore() {
    let mut config = Config::default();
    config.ignore_crates = vec!["ignored-crate".to_string(), "another".to_string()];

    assert!(config.should_ignore("ignored-crate"));
    assert!(config.should_ignore("another"));
    assert!(!config.should_ignore("not-ignored"));
}

#[test]
fn test_config_sample_generation() {
    let sample = Config::sample();

    assert!(sample.contains("auto_update_patch"));
    assert!(sample.contains("auto_update_minor"));
    assert!(sample.contains("ignore_crates"));
    assert!(sample.contains("verbose"));
    assert!(sample.contains("create_backups"));
    assert!(sample.contains("check_security"));
}

#[test]
fn test_config_partial_load() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");

    // Only set some values, others should use defaults
    let content = r#"
auto_update_patch = true
"#;

    fs::write(&config_path, content).expect("Failed to write config file");

    let config = Config::from_file(&config_path).expect("Failed to load config");

    // Explicitly set value
    assert!(config.auto_update_patch);
    // Default values
    assert!(!config.auto_update_minor);
    assert!(config.ignore_crates.is_empty());
    assert!(!config.verbose);
    assert!(config.create_backups); // Default is true
    assert!(config.check_security); // Default is true
}
