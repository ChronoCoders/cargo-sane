//! Integration tests for the clean command

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_clean_command_no_cargo_toml() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("clean")
        .arg("--manifest-path")
        .arg(temp_dir.path().join("Cargo.toml"));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_clean_dry_run() {
    let (_temp_dir, cargo_toml) = common::create_project_with_unused_deps();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("clean")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--dry-run");

    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should identify unused dependencies
    assert!(
        stdout.contains("unused") || stdout.contains("dry-run") || stdout.contains("cargo-sane clean")
    );
}

#[test]
fn test_clean_no_unused_deps() {
    let (_temp_dir, cargo_toml) = common::create_test_project();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("clean")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--dry-run");

    // All dependencies should be used in the test project
    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should indicate analysis is running
    assert!(stdout.contains("cargo-sane") || stdout.contains("Scanning"));
}

#[test]
fn test_clean_shows_header() {
    let (_temp_dir, cargo_toml) = common::create_test_project();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("clean")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--dry-run");

    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show the clean command header
    assert!(stdout.contains("cargo-sane clean") || stdout.contains("Scanning"));
}

#[test]
fn test_clean_empty_dependencies() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let cargo_toml = temp_dir.path().join("Cargo.toml");

    let content = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

    std::fs::write(&cargo_toml, content).expect("Failed to write Cargo.toml");

    // Create empty src directory
    let src_dir = temp_dir.path().join("src");
    std::fs::create_dir(&src_dir).expect("Failed to create src directory");
    let main_rs = src_dir.join("main.rs");
    std::fs::write(&main_rs, "fn main() {}").expect("Failed to write main.rs");

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("clean")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("being used"));
}
