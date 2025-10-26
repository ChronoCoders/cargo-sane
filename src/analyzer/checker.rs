//! Check for dependency updates

use crate::core::dependency::Dependency;
use crate::core::manifest::Manifest;
use crate::utils::crates_io::CratesIoClient;
use crate::Result;
use indicatif::{ProgressBar, ProgressStyle};
use semver::Version;

pub struct DependencyChecker {
    client: CratesIoClient,
}

impl DependencyChecker {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: CratesIoClient::new()?,
        })
    }

    /// Analyze all dependencies in a manifest
    pub fn check_dependencies(&self, manifest: &Manifest) -> Result<Vec<Dependency>> {
        let deps = manifest.get_dependencies();
        let mut results = Vec::new();

        if deps.is_empty() {
            return Ok(results);
        }

        // Create progress bar
        let pb = ProgressBar::new(deps.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .expect("Failed to set progress style")
                .progress_chars("#>-"),
        );

        for (name, spec) in deps {
            pb.set_message(format!("Checking {}", name));

            // Skip git and path dependencies
            if !spec.is_crates_io() {
                pb.inc(1);
                continue;
            }

            // Get current version
            let version_str = match spec.version() {
                Some(v) => v,
                None => {
                    pb.inc(1);
                    continue;
                }
            };

            // Parse version requirement (remove ^, ~, etc)
            let current_version = match parse_version_req(version_str) {
                Some(v) => v,
                None => {
                    eprintln!("Warning: Could not parse version '{}' for {}", version_str, name);
                    pb.inc(1);
                    continue;
                }
            };

            // Fetch latest version from crates.io
            let latest_version = match self.client.get_latest_version(&name) {
                Ok(v) => Some(v),
                Err(e) => {
                    eprintln!("Warning: Failed to fetch info for {}: {}", name, e);
                    None
                }
            };

            let mut dep = Dependency::new(name.clone(), current_version, true);
            if let Some(latest) = latest_version {
                dep = dep.with_latest(latest);
            }

            results.push(dep);
            pb.inc(1);
        }

        pb.finish_with_message("Done");
        println!();

        Ok(results)
    }
}

impl Default for DependencyChecker {
    fn default() -> Self {
        Self::new().expect("Failed to create DependencyChecker")
    }
}

/// Parse a version requirement string and extract a concrete version
/// Examples:
///   "1.0.5" -> Some(1.0.5)
///   "1.0" -> Some(1.0.0)
///   "1" -> Some(1.0.0)
///   "^1.0.5" -> Some(1.0.5)
///   "~1.0.5" -> Some(1.0.5)
///   ">=1.0.5" -> Some(1.0.5)
fn parse_version_req(req: &str) -> Option<Version> {
    // Remove common version requirement prefixes
    let cleaned = req
        .trim()
        .trim_start_matches('^')
        .trim_start_matches('~')
        .trim_start_matches('=')
        .trim_start_matches('>')
        .trim_start_matches('<')
        .trim();

    // Try to parse the cleaned version directly
    if let Ok(v) = Version::parse(cleaned) {
        return Some(v);
    }

    // If it fails, try to normalize the version
    // "1.0" -> "1.0.0"
    // "1" -> "1.0.0"
    let normalized = normalize_version(cleaned);
    Version::parse(&normalized).ok()
}

/// Normalize a version string to major.minor.patch format
/// Examples:
///   "1" -> "1.0.0"
///   "1.0" -> "1.0.0"
///   "1.0.5" -> "1.0.5"
fn normalize_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    
    match parts.len() {
        1 => format!("{}.0.0", parts[0]),
        2 => format!("{}.{}.0", parts[0], parts[1]),
        _ => version.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_version() {
        assert_eq!(normalize_version("1"), "1.0.0");
        assert_eq!(normalize_version("1.0"), "1.0.0");
        assert_eq!(normalize_version("1.0.5"), "1.0.5");
        assert_eq!(normalize_version("1.35"), "1.35.0");
    }

    #[test]
    fn test_parse_version_req() {
        assert_eq!(
            parse_version_req("1.0.5"),
            Some(Version::new(1, 0, 5))
        );
        assert_eq!(
            parse_version_req("1.0"),
            Some(Version::new(1, 0, 0))
        );
        assert_eq!(
            parse_version_req("1"),
            Some(Version::new(1, 0, 0))
        );
        assert_eq!(
            parse_version_req("^1.0.5"),
            Some(Version::new(1, 0, 5))
        );
        assert_eq!(
            parse_version_req("~1.0.5"),
            Some(Version::new(1, 0, 5))
        );
        assert_eq!(
            parse_version_req("1.35"),
            Some(Version::new(1, 35, 0))
        );
    }
}