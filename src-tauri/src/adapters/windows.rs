use async_trait::async_trait;
use std::process::Command;
use crate::backend::{
    error::Result,
    models::{Driver, DriverUpdate, Package, PackageManager, PackageUpdate, Platform, UpdateResult, PackageDetails, PackageCategory},
};
use super::platform_adapter::PlatformAdapter;

pub struct WindowsAdapter;

impl WindowsAdapter {
    pub fn new() -> Self {
        Self
    }

    fn parse_winget_list(&self, output: &str) -> Vec<Package> {
        let mut packages = Vec::new();
        let lines: Vec<&str> = output.lines().collect();
        
        // Skip header lines (first 2 lines)
        for line in lines.iter().skip(2) {
            if line.trim().is_empty() {
                continue;
            }
            
            // Parse winget output format: Name  Id  Version  Available  Source
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let id = if parts.len() > 1 { parts[1].to_string() } else { name.clone() };
                let version = if parts.len() > 2 { parts[2].to_string() } else { "unknown".to_string() };
                
                packages.push(Package {
                    id: id.clone(),
                    name,
                    version,
                    manager: PackageManager::Winget,
                    description: None,
                    installed_date: None,
                });
            }
        }
        
        packages
    }

    fn parse_winget_upgrades(&self, output: &str) -> Vec<PackageUpdate> {
        let mut updates = Vec::new();
        let lines: Vec<&str> = output.lines().collect();
        
        // Find the separator line (dashes)
        let separator_index = lines.iter().position(|line| {
            line.contains("---") && line.len() > 50
        });
        
        if separator_index.is_none() {
            tracing::warn!("Could not find separator line in winget output");
            return updates;
        }
        
        // Process lines after separator
        for line in lines.iter().skip(separator_index.unwrap() + 1) {
            let trimmed = line.trim();
            
            // Skip empty lines and summary lines
            if trimmed.is_empty() || trimmed.contains("upgrades available") {
                continue;
            }
            
            // Try to extract using regex-like pattern
            // Format: Name (variable width) | Id (variable width) | Version | Available | Source
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() < 4 {
                continue;
            }
            
            // Find version-like strings (contain dots or numbers)
            let mut version_indices = Vec::new();
            for (i, part) in parts.iter().enumerate() {
                if part.contains('.') || part.chars().all(|c| c.is_numeric() || c == '.') {
                    version_indices.push(i);
                }
            }
            
            if version_indices.len() < 2 {
                continue;
            }
            
            // Last two version-like strings are current and available versions
            let current_version_idx = version_indices[version_indices.len() - 2];
            let new_version_idx = version_indices[version_indices.len() - 1];
            
            let current_version = parts[current_version_idx].to_string();
            let new_version = parts[new_version_idx].to_string();
            
            // ID is typically before the version
            let id_idx = if current_version_idx > 0 {
                current_version_idx - 1
            } else {
                continue;
            };
            
            let id = parts[id_idx].to_string();
            
            // Name is everything before the ID
            let name = parts[..id_idx].join(" ");
            
            if name.is_empty() || id.is_empty() {
                continue;
            }
            
            let package = Package {
                id: id.clone(),
                name,
                version: current_version,
                manager: PackageManager::Winget,
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
        
        tracing::info!("Parsed {} updates from winget", updates.len());
        updates
    }

    fn parse_winget_show(&self, output: &str) -> Option<PackageDetails> {
        let mut id = String::new();
        let mut name = String::new();
        let mut version = String::new();
        let mut description = None;
        let mut publisher = None;
        let mut homepage = None;
        
        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            if let Some(value) = line.strip_prefix("Id:") {
                id = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("Name:") {
                name = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("Version:") {
                version = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("Description:") {
                description = Some(value.trim().to_string());
            } else if let Some(value) = line.strip_prefix("Publisher:") {
                publisher = Some(value.trim().to_string());
            } else if let Some(value) = line.strip_prefix("Homepage:") {
                homepage = Some(value.trim().to_string());
            }
        }
        
        if id.is_empty() {
            return None;
        }
        
        Some(PackageDetails {
            id: id.clone(),
            name: if name.is_empty() { id.clone() } else { name },
            version,
            description,
            publisher,
            install_date: None,
            size: None,
            homepage,
            category: PackageCategory::from_package_id(&id),
        })
    }
}

#[async_trait]
impl PlatformAdapter for WindowsAdapter {
    fn detect_platform(&self) -> Platform {
        Platform::Windows(crate::backend::models::WindowsVersion::Windows11)
    }

    fn get_package_managers(&self) -> Vec<PackageManager> {
        vec![PackageManager::Winget]
    }

    async fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Winget => {
                let output = Command::new("winget")
                    .args(&["list"])
                    .output()?;
                
                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(self.parse_winget_list(&stdout))
            }
            _ => Ok(Vec::new()),
        }
    }

    async fn check_package_updates(&self, _packages: &[Package]) -> Result<Vec<PackageUpdate>> {
        // Use winget upgrade to check for updates
        let output = Command::new("winget")
            .args(&["upgrade"])
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_winget_upgrades(&stdout))
    }

    async fn apply_package_update(&self, update: &PackageUpdate) -> Result<UpdateResult> {
        let output = Command::new("winget")
            .args(&["upgrade", "--id", &update.package.id, "--silent", "--accept-source-agreements", "--accept-package-agreements"])
            .output()?;
        
        if output.status.success() {
            Ok(UpdateResult {
                status: crate::backend::models::UpdateStatus::Completed,
                error: None,
                duration: chrono::Duration::seconds(0),
            })
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(UpdateResult {
                status: crate::backend::models::UpdateStatus::Failed,
                error: Some(error_msg),
                duration: chrono::Duration::seconds(0),
            })
        }
    }

    async fn scan_drivers(&self) -> Result<Vec<Driver>> {
        // Placeholder - will be implemented later
        Ok(Vec::new())
    }

    async fn check_driver_updates(&self, _drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        // Placeholder - will be implemented later
        Ok(Vec::new())
    }

    async fn apply_driver_update(&self, _update: &DriverUpdate) -> Result<UpdateResult> {
        // Placeholder - will be implemented later
        Err(crate::backend::error::Error::NotImplemented)
    }

    async fn backup_driver(&self, _driver: &Driver) -> Result<std::path::PathBuf> {
        // Placeholder - will be implemented later
        Err(crate::backend::error::Error::NotImplemented)
    }

    async fn restore_driver(&self, _backup_path: &std::path::PathBuf) -> Result<()> {
        // Placeholder - will be implemented later
        Err(crate::backend::error::Error::NotImplemented)
    }

    fn requires_elevation(&self, _operation: &str) -> bool {
        true
    }

    async fn request_elevation(&self) -> Result<()> {
        // Placeholder - will be implemented later
        Ok(())
    }

    async fn get_package_details(&self, package_id: &str) -> Result<Option<PackageDetails>> {
        let output = Command::new("winget")
            .args(&["show", "--id", package_id])
            .output()?;
        
        if !output.status.success() {
            return Ok(None);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_winget_show(&stdout))
    }
}
