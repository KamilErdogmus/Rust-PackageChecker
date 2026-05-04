use async_trait::async_trait;
use std::process::Command;
use std::path::PathBuf;
use crate::backend::{
    error::Result,
    models::{Driver, DriverUpdate, Package, PackageManager, PackageUpdate, Platform, UpdateResult, PackageDetails, LinuxDistro},
};
use super::platform_adapter::PlatformAdapter;

pub struct LinuxAdapter;

impl LinuxAdapter {
    pub fn new() -> Self {
        Self
    }

    fn detect_system_package_manager() -> Option<PackageManager> {
        if Command::new("apt").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            Some(PackageManager::Apt)
        } else if Command::new("dnf").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            Some(PackageManager::Dnf)
        } else if Command::new("pacman").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            Some(PackageManager::Pacman)
        } else {
            None
        }
    }

    fn is_command_available(cmd: &str) -> bool {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn parse_apt_list(output: &str) -> Vec<Package> {
        output
            .lines()
            .filter(|line| !line.starts_with("Listing...") && !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('/').collect();
                if parts.is_empty() {
                    return None;
                }
                let name = parts[0].trim().to_string();
                let version = line.split_whitespace().nth(1).unwrap_or("unknown").to_string();
                if name.is_empty() {
                    return None;
                }
                Some(Package {
                    id: name.clone(),
                    name,
                    version,
                    manager: PackageManager::Apt,
                    description: None,
                    installed_date: None,
                })
            })
            .collect()
    }

    fn parse_dnf_list(output: &str) -> Vec<Package> {
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
                    manager: PackageManager::Dnf,
                    description: None,
                    installed_date: None,
                })
            })
            .collect()
    }

    fn parse_pacman_list(output: &str) -> Vec<Package> {
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
                    manager: PackageManager::Pacman,
                    description: None,
                    installed_date: None,
                })
            })
            .collect()
    }
}

#[async_trait]
impl PlatformAdapter for LinuxAdapter {
    fn detect_platform(&self) -> Platform {
        Platform::Linux(LinuxDistro::Other("Linux".to_string()))
    }

    fn get_package_managers(&self) -> Vec<PackageManager> {
        let mut managers = Vec::new();
        if let Some(sys_pm) = Self::detect_system_package_manager() {
            managers.push(sys_pm);
        }
        if Self::is_command_available("flatpak") {
            managers.push(PackageManager::Flatpak);
        }
        if Self::is_command_available("snap") {
            managers.push(PackageManager::Snap);
        }
        if Self::is_command_available("npm") {
            managers.push(PackageManager::Npm);
        }
        if Self::is_command_available("pip") || Self::is_command_available("pip3") {
            managers.push(PackageManager::Pip);
        }
        if Self::is_command_available("gem") {
            managers.push(PackageManager::Gem);
        }
        managers
    }

    async fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Apt => {
                if let Ok(output) = Command::new("apt").args(["list", "--installed"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(Self::parse_apt_list(&stdout));
                }
                Ok(Vec::new())
            }
            PackageManager::Dnf => {
                if let Ok(output) = Command::new("dnf").args(["list", "installed"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(Self::parse_dnf_list(&stdout));
                }
                Ok(Vec::new())
            }
            PackageManager::Pacman => {
                if let Ok(output) = Command::new("pacman").args(["-Q"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(Self::parse_pacman_list(&stdout));
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
            PackageManager::Pip => {
                let pip_cmd = if Self::is_command_available("pip3") { "pip3" } else { "pip" };
                if let Ok(output) = Command::new(pip_cmd).args(&["list", "--format=json"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
                        let mut packages = Vec::new();
                        for item in arr {
                            if let (Some(name), Some(version)) = (
                                item.get("name").and_then(|v| v.as_str()),
                                item.get("version").and_then(|v| v.as_str()),
                            ) {
                                packages.push(Package {
                                    id: name.to_string(),
                                    name: name.to_string(),
                                    version: version.to_string(),
                                    manager: PackageManager::Pip,
                                    description: None,
                                    installed_date: None,
                                });
                            }
                        }
                        return Ok(packages);
                    }
                }
                Ok(Vec::new())
            }
            PackageManager::Gem => {
                if let Ok(output) = Command::new("gem").args(&["list", "--local"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut packages = Vec::new();
                    for line in stdout.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with("***") {
                            continue;
                        }
                        if let Some(paren_pos) = line.find('(') {
                            let name = line[..paren_pos].trim().to_string();
                            let version = line[paren_pos + 1..]
                                .trim_end_matches(')')
                                .split(',')
                                .next()
                                .unwrap_or("unknown")
                                .trim()
                                .to_string();
                            if !name.is_empty() {
                                packages.push(Package {
                                    id: name.clone(),
                                    name,
                                    version,
                                    manager: PackageManager::Gem,
                                    description: None,
                                    installed_date: None,
                                });
                            }
                        }
                    }
                    return Ok(packages);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        }
    }

    async fn scan_drivers(&self) -> Result<Vec<Driver>> {
        Ok(Vec::new())
    }

    async fn check_package_updates(&self, _packages: &[Package]) -> Result<Vec<PackageUpdate>> {
        let mut updates = Vec::new();

        if Self::is_command_available("apt") {
            if let Ok(output) = Command::new("apt").args(["list", "--upgradable"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.starts_with("Listing...") || line.trim().is_empty() {
                        continue;
                    }
                    let parts: Vec<&str> = line.split('/').collect();
                    if parts.is_empty() { continue; }
                    let name = parts[0].trim().to_string();
                    if let (Some(b), Some(e)) = (line.find('['), line.find(']')) {
                        let new_version = line[b + 1..e].trim().to_string();
                        updates.push(PackageUpdate {
                            package: Package {
                                id: name.clone(),
                                name,
                                version: "unknown".to_string(),
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
        }

        if Self::is_command_available("dnf") {
            if let Ok(output) = Command::new("dnf").args(["check-update"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.trim().is_empty() || line.contains("Last metadata") { continue; }
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name = parts[0].split('.').next().unwrap_or("").to_string();
                        let new_version = parts[1].to_string();
                        if !name.is_empty() {
                            updates.push(PackageUpdate {
                                package: Package {
                                    id: name.clone(),
                                    name,
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
            }
        }

        if Self::is_command_available("pacman") {
            if let Ok(output) = Command::new("pacman").args(["-Qu"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.trim().is_empty() { continue; }
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let name = parts[0].to_string();
                        let new_version = parts[3].to_string();
                        updates.push(PackageUpdate {
                            package: Package {
                                id: name.clone(),
                                name,
                                version: parts[1].to_string(),
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
        }

        if Self::is_command_available("npm") {
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
        }

        Ok(updates)
    }

    async fn check_driver_updates(&self, _drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        Ok(Vec::new())
    }

    async fn rollback_package(&self, _package: &Package, _target_version: &str) -> Result<UpdateResult> {
        Ok(UpdateResult {
            status: crate::backend::models::UpdateStatus::Failed,
            error: Some("Rollback not supported on Linux".to_string()),
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
            PackageManager::Pip => {
                let pip_cmd = if Self::is_command_available("pip3") { "pip3" } else { "pip" };
                Command::new(pip_cmd)
                    .args(["install", "--upgrade", &update.package.id])
                    .output()?
            }
            PackageManager::Gem => {
                Command::new("gem")
                    .args(["update", &update.package.id])
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
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(UpdateResult {
                status: crate::backend::models::UpdateStatus::Failed,
                error: Some(stderr),
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
