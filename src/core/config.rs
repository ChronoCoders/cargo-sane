//! Configuration file handling

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub auto_update_patch: bool,
    pub auto_update_minor: bool,
    pub ignore_crates: Vec<String>,
}
