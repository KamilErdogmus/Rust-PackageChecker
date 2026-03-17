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
    async fn scan_packages(&self) -> Result<Vec<Package>> {
        match self.package_manager {
            PackageManager::Apt => {
                let output = Command::new("apt")
                    .args(["list", "--installed"])
                    .output()?;

                if !output.status.success() {
                    return Err("Failed to execute apt list".into());
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(self.parse_apt_list(&stdout))
            }
            PackageManager::Dnf => {
                let output = Command::new("dnf")
                    .args(["list", "installed"])
                    .output()?;

                if !output.status.success() {
                    return Err("Failed to execute dnf list".into());
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(self.parse_dnf_list(&stdout))
            }
            PackageManager::Pacman => {
                let output = Command::new("pacman")
                    .args(["-Q"])
                    .output()?;

                if !output.status.success() {
                    return Err("Failed to execute pacman -Q".into());
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(self.parse_pacman_list(&stdout))
            }
            PackageManager::Unknown => {
                Err("No supported package manager found".into())
            }
        }
    }

    async fn scan_drivers(&self) -> Result<Vec<Driver>> {
        // Linux driver management is complex and varies by distribution
        // For now, return empty - can be extended with fwupd support
        Ok(Vec::new())
    }

    async fn check_package_updates(&self, packages: &[Package]) -> Result<Vec<(Package, String)>> {
        match self.package_manager {
            PackageManager::Apt => {
                let output = Command::new("apt")
                    .args(["list", "--upgradable"])
                    .output()?;

                if !output.status.success() {
                    return Ok(Vec::new());
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                let upgradable = self.parse_apt_upgradable(&stdout);
                
                let mut updates = Vec::new();
                for (pkg_name, new_version) in upgradable {
                    if let Some(package) = packages.iter().find(|p| p.id == pkg_name) {
                        updates.push((package.clone(), new_version));
                    }
                }

                Ok(updates)
            }
            PackageManager::Dnf => {
                let output = Command::new("dnf")
                    .args(["check-update"])
                    .output()?;

                let stdout = String::from_utf8_lossy(&output.stdout);
                let upgradable = self.parse_dnf_check_update(&stdout);
                
                let mut updates = Vec::new();
                for (pkg_name, new_version) in upgradable {
                    if let Some(package) = packages.iter().find(|p| p.id == pkg_name) {
                        updates.push((package.clone(), new_version));
                    }
                }

                Ok(updates)
            }
            PackageManager::Pacman => {
                let output = Command::new("pacman")
                    .args(["-Qu"])
                    .output()?;

                if !output.status.success() {
                    return Ok(Vec::new());
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                let upgradable = self.parse_pacman_updates(&stdout);
                
                let mut updates = Vec::new();
                for (pkg_name, new_version) in upgradable {
                    if let Some(package) = packages.iter().find(|p| p.id == pkg_name) {
                        updates.push((package.clone(), new_version));
                    }
                }

                Ok(updates)
            }
            PackageManager::Unknown => {
                Err("No supported package manager found".into())
            }
        }
    }

    async fn check_driver_updates(&self, _drivers: &[Driver]) -> Result<Vec<(Driver, String)>> {
        Ok(Vec::new())
    }

    async fn update_package(&self, package: &Package) -> Result<()> {
        match self.package_manager {
            PackageManager::Apt => {
                let output = Command::new("sudo")
                    .args(["apt", "install", "--only-upgrade", "-y", &package.id])
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to update package: {}", stderr).into());
                }

                Ok(())
            }
            PackageManager::Dnf => {
                let output = Command::new("sudo")
                    .args(["dnf", "upgrade", "-y", &package.id])
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to update package: {}", stderr).into());
                }

                Ok(())
            }
            PackageManager::Pacman => {
                let output = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", &package.id])
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to update package: {}", stderr).into());
                }

                Ok(())
            }
            PackageManager::Unknown => {
                Err("No supported package manager found".into())
            }
        }
    }

    async fn update_driver(&self, _driver: &Driver) -> Result<()> {
        Err("Driver updates not supported on Linux".into())
    }
}
