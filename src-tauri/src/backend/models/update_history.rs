use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHistoryEntry {
    pub package_id: String,
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub timestamp: DateTime<Utc>,
    pub status: UpdateHistoryStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateHistoryStatus {
    Success,
    Failed,
    Rollback,
}

impl UpdateHistoryEntry {
    pub fn new_success(
        package_id: String,
        package_name: String,
        old_version: String,
        new_version: String,
    ) -> Self {
        Self {
            package_id,
            package_name,
            old_version,
            new_version,
            timestamp: Utc::now(),
            status: UpdateHistoryStatus::Success,
            error_message: None,
        }
    }

    pub fn new_failed(
        package_id: String,
        package_name: String,
        old_version: String,
        new_version: String,
        error: String,
    ) -> Self {
        Self {
            package_id,
            package_name,
            old_version,
            new_version,
            timestamp: Utc::now(),
            status: UpdateHistoryStatus::Failed,
            error_message: Some(error),
        }
    }
}
