use async_trait::async_trait;
use std::process::Command;
use std::path::PathBuf;
use crate::backend::{
    error::Result,
    models::{Driver, DriverUpdate, Package, PackageManager, PackageUpdate, Platform, UpdateResult, PackageDetails},
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
        Platform::MacOS(crate::backend::models::MacOSVersion::Sonoma)
    }

    fn get_package_managers(&self) -> Vec<PackageManager> {
        vec![
            PackageManager::Homebrew,
            PackageManager::Npm,
        ]
    }

    async fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Homebrew => {
                let mut packages = Vec::new();
                if let Ok(output) = Command::new("brew").args(["--version"]).output() {
                    let ver_str = String::from_utf8_lossy(&output.stdout);
                    let version = ver_str.lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                        .unwrap_or("")
                        .to_string();
                    if !version.is_empty() {
                        packages.push(Package {
                            id: "brew".to_string(),
                            name: "Homebrew".to_string(),
                            version,
                            manager: PackageManager::Homebrew,
                            description: None,
                            installed_date: None,
                        });
                    }
                }
                if let Ok(output) = Command::new("brew").args(["list", "--versions"]).output() {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        packages.extend(self.parse_brew_list(&stdout));
                    }
                }
                Ok(packages)
            }
            PackageManager::Npm => {
                let mut packages = Vec::new();
                if let Ok(output) = Command::new("npm").args(&["--version"]).output() {
                    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !version.is_empty() {
                        packages.push(Package {
                            id: "npm".to_string(),
                            name: "npm".to_string(),
                            version,
                            manager: PackageManager::Npm,
                            description: None,
                            installed_date: None,
                        });
                    }
                }
                if let Ok(output) = Command::new("npm").args(&["list", "-g", "--depth=0", "--json"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                        if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
                            for (name, info) in deps {
                                if let Some(version) = info.get("version").and_then(|v| v.as_str()) {
                                    packages.push(Package {
                                        id: name.clone(),
                                        name: name.clone(),
                                        version: version.to_string(),
                                        manager: PackageManager::Npm,
                                        description: None,
                                        installed_date: None,
                                    });
                                }
                            }
                        }
                    }
                }
                Ok(packages)
            }
            _ => Ok(Vec::new()),
        }
    }

    async fn scan_drivers(&self) -> Result<Vec<Driver>> {
        Ok(Vec::new())
    }

    async fn check_package_updates(&self, _packages: &[Package]) -> Result<Vec<PackageUpdate>> {
        let mut all_updates = Vec::new();

        if let Ok(ver_out) = Command::new("brew").args(["--version"]).output() {
            let ver_str = String::from_utf8_lossy(&ver_out.stdout);
            let current = ver_str.lines().next()
                .and_then(|l| l.split_whitespace().nth(1))
                .unwrap_or("").to_string();
            if !current.is_empty() {
                if let Ok(latest_out) = Command::new("brew").args(["info", "--json=v2", "brew"]).output() {
                    let latest_str = String::from_utf8_lossy(&latest_out.stdout);
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&latest_str) {
                        if let Some(latest) = json.pointer("/formulae/0/versions/stable").and_then(|v| v.as_str()) {
                            if latest != current {
                                all_updates.push(PackageUpdate {
                                    package: Package {
                                        id: "brew".to_string(),
                                        name: "Homebrew".to_string(),
                                        version: current,
                                        manager: PackageManager::Homebrew,
                                        description: None,
                                        installed_date: None,
                                    },
                                    new_version: latest.to_string(),
                                    size_bytes: None,
                                    priority: crate::backend::models::UpdatePriority::Normal,
                                    changelog: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        if let Ok(output) = Command::new("brew").args(["outdated", "--verbose"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            all_updates.extend(self.parse_brew_outdated(&stdout));
        }

        if let Ok(ver_out) = Command::new("npm").args(&["--version"]).output() {
            let current = String::from_utf8_lossy(&ver_out.stdout).trim().to_string();
            if !current.is_empty() {
                if let Ok(latest_out) = Command::new("npm").args(&["view", "npm", "version"]).output() {
                    let latest = String::from_utf8_lossy(&latest_out.stdout).trim().to_string();
                    if !latest.is_empty() && latest != current {
                        all_updates.push(PackageUpdate {
                            package: Package {
                                id: "npm".to_string(),
                                name: "npm".to_string(),
                                version: current,
                                manager: PackageManager::Npm,
                                description: None,
                                installed_date: None,
                            },
                            new_version: latest,
                            size_bytes: None,
                            priority: crate::backend::models::UpdatePriority::Normal,
                            changelog: None,
                        });
                    }
                }
            }
        }

        if let Ok(output) = Command::new("npm").args(&["outdated", "-g", "--json"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                if let Some(deps) = json.as_object() {
                    for (name, info) in deps {
                        if let (Some(current), Some(latest)) = (
                            info.get("current").and_then(|v| v.as_str()),
                            info.get("latest").and_then(|v| v.as_str())
                        ) {
                            if current != latest {
                                all_updates.push(PackageUpdate {
                                    package: Package {
                                        id: name.clone(),
                                        name: name.clone(),
                                        version: current.to_string(),
                                        manager: PackageManager::Npm,
                                        description: None,
                                        installed_date: None,
                                    },
                                    new_version: latest.to_string(),
                                    size_bytes: None,
                                    priority: crate::backend::models::UpdatePriority::Normal,
                                    changelog: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(all_updates)
    }

    async fn check_driver_updates(&self, _drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        Ok(Vec::new())
    }

    async fn apply_package_update(&self, update: &PackageUpdate) -> Result<UpdateResult> {
        let output = match update.package.manager {
            PackageManager::Homebrew => {
                Command::new("brew")
                    .args(["upgrade", "-q", &update.package.id])
                    .output()?
            }
            PackageManager::Npm => {
                Command::new("npm")
                    .args(&["install", "-g", "--silent", &format!("{}@latest", update.package.id)])
                    .output()?
            }
            _ => {
                return Ok(UpdateResult {
                    status: crate::backend::models::UpdateStatus::Failed,
                    error: Some("Unsupported package manager on macOS".to_string()),
                    duration: chrono::Duration::seconds(0),
                })
            }
        };

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

    async fn rollback_package(&self, package: &Package, target_version: &str) -> Result<UpdateResult> {
        let output = match package.manager {
            PackageManager::Homebrew => {
                return Ok(UpdateResult {
                    status: crate::backend::models::UpdateStatus::Failed,
                    error: Some("Homebrew doesn't easily support arbitrary version rollbacks without specific taps/formulas.".to_string()),
                    duration: chrono::Duration::seconds(0),
                });
            }
            PackageManager::Npm => {
                Command::new("npm")
                    .args(&["install", "-g", "--silent", &format!("{}@{}", package.id, target_version)])
                    .output()?
            }
            _ => {
                return Ok(UpdateResult {
                    status: crate::backend::models::UpdateStatus::Failed,
                    error: Some("Unsupported package manager for rollback".to_string()),
                    duration: chrono::Duration::seconds(0),
                })
            }
        };

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
        }))
    }
}
