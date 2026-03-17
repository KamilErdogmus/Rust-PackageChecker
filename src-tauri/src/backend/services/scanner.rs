// Update Scanner Service
use std::sync::Arc;
use crate::backend::{
    error::Result,
    models::{Package, Driver, PackageUpdate, DriverUpdate, Platform},
};
use crate::adapters::platform_adapter::PlatformAdapter;

pub struct UpdateScanner {
    platform_adapter: Arc<dyn PlatformAdapter>,
}

impl UpdateScanner {
    pub fn new(platform_adapter: Arc<dyn PlatformAdapter>) -> Self {
        Self { platform_adapter }
    }

    pub async fn scan_system(&self) -> Result<ScanResult> {
        let platform = self.platform_adapter.detect_platform();
        let package_managers = self.platform_adapter.get_package_managers();
        
        let mut all_packages = Vec::new();
        for manager in &package_managers {
            match self.platform_adapter.scan_packages(manager).await {
                Ok(packages) => all_packages.extend(packages),
                Err(e) => tracing::warn!("Failed to scan packages for {:?}: {}", manager, e),
            }
        }
        
        let drivers = self.platform_adapter.scan_drivers().await.unwrap_or_default();
        
        Ok(ScanResult {
            platform,
            packages: all_packages,
            drivers,
        })
    }

    pub async fn check_updates(&self, scan_result: &ScanResult) -> Result<Vec<UpdateInfo>> {
        let mut all_updates = Vec::new();
        
        // Check package updates
        match self.platform_adapter.check_package_updates(&scan_result.packages).await {
            Ok(package_updates) => {
                for update in package_updates {
                    all_updates.push(UpdateInfo::Package(update));
                }
            }
            Err(e) => tracing::warn!("Failed to check package updates: {}", e),
        }
        
        // Check driver updates
        match self.platform_adapter.check_driver_updates(&scan_result.drivers).await {
            Ok(driver_updates) => {
                for update in driver_updates {
                    all_updates.push(UpdateInfo::Driver(update));
                }
            }
            Err(e) => tracing::warn!("Failed to check driver updates: {}", e),
        }
        
        Ok(all_updates)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanResult {
    pub platform: Platform,
    pub packages: Vec<Package>,
    pub drivers: Vec<Driver>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum UpdateInfo {
    Package(PackageUpdate),
    Driver(DriverUpdate),
}
