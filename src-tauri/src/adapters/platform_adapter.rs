use std::path::PathBuf;
use async_trait::async_trait;

use crate::backend::{
    models::{Driver, DriverUpdate, Package, PackageManager, PackageUpdate, Platform, UpdateResult, PackageDetails},
    Result,
};

/// Platform adapter trait - defines platform-specific operations
#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    // Platform detection
    fn detect_platform(&self) -> Platform;
    
    // Package manager operations
    fn get_package_managers(&self) -> Vec<PackageManager>;
    async fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>>;
    async fn check_package_updates(&self, packages: &[Package]) -> Result<Vec<PackageUpdate>>;
    async fn apply_package_update(&self, update: &PackageUpdate) -> Result<UpdateResult>;
    async fn rollback_package(&self, package: &Package, target_version: &str) -> Result<UpdateResult>;
    async fn get_package_details(&self, package_id: &str) -> Result<Option<PackageDetails>>;
    
    // Driver operations
    async fn scan_drivers(&self) -> Result<Vec<Driver>>;
    async fn check_driver_updates(&self, drivers: &[Driver]) -> Result<Vec<DriverUpdate>>;
    async fn apply_driver_update(&self, update: &DriverUpdate) -> Result<UpdateResult>;
    async fn backup_driver(&self, driver: &Driver) -> Result<PathBuf>;
    async fn restore_driver(&self, backup_path: &PathBuf) -> Result<()>;
    
    // Permission management
    fn requires_elevation(&self, operation: &str) -> bool;
    async fn request_elevation(&self) -> Result<()>;
}
