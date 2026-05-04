use std::sync::Arc;
use crate::backend::{
    error::Result,
    models::{PackageUpdate, DriverUpdate, UpdateResult},
};
use crate::adapters::platform_adapter::PlatformAdapter;
use super::scanner::UpdateInfo;

pub struct UpdateEngine {
    platform_adapter: Arc<dyn PlatformAdapter>,
}

impl UpdateEngine {
    pub fn new(platform_adapter: Arc<dyn PlatformAdapter>) -> Self {
        Self { platform_adapter }
    }

    pub async fn apply_update(&self, update: &UpdateInfo) -> Result<UpdateResult> {
        match update {
            UpdateInfo::Package(pkg_update) => {
                self.apply_package_update(pkg_update).await
            }
            UpdateInfo::Driver(drv_update) => {
                self.apply_driver_update(drv_update).await
            }
        }
    }

    async fn apply_package_update(&self, update: &PackageUpdate) -> Result<UpdateResult> {
        tracing::info!("Applying package update: {} -> {}", update.package.name, update.new_version);
        self.platform_adapter.apply_package_update(update).await
    }

    async fn apply_driver_update(&self, update: &DriverUpdate) -> Result<UpdateResult> {
        tracing::info!("Applying driver update: {} -> {}", update.driver.name, update.new_version);
        self.platform_adapter.apply_driver_update(update).await
    }

    pub async fn apply_batch_updates(&self, updates: Vec<UpdateInfo>) -> Result<BatchUpdateResult> {
        let mut successful = 0;
        let mut failed = 0;
        let mut results_summary = Vec::new();
        
        for update in updates {
            match self.apply_update(&update).await {
                Ok(result) => {
                    if matches!(result.status, crate::backend::models::UpdateStatus::Completed) {
                        successful += 1;
                    } else {
                        failed += 1;
                    }
                    results_summary.push(UpdateResultSummary {
                        name: match &update {
                            UpdateInfo::Package(p) => p.package.name.clone(),
                            UpdateInfo::Driver(d) => d.driver.name.clone(),
                        },
                        status: result.status,
                        error: result.error,
                    });
                }
                Err(e) => {
                    failed += 1;
                    results_summary.push(UpdateResultSummary {
                        name: match &update {
                            UpdateInfo::Package(p) => p.package.name.clone(),
                            UpdateInfo::Driver(d) => d.driver.name.clone(),
                        },
                        status: crate::backend::models::UpdateStatus::Failed,
                        error: Some(e.to_string()),
                    });
                }
            }
        }
        
        Ok(BatchUpdateResult {
            successful,
            failed,
            results: results_summary,
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateResultSummary {
    pub name: String,
    pub status: crate::backend::models::UpdateStatus,
    pub error: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchUpdateResult {
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<UpdateResultSummary>,
}
