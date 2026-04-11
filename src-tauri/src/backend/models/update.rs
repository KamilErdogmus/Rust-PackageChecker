use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::{Driver, Package};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageUpdate {
    pub package: Package,
    pub new_version: String,
    pub size_bytes: Option<u64>,
    pub priority: UpdatePriority,
    pub changelog: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverUpdate {
    pub driver: Driver,
    pub new_version: String,
    pub size_bytes: Option<u64>,
    pub requires_reboot: bool,
    pub release_notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UpdatePriority {
    Critical = 3,
    Important = 2,
    Normal = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub status: UpdateStatus,
    pub error: Option<String>,
    pub duration: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateStatus {
    Completed,
    Failed,
    RequiresReboot,
}
