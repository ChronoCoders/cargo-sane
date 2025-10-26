//! # cargo-sane
//!
//! Stop losing your mind over Rust dependency conflicts.
//!
//! `cargo-sane` is a CLI tool that helps you manage Rust dependencies intelligently.
//! It provides commands to check for updates, resolve conflicts, clean unused dependencies,
//! and monitor the health of your dependency tree.

pub mod analyzer;
pub mod cli;
pub mod core;
pub mod updater;
pub mod utils;

// Re-export commonly used types
pub use anyhow::{Error, Result};
