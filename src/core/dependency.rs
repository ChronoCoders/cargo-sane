//! Dependency representation

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub current_version: Version,
    pub latest_version: Option<Version>,
    pub is_direct: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateType {
    Patch,
    Minor,
    Major,
    UpToDate,
}

impl Dependency {
    pub fn new(name: String, current_version: Version, is_direct: bool) -> Self {
        Self {
            name,
            current_version,
            latest_version: None,
            is_direct,
        }
    }

    pub fn with_latest(mut self, latest: Version) -> Self {
        self.latest_version = Some(latest);
        self
    }

    /// Determine the type of update available
    pub fn update_type(&self) -> UpdateType {
        match &self.latest_version {
            None => UpdateType::UpToDate,
            Some(latest) => {
                if latest <= &self.current_version {
                    UpdateType::UpToDate
                } else if latest.major > self.current_version.major {
                    UpdateType::Major
                } else if latest.minor > self.current_version.minor {
                    UpdateType::Minor
                } else {
                    UpdateType::Patch
                }
            }
        }
    }

    /// Check if update is available
    pub fn has_update(&self) -> bool {
        self.update_type() != UpdateType::UpToDate
    }
}