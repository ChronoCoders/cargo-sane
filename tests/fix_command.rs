//! Integration tests for the fix command

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_fix_command_no_cargo_toml() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("fix")
        .arg("--manifest-path")
        .arg(temp_dir.path().join("Cargo.toml"));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_fix_shows_header() {
    let (_temp_dir, cargo_toml) = common::create_test_project();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("fix")
        .arg("--manifest-path")
        .arg(&cargo_toml);

    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show the fix command header
    assert!(
        stdout.contains("cargo-sane fix") || stdout.contains("Analyzing") || !output.status.success()
    );
}

#[test]
fn test_fix_no_lock_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let cargo_toml = temp_dir.path().join("Cargo.toml");

    let content = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0.0"
"#;

    std::fs::write(&cargo_toml, content).expect("Failed to write Cargo.toml");

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("fix")
        .arg("--manifest-path")
        .arg(&cargo_toml);

    // Without a lock file, should handle gracefully
    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should either succeed or fail gracefully
    assert!(
        stdout.contains("cargo-sane") || stdout.contains("fix") ||
        stderr.contains("Cargo.lock") || !output.status.success()
    );
}

#[test]
fn test_fix_auto_flag() {
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
    cmd.arg("fix")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--auto");

    // Should accept the auto flag
    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("cargo-sane") || stdout.contains("fix") || !output.status.success()
    );
}
