//! Cargo command execution and dependency usage analysis

use crate::Result;
use anyhow::Context;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Analyzes Rust source code to find which dependencies are actually used
pub struct DependencyUsageAnalyzer {
    source_root: PathBuf,
}

impl DependencyUsageAnalyzer {
    pub fn new(manifest_path: &Path) -> Result<Self> {
        let source_root = manifest_path
            .parent()
            .context("Failed to get parent directory of Cargo.toml")?
            .to_path_buf();

        Ok(Self { source_root })
    }

    /// Find all Rust source files in the project
    fn find_rust_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let src_dir = self.source_root.join("src");
        if src_dir.exists() {
            self.collect_rust_files(&src_dir, &mut files)?;
        }

        // Also check tests directory
        let tests_dir = self.source_root.join("tests");
        if tests_dir.exists() {
            self.collect_rust_files(&tests_dir, &mut files)?;
        }

        // Check examples directory
        let examples_dir = self.source_root.join("examples");
        if examples_dir.exists() {
            self.collect_rust_files(&examples_dir, &mut files)?;
        }

        Ok(files)
    }

    fn collect_rust_files(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries = fs::read_dir(dir).context(format!("Failed to read directory: {:?}", dir))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.collect_rust_files(&path, files)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path);
            }
        }

        Ok(())
    }

    /// Analyze source files to find used dependencies
    pub fn find_used_dependencies(&self) -> Result<HashSet<String>> {
        let files = self.find_rust_files()?;
        let mut used_deps = HashSet::new();

        // Patterns to match dependency usage
        let use_pattern = Regex::new(r"(?m)^use\s+([a-zA-Z_][a-zA-Z0-9_]*)(?:::|;)")
            .context("Failed to compile use pattern")?;
        let extern_pattern = Regex::new(r"(?m)^extern\s+crate\s+([a-zA-Z_][a-zA-Z0-9_]*)")
            .context("Failed to compile extern pattern")?;
        let macro_use_pattern =
            Regex::new(r"#\[macro_use\]\s*extern\s+crate\s+([a-zA-Z_][a-zA-Z0-9_]*)")
                .context("Failed to compile macro_use pattern")?;
        // Also check for direct macro invocations like serde_json::json!
        let macro_call_pattern = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)(?:::|\!)")
            .context("Failed to compile macro call pattern")?;

        for file in files {
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Find use statements
            for cap in use_pattern.captures_iter(&content) {
                if let Some(name) = cap.get(1) {
                    let dep_name = name.as_str().to_string();
                    // Convert underscores back to hyphens (cargo convention)
                    let normalized = dep_name.replace('_', "-");
                    used_deps.insert(dep_name);
                    used_deps.insert(normalized);
                }
            }

            // Find extern crate declarations
            for cap in extern_pattern.captures_iter(&content) {
                if let Some(name) = cap.get(1) {
                    let dep_name = name.as_str().to_string();
                    let normalized = dep_name.replace('_', "-");
                    used_deps.insert(dep_name);
                    used_deps.insert(normalized);
                }
            }

            // Find #[macro_use] extern crate
            for cap in macro_use_pattern.captures_iter(&content) {
                if let Some(name) = cap.get(1) {
                    let dep_name = name.as_str().to_string();
                    let normalized = dep_name.replace('_', "-");
                    used_deps.insert(dep_name);
                    used_deps.insert(normalized);
                }
            }

            // Find macro calls (e.g., serde_json::json!)
            for cap in macro_call_pattern.captures_iter(&content) {
                if let Some(name) = cap.get(1) {
                    let dep_name = name.as_str().to_string();
                    let normalized = dep_name.replace('_', "-");
                    used_deps.insert(dep_name);
                    used_deps.insert(normalized);
                }
            }
        }

        // Add standard library crates that are always "used"
        let std_crates = ["std", "core", "alloc", "proc_macro", "test"];
        for crate_name in &std_crates {
            used_deps.insert(crate_name.to_string());
        }

        Ok(used_deps)
    }

    /// Find unused dependencies by comparing declared vs used
    pub fn find_unused_dependencies(
        &self,
        declared_deps: &[(String, crate::core::manifest::DependencySpec)],
    ) -> Result<Vec<String>> {
        let used = self.find_used_dependencies()?;
        let mut unused = Vec::new();

        for (name, _spec) in declared_deps {
            // Normalize the dependency name (replace hyphens with underscores)
            let normalized_name = name.replace('-', "_");

            // Check if either form is used
            let is_used = used.contains(name)
                || used.contains(&normalized_name)
                || used.contains(&name.replace('_', "-"));

            if !is_used {
                unused.push(name.clone());
            }
        }

        Ok(unused)
    }
}
