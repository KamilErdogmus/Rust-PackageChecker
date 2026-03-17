use async_trait::async_trait;
use std::process::Command;
use std::path::PathBuf;
use crate::backend::{
    error::Result,
    models::{Driver, DriverUpdate, Package, PackageManager, PackageUpdate, Platform, UpdateResult, PackageDetails, PackageCategory},
};
use super::platform_adapter::PlatformAdapter;

pub struct MacOSAdapter;

impl MacOSAdapter {
    pub fn new() -> Self {
        Self
    }

    fn parse_brew_list(&self, output: &str) -> Vec<Package> {
        output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let name = parts.first().unwrap_or(&"unknown").to_string();
                
                Package {
                    id: name.clone(),
                    name: name.clone(),
                    version: parts.get(1).unwrap_or(&"unknown").to_string(),
                    manager: PackageManager::Homebrew,
                    description: None,
                    installed_date: None,
                }
            })
            .collect()
    }

    fn parse_brew_outdated(&self, output: &str) -> Vec<PackageUpdate> {
        let mut updates = Vec::new();
        
        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let package_name = parts[0].to_string();
                let current_version = parts[1].to_string();
                let new_version = parts[3].to_string();
                
                let package = Package {
                    id: package_name.clone(),
                    name: package_name,
                    version: current_version,
                    manager: PackageManager::Homebrew,
                    description: None,
                    installed_date: None,
                };
                
                updates.push(PackageUpdate {
                    package,
                    new_version,
                    size_bytes: None,
                    priority: crate::backend::models::UpdatePriority::Normal,
                    changelog: None,
                });
            }
        }
        
        updates
    }
}

#[async_trait]
impl PlatformAdapter for MacOSAdapter {
    fn detect_platform(&self) -> Platform {
        Platform::MacOS
    }

    fn get_package_managers(&self) -> Vec<PackageManager> {
        vec![PackageManager::Homebrew]
    }

    async fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Homebrew => {
                let output = Command::new("brew")
                    .args(["list", "--versions"])
                    .output()?;

                if !output.status.success() {
                    return Err("Failed to execute brew list".into());
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(self.parse_brew_list(&stdout))
            }
            _ => Ok(Vec::new()),
        }
    }

    async fn scan_drivers(&self) -> Result<Vec<Driver>> {
        Ok(Vec::new())
    }

    async fn check_package_updates(&self, _packages: &[Package]) -> Result<Vec<PackageUpdate>> {
        let output = Command::new("brew")
            .args(["outdated", "--verbose"])
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_brew_outdated(&stdout))
    }

    async fn check_driver_updates(&self, _drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        Ok(Vec::new())
    }

    async fn apply_package_update(&self, update: &PackageUpdate) -> Result<UpdateResult> {
        let output = Command::new("brew")
            .args(["upgrade", &update.package.id])
            .output()?;

        if output.status.success() {
            Ok(UpdateResult {
                status: crate::backend::models::UpdateStatus::Completed,
                error: None,
                duration: chrono::Duration::seconds(0),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Ok(UpdateResult {
                status: crate::backend::models::UpdateStatus::Failed,
                error: Some(stderr.to_string()),
                duration: chrono::Duration::seconds(0),
            })
        }
    }

    async fn apply_driver_update(&self, _update: &DriverUpdate) -> Result<UpdateResult> {
        Err(crate::backend::error::Error::NotImplemented)
    }

    async fn backup_driver(&self, _driver: &Driver) -> Result<PathBuf> {
        Err(crate::backend::error::Error::NotImplemented)
    }

    async fn restore_driver(&self, _backup_path: &PathBuf) -> Result<()> {
        Err(crate::backend::error::Error::NotImplemented)
    }

    fn requires_elevation(&self, _operation: &str) -> bool {
        true
    }

    async fn request_elevation(&self) -> Result<()> {
        Ok(())
    }

    async fn get_package_details(&self, package_id: &str) -> Result<Option<PackageDetails>> {
        let output = Command::new("brew")
            .args(["info", package_id])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        
        if lines.is_empty() {
            return Ok(None);
        }

        let first_line = lines[0];
        let parts: Vec<&str> = first_line.split(':').collect();
        let name = parts[0].trim().to_string();
        
        let description = lines.iter()
            .skip(1)
            .find(|line| !line.trim().is_empty())
            .map(|s| s.trim().to_string());

        Ok(Some(PackageDetails {
            id: package_id.to_string(),
            name,
            version: "".to_string(),
            description,
            publisher: Some("Homebrew".to_string()),
            install_date: None,
            size: None,
            homepage: None,
            category: PackageCategory::from_package_id(package_id),
        }))
    }
}
