//! Detect and resolve version conflicts

use crate::core::manifest::Manifest;
use crate::Result;
use anyhow::Context;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

/// Represents a version conflict in the dependency tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub package_name: String,
    pub versions: Vec<String>,
    pub dependents: Vec<String>,
    pub suggested_version: Option<String>,
}

/// Result of conflict analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictReport {
    pub conflicts: Vec<Conflict>,
    pub total_packages: usize,
    pub has_conflicts: bool,
}

impl ConflictReport {
    pub fn new(conflicts: Vec<Conflict>, total_packages: usize) -> Self {
        let has_conflicts = !conflicts.is_empty();
        Self {
            conflicts,
            total_packages,
            has_conflicts,
        }
    }
}

/// Conflict detector that analyzes the dependency tree
pub struct ConflictDetector;

impl ConflictDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect conflicts by analyzing cargo tree output
    pub fn detect_conflicts(&self, manifest: &Manifest) -> Result<ConflictReport> {
        let manifest_dir = manifest
            .path
            .parent()
            .context("Failed to get manifest directory")?;

        // Run cargo tree to get dependency information
        let output = Command::new("cargo")
            .arg("tree")
            .arg("--duplicates")
            .arg("--charset=ascii")
            .current_dir(manifest_dir)
            .output()
            .context("Failed to run cargo tree. Make sure cargo is installed.")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // If cargo tree fails, it might mean no Cargo.lock exists yet
            if stderr.contains("Cargo.lock") {
                return Ok(ConflictReport::new(vec![], 0));
            }
            anyhow::bail!("cargo tree failed: {}", stderr);
        }

        let tree_output = String::from_utf8_lossy(&output.stdout);
        let conflicts = self.parse_duplicates(&tree_output);

        // Count total packages
        let total_output = Command::new("cargo")
            .arg("tree")
            .arg("--charset=ascii")
            .arg("--prefix=none")
            .current_dir(manifest_dir)
            .output()
            .context("Failed to run cargo tree")?;

        let total_packages = if total_output.status.success() {
            String::from_utf8_lossy(&total_output.stdout)
                .lines()
                .filter(|l| !l.trim().is_empty())
                .count()
        } else {
            0
        };

        Ok(ConflictReport::new(conflicts, total_packages))
    }

    /// Parse the cargo tree --duplicates output to find conflicts
    fn parse_duplicates(&self, output: &str) -> Vec<Conflict> {
        let mut package_versions: HashMap<String, Vec<String>> = HashMap::new();
        let mut package_dependents: HashMap<String, Vec<String>> = HashMap::new();

        // Parse lines like: "serde v1.0.200" or "serde v1.0.100"
        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Extract package name and version
            // Format: "package_name vX.Y.Z"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].trim_start_matches(|c| c == '|' || c == '-' || c == ' ' || c == '`');
                if let Some(version_str) = parts.get(1) {
                    if let Some(version) = version_str.strip_prefix('v') {
                        let name = name.to_string();
                        let version = version.to_string();

                        package_versions
                            .entry(name.clone())
                            .or_default()
                            .push(version);

                        // Track which packages depend on this
                        // For simplicity, we'll just note that it's a duplicate
                        package_dependents
                            .entry(name)
                            .or_default()
                            .push("(dependency tree)".to_string());
                    }
                }
            }
        }

        // Build conflict list
        let mut conflicts = Vec::new();

        for (name, versions) in package_versions {
            // Only report if there are actually multiple different versions
            let unique_versions: Vec<String> = versions
                .iter()
                .cloned()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            if unique_versions.len() > 1 {
                let suggested = self.suggest_version(&unique_versions);
                let dependents = package_dependents.get(&name).cloned().unwrap_or_default();

                conflicts.push(Conflict {
                    package_name: name,
                    versions: unique_versions,
                    dependents,
                    suggested_version: suggested,
                });
            }
        }

        // Sort by package name for consistent output
        conflicts.sort_by(|a, b| a.package_name.cmp(&b.package_name));
        conflicts
    }

    /// Suggest the best version to use (usually the highest compatible version)
    fn suggest_version(&self, versions: &[String]) -> Option<String> {
        let mut parsed: Vec<Version> = versions
            .iter()
            .filter_map(|v| Version::parse(v).ok())
            .collect();

        if parsed.is_empty() {
            return None;
        }

        parsed.sort();
        parsed.last().map(|v| v.to_string())
    }
}

impl Default for ConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}
