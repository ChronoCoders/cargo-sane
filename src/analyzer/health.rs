//! Health and security analysis for dependencies

use crate::core::dependency::Dependency;
use crate::Result;
use indicatif::{ProgressBar, ProgressStyle};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security advisory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advisory {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub affected_versions: String,
    pub patched_versions: Option<String>,
    pub url: Option<String>,
}

/// Severity level of a security advisory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Unknown,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Unknown => "UNKNOWN",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Severity::Critical => "🔴",
            Severity::High => "🟠",
            Severity::Medium => "🟡",
            Severity::Low => "🟢",
            Severity::Unknown => "⚪",
        }
    }
}

/// Health status of a dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyHealth {
    pub name: String,
    pub version: String,
    pub advisories: Vec<Advisory>,
    pub is_outdated: bool,
    pub maintenance_score: Option<u8>, // 0-100
}

impl DependencyHealth {
    pub fn is_vulnerable(&self) -> bool {
        !self.advisories.is_empty()
    }

    pub fn highest_severity(&self) -> Option<Severity> {
        self.advisories
            .iter()
            .map(|a| a.severity)
            .max_by_key(|s| match s {
                Severity::Critical => 4,
                Severity::High => 3,
                Severity::Medium => 2,
                Severity::Low => 1,
                Severity::Unknown => 0,
            })
    }
}

/// Overall health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub dependencies: Vec<DependencyHealth>,
    pub total_dependencies: usize,
    pub vulnerable_count: usize,
    pub outdated_count: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
}

impl HealthReport {
    pub fn new(dependencies: Vec<DependencyHealth>) -> Self {
        let total_dependencies = dependencies.len();
        let vulnerable_count = dependencies.iter().filter(|d| d.is_vulnerable()).count();
        let outdated_count = dependencies.iter().filter(|d| d.is_outdated).count();

        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;

        for dep in &dependencies {
            for advisory in &dep.advisories {
                match advisory.severity {
                    Severity::Critical => critical_count += 1,
                    Severity::High => high_count += 1,
                    Severity::Medium => medium_count += 1,
                    Severity::Low => low_count += 1,
                    Severity::Unknown => {}
                }
            }
        }

        Self {
            dependencies,
            total_dependencies,
            vulnerable_count,
            outdated_count,
            critical_count,
            high_count,
            medium_count,
            low_count,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.vulnerable_count == 0
    }
}

/// Health checker for analyzing dependency security and maintenance
pub struct HealthChecker {
    // Cache of known vulnerabilities (crate name -> advisories)
    advisory_cache: HashMap<String, Vec<Advisory>>,
}

impl HealthChecker {
    pub fn new() -> Result<Self> {
        Ok(Self {
            advisory_cache: Self::load_advisory_database()?,
        })
    }

    /// Load a basic advisory database
    /// In a full implementation, this would fetch from RustSec advisory-db
    fn load_advisory_database() -> Result<HashMap<String, Vec<Advisory>>> {
        let mut db = HashMap::new();

        // Known vulnerabilities (examples - in production would fetch from RustSec)
        // These are real historical CVEs for demonstration
        db.insert(
            "hyper".to_string(),
            vec![Advisory {
                id: "RUSTSEC-2021-0078".to_string(),
                title: "Integer overflow in hyper's parsing of the Transfer-Encoding header"
                    .to_string(),
                description:
                    "An integer overflow exists in hyper's header parsing code that can lead to data loss"
                        .to_string(),
                severity: Severity::High,
                affected_versions: "< 0.14.10".to_string(),
                patched_versions: Some("0.14.10".to_string()),
                url: Some("https://rustsec.org/advisories/RUSTSEC-2021-0078.html".to_string()),
            }],
        );

        db.insert(
            "regex".to_string(),
            vec![Advisory {
                id: "RUSTSEC-2022-0013".to_string(),
                title: "Regex denial of service".to_string(),
                description:
                    "The regex crate has a potential denial of service vulnerability with certain patterns"
                        .to_string(),
                severity: Severity::Medium,
                affected_versions: "< 1.5.5".to_string(),
                patched_versions: Some("1.5.5".to_string()),
                url: Some("https://rustsec.org/advisories/RUSTSEC-2022-0013.html".to_string()),
            }],
        );

        db.insert(
            "tokio".to_string(),
            vec![Advisory {
                id: "RUSTSEC-2023-0001".to_string(),
                title: "tokio::io::ReadHalf::unsplit is Unsound".to_string(),
                description:
                    "tokio::io::ReadHalf::unsplit can violate the API contract of ReadHalf and WriteHalf"
                        .to_string(),
                severity: Severity::High,
                affected_versions: ">= 1.8.0, < 1.18.5".to_string(),
                patched_versions: Some("1.18.5".to_string()),
                url: Some("https://rustsec.org/advisories/RUSTSEC-2023-0001.html".to_string()),
            }],
        );

        db.insert(
            "chrono".to_string(),
            vec![Advisory {
                id: "RUSTSEC-2020-0159".to_string(),
                title: "Potential segfault in localtime_r invocations".to_string(),
                description: "chrono had potential unsoundness in localtime_r usage".to_string(),
                severity: Severity::Medium,
                affected_versions: "< 0.4.20".to_string(),
                patched_versions: Some("0.4.20".to_string()),
                url: Some("https://rustsec.org/advisories/RUSTSEC-2020-0159.html".to_string()),
            }],
        );

        Ok(db)
    }

    /// Check if a version is affected by an advisory
    fn is_version_affected(&self, version: &Version, affected_spec: &str) -> bool {
        // Parse version range specifications like "< 1.5.5" or ">= 1.0.0, < 2.0.0"
        let parts: Vec<&str> = affected_spec.split(',').map(|s| s.trim()).collect();

        for part in parts {
            let part = part.trim();

            if let Some(v) = part.strip_prefix("< ") {
                if let Ok(max) = Version::parse(v.trim()) {
                    if version >= &max {
                        return false;
                    }
                }
            } else if let Some(v) = part.strip_prefix("<= ") {
                if let Ok(max) = Version::parse(v.trim()) {
                    if version > &max {
                        return false;
                    }
                }
            } else if let Some(v) = part.strip_prefix(">= ") {
                if let Ok(min) = Version::parse(v.trim()) {
                    if version < &min {
                        return false;
                    }
                }
            } else if let Some(v) = part.strip_prefix("> ") {
                if let Ok(min) = Version::parse(v.trim()) {
                    if version <= &min {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Check health of all dependencies
    pub fn check_health(&self, dependencies: &[Dependency]) -> Result<HealthReport> {
        let mut health_results = Vec::new();

        let pb = ProgressBar::new(dependencies.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .expect("Failed to set progress style")
                .progress_chars("#>-"),
        );

        for dep in dependencies {
            pb.set_message(format!("Checking security for {}", dep.name));

            let mut advisories = Vec::new();

            // Check against known vulnerabilities
            if let Some(known_advisories) = self.advisory_cache.get(&dep.name) {
                for advisory in known_advisories {
                    if self.is_version_affected(&dep.current_version, &advisory.affected_versions) {
                        advisories.push(advisory.clone());
                    }
                }
            }

            let health = DependencyHealth {
                name: dep.name.clone(),
                version: dep.current_version.to_string(),
                advisories,
                is_outdated: dep.has_update(),
                maintenance_score: None, // Would require additional API calls
            };

            health_results.push(health);
            pb.inc(1);
        }

        pb.finish_with_message("Security check complete");
        println!();

        Ok(HealthReport::new(health_results))
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new().expect("Failed to create HealthChecker")
    }
}
