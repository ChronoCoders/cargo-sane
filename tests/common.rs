//! Common test utilities and fixtures

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Create a temporary directory with a valid Cargo.toml
pub fn create_test_project() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let cargo_toml = temp_dir.path().join("Cargo.toml");

    let content = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0.100"
tokio = "1.0.0"
clap = "4.0.0"
"#;

    fs::write(&cargo_toml, content).expect("Failed to write Cargo.toml");

    // Create a minimal src/main.rs
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");

    let main_rs = src_dir.join("main.rs");
    let main_content = r#"
use serde::Serialize;
use tokio::runtime::Runtime;
use clap::Parser;

fn main() {
    println!("Hello, world!");
}
"#;
    fs::write(&main_rs, main_content).expect("Failed to write main.rs");

    (temp_dir, cargo_toml)
}

/// Create a test project with unused dependencies
pub fn create_project_with_unused_deps() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let cargo_toml = temp_dir.path().join("Cargo.toml");

    let content = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0.100"
tokio = "1.0.0"
unused-crate = "1.0.0"
another-unused = "2.0.0"
"#;

    fs::write(&cargo_toml, content).expect("Failed to write Cargo.toml");

    // Create src/main.rs that only uses some dependencies
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");

    let main_rs = src_dir.join("main.rs");
    let main_content = r#"
use serde::Serialize;
use tokio::runtime::Runtime;

fn main() {
    println!("Hello, world!");
}
"#;
    fs::write(&main_rs, main_content).expect("Failed to write main.rs");

    (temp_dir, cargo_toml)
}

/// Create a test configuration file
pub fn create_test_config(dir: &Path) -> PathBuf {
    let config_path = dir.join(".cargo-sane.toml");

    let content = r#"
auto_update_patch = true
auto_update_minor = false
ignore_crates = ["test-crate"]
verbose = false
create_backups = true
check_security = true
"#;

    fs::write(&config_path, content).expect("Failed to write config file");
    config_path
}

/// Get the path to the cargo-sane binary
pub fn cargo_sane_bin() -> PathBuf {
    // During tests, the binary is in target/debug/
    let mut path = std::env::current_exe().expect("Failed to get current exe path");
    path.pop(); // Remove test binary name
    path.pop(); // Remove deps
    path.push("cargo-sane");
    path
}
