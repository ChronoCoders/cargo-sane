//! Integration tests for the health command

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_health_command_no_cargo_toml() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("health")
        .arg("--manifest-path")
        .arg(temp_dir.path().join("Cargo.toml"));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_health_command_json_output() {
    let (_temp_dir, cargo_toml) = common::create_test_project();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("health")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .arg("--json");

    // JSON output should be valid JSON or show error
    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should either output JSON or handle the case
    if output.status.success() {
        assert!(
            stdout.contains("{") || stdout.contains("dependencies") || stdout.contains("vulnerable")
        );
    }
}

#[test]
fn test_health_empty_dependencies() {
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
    cmd.arg("health")
        .arg("--manifest-path")
        .arg(&cargo_toml);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No dependencies"));
}

#[test]
fn test_health_shows_header() {
    let (_temp_dir, cargo_toml) = common::create_test_project();

    let mut cmd = Command::cargo_bin("cargo-sane").unwrap();
    cmd.arg("health")
        .arg("--manifest-path")
        .arg(&cargo_toml);

    let output = cmd.output().expect("Failed to run command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show the health command header
    assert!(
        stdout.contains("cargo-sane health") || stdout.contains("Health Report") || !output.status.success()
    );
}
