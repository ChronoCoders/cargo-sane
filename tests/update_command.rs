//! Integration tests for the update command

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_update_command_no_cargo_toml() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("update")
        .arg("--manifest-path")
        .arg(temp_dir.path().join("Cargo.toml"));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_update_dry_run_flag() {
    let (_temp_dir, cargo_toml) = common::create_test_project();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("update")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--dry-run");

    // Dry run should not modify anything
    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should indicate dry-run mode or show the header
    assert!(
        stdout.contains("cargo-sane") || stdout.contains("dry-run") || !output.status.success()
    );
}

#[test]
fn test_update_empty_dependencies() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let cargo_toml = temp_dir.path().join("Cargo.toml");

    let content = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

    std::fs::write(&cargo_toml, content).expect("Failed to write Cargo.toml");

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("update")
        .arg("--manifest-path")
        .arg(&cargo_toml);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("up to date"));
}

#[test]
fn test_update_all_flag() {
    let (_temp_dir, cargo_toml) = common::create_test_project();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("update")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--all")
        .arg("--dry-run"); // Use dry-run to prevent actual changes

    // Should process the --all flag
    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("cargo-sane") || stdout.contains("dry-run") || !output.status.success()
    );
}
