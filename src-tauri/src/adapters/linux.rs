use async_trait::async_trait;
use std::process::Command;
use std::path::PathBuf;
use crate::backend::{
    error::Result,
    models::{Driver, DriverUpdate, Package, PackageManager, PackageUpdate, Platform, UpdateResult, PackageDetails, PackageCategory},
};
use super::platform_adapter::PlatformAdapter;

pub struct LinuxAdapter {
    package_manager: PackageManager,
}

#[derive(Debug, Clone)]
enum PackageManager {
    Apt,
    Dnf,
    Pacman,
    Unknown,
}

impl LinuxAdapter {
    pub fn new() -> Self {
        let package_manager = Self::detect_package_manager();
        Self { package_manager }
    }

    fn detect_package_manager() -> PackageManager {
        if Command::new("apt").arg("--version").output().is_ok() {
            PackageManager::Apt
        } else if Command::new("dnf").arg("--version").output().is_ok() {
            PackageManager::Dnf
        } else if Command::new("pacman").arg("--version").output().is_ok() {
            PackageManager::Pacman
        } else {
            PackageManager::Unknown
        }
    }

    fn parse_apt_list(&self, output: &str) -> Vec<Package> {
        output
            .lines()
            .filter(|line| line.starts_with("Listing...") == false && !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('/').collect();
                if parts.is_empty() {
                    return None;
                }
                
                let name_version: Vec<&str> = parts[0].split_whitespace().collect();
                let name = name_version.first()?.to_string();                
                // Extract version from the line
                let version = if let Some(version_part) = line.split_whitespace().nth(1) {
                    version_part.to_string()
                } else {
                    "unknown".to_string()
                };

                Some(Package {
                    id: name.clone(),
                    name,
                    version,
                    platform: Platform::Linux,
                })
            })
            .collect()
    }

    fn parse_dnf_list(&self, output: &str) -> Vec<Package> {
        output
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with("Installed Packages"))
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    return None;
                }
                
                let name = parts[0].split('.').next()?.to_string();
                let version = parts[1].to_string();

                Some(Package {
                    id: name.clone(),
                    name,
                    version,
                    platform: Platform::Linux,
                })
            })
            .collect()
    }

    fn parse_pacman_list(&self, output: &str) -> Vec<Package> {
        output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    return None;
                }
                
                let name = parts[0].to_string();
                let version = parts[1].to_string();

                Some(Package {
                    id: name.clone(),
                    name,
                    version,
                    platform: Platform::Linux,
                })
            })
            .collect()
    }

    fn parse_apt_upgradable(&self, output: &str) -> Vec<(String, String)> {
        output
            .lines()
            .filter(|line| !line.starts_with("Listing...") && !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('/').collect();
                if parts.is_empty() {
                    return None;
                }
                
                let name = parts[0].trim().to_string();
                
                // Extract new version from brackets [version]
                if let Some(bracket_start) = line.find('[') {
                    if let Some(bracket_end) = line.find(']') {
                        let new_version = line[bracket_start + 1..bracket_end].trim().to_string();
                        return Some((name, new_version));
                    }
                }
                
                None
            })
            .collect()
    }

    fn parse_dnf_check_update(&self, output: &str) -> Vec<(String, String)> {
        output
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.contains("Last metadata"))
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    return None;
                }
                
                let name = parts[0].split('.').next()?.to_string();
                let new_version = parts[1].to_string();
                Some((name, new_version))
            })
            .collect()
    }

    fn parse_pacman_updates(&self, output: &str) -> Vec<(String, String)> {
        output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    return None;
                }
                
                let name = parts[0].to_string();
                let new_version = parts[3].to_string();
                Some((name, new_version))
            })
            .collect()
    }
}

#[async_trait]
impl PlatformAdapter for LinuxAdapter {
    fn get_package_managers(&self) -> Vec<PackageManager> {
        let mut managers = vec![self.package_manager.clone()];
        managers.push(PackageManager::Npm);
        managers
    }

    async fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Apt => {
                if let Ok(output) = Command::new("apt").args(["list", "--installed"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(self.parse_apt_list(&stdout));
                }
                Ok(Vec::new())
            }
            PackageManager::Dnf => {
                if let Ok(output) = Command::new("dnf").args(["list", "installed"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(self.parse_dnf_list(&stdout));
                }
                Ok(Vec::new())
            }
            PackageManager::Pacman => {
                if let Ok(output) = Command::new("pacman").args(["-Q"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(self.parse_pacman_list(&stdout));
                }
                Ok(Vec::new())
            }
            PackageManager::Npm => {
                if let Ok(output) = Command::new("npm").args(&["list", "-g", "--depth=0", "--json"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                        let mut packages = Vec::new();
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
                        return Ok(packages);
                    }
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        }
    }

    async fn scan_drivers(&self) -> Result<Vec<Driver>> {
        // Linux driver management is complex and varies by distribution
        // For now, return empty - can be extended with fwupd support
        Ok(Vec::new())
    }

    async fn check_package_updates(&self, packages: &[Package]) -> Result<Vec<PackageUpdate>> {
        let mut updates = Vec::new();

        // System PM
        match self.package_manager {
            PackageManager::Apt => {
                if let Ok(output) = Command::new("apt").args(["list", "--upgradable"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let upgradable = self.parse_apt_upgradable(&stdout);
                    for (pkg_name, new_version) in upgradable {
                        updates.push(PackageUpdate {
                            package: Package {
                                id: pkg_name.clone(),
                                name: pkg_name,
                                version: "unknown".to_string(), // In real app, match from packages
                                manager: PackageManager::Apt,
                                description: None,
                                installed_date: None,
                            },
                            new_version,
                            size_bytes: None,
                            priority: crate::backend::models::UpdatePriority::Normal,
                            changelog: None,
                        });
                    }
                }
            }
            PackageManager::Dnf => {
                if let Ok(output) = Command::new("dnf").args(["check-update"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let upgradable = self.parse_dnf_check_update(&stdout);
                    for (pkg_name, new_version) in upgradable {
                        updates.push(PackageUpdate {
                            package: Package {
                                id: pkg_name.clone(),
                                name: pkg_name,
                                version: "unknown".to_string(),
                                manager: PackageManager::Dnf,
                                description: None,
                                installed_date: None,
                            },
                            new_version,
                            size_bytes: None,
                            priority: crate::backend::models::UpdatePriority::Normal,
                            changelog: None,
                        });
                    }
                }
            }
            PackageManager::Pacman => {
                if let Ok(output) = Command::new("pacman").args(["-Qu"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let upgradable = self.parse_pacman_updates(&stdout);
                    for (pkg_name, new_version) in upgradable {
                        updates.push(PackageUpdate {
                            package: Package {
                                id: pkg_name.clone(),
                                name: pkg_name,
                                version: "unknown".to_string(),
                                manager: PackageManager::Pacman,
                                description: None,
                                installed_date: None,
                            },
                            new_version,
                            size_bytes: None,
                            priority: crate::backend::models::UpdatePriority::Normal,
                            changelog: None,
                        });
                    }
                }
            }
            _ => {}
        }

        // NPM
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
                                updates.push(PackageUpdate {
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

        Ok(updates)
    }

    async fn check_driver_updates(&self, _drivers: &[Driver]) -> Result<Vec<(Driver, String)>> {
        Ok(Vec::new())
    }

    async fn rollback_package(&self, _package: &Package, _target_version: &str) -> Result<UpdateResult> {
        Ok(UpdateResult {
            status: crate::backend::models::UpdateStatus::Failed,
            error: Some("Rollback not implemented on Linux".to_string()),
            duration: chrono::Duration::seconds(0),
        })
    }

    async fn apply_package_update(&self, update: &PackageUpdate) -> Result<UpdateResult> {
        let output = match update.package.manager {
            PackageManager::Apt => {
                Command::new("sudo")
                    .args(["apt", "install", "--only-upgrade", "-y", "-qq", &update.package.id])
                    .output()?
            }
            PackageManager::Dnf => {
                Command::new("sudo")
                    .args(["dnf", "upgrade", "-y", "-q", &update.package.id])
                    .output()?
            }
            PackageManager::Pacman => {
                Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "--quiet", &update.package.id])
                    .output()?
            }
            PackageManager::Npm => {
                Command::new("npm")
                    .args(["install", "-g", "--silent", &format!("{}@latest", update.package.id)])
                    .output()?
            }
            _ => {
                return Ok(UpdateResult {
                    status: crate::backend::models::UpdateStatus::Failed,
                    error: Some("Unsupported package manager on Linux".to_string()),
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

    async fn get_package_details(&self, _package_id: &str) -> Result<Option<PackageDetails>> {
        Ok(None)
    }
}
