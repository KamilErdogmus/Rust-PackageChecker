use async_trait::async_trait;
use std::process::Command;
use crate::backend::{
    error::Result,
    models::{Driver, DriverUpdate, Package, PackageManager, PackageUpdate, Platform, UpdateResult, PackageDetails, PackageCategory},
};
use super::platform_adapter::PlatformAdapter;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct WindowsAdapter;

impl WindowsAdapter {
    pub fn new() -> Self {
        Self
    }

    fn silent_process(program: &str) -> Command {
        let mut c = Command::new(program);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            c.creation_flags(CREATE_NO_WINDOW);
        }
        c
    }

    fn silent_cmd() -> Command {
        Self::silent_process("cmd")
    }

    fn is_command_available(cmd: &str) -> bool {
        Self::silent_cmd()
            .args(&["/C", &format!("where {}", cmd)])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn parse_winget_list(&self, output: &str) -> Vec<Package> {
        let mut packages = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        let mut dash_idx = None;
        for (i, line) in lines.iter().enumerate() {
            let dash_count = line.chars().filter(|&c| c == '-').count();
            if dash_count > 20 {
                dash_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = dash_idx {
            for line in lines.iter().skip(idx + 1) {
                if line.trim().is_empty() {
                    continue;
                }

                let cols: Vec<&str> = line.split("  ")
                    .flat_map(|s| s.split('\t'))
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if cols.len() < 2 {
                    continue;
                }

                let name = cols[0].replace("\x1b", "").replace("[0m", "").replace("[32m", "").to_string();
                let id = cols[1].replace("\x1b", "").replace("[0m", "").replace("[32m", "").to_string();
                let version = if cols.len() > 2 {
                    cols[2].replace("\x1b", "").replace("[0m", "").replace("[32m", "").to_string()
                } else {
                    "unknown".to_string()
                };

                if id.to_lowercase().contains("packages") || name.to_lowercase().contains("source requires") || name.contains("---") {
                    continue;
                }

                if !name.is_empty() && !id.is_empty() {
                    packages.push(Package {
                        id,
                        name,
                        version,
                        manager: PackageManager::Winget,
                        description: None,
                        installed_date: None,
                    });
                }
            }
        }

        packages
    }

    fn parse_winget_upgrades(&self, output: &str) -> Vec<PackageUpdate> {
        let mut updates = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        let mut dash_idx = None;
        for (i, line) in lines.iter().enumerate() {
            let dash_count = line.chars().filter(|&c| c == '-').count();
            if dash_count > 20 {
                dash_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = dash_idx {
            for line in lines.iter().skip(idx + 1) {
                if line.trim().is_empty() {
                    continue;
                }

                let rest_words: Vec<&str> = line.split_whitespace().collect();
                if rest_words.len() < 4 {
                    continue;
                }

                let len = rest_words.len();
                let mut new_version_idx = len - 1;

                let last_word_lower = rest_words[len - 1].to_lowercase();
                if last_word_lower.contains("winget") || last_word_lower.contains("msstore") || last_word_lower.contains("source") {
                    new_version_idx = len - 2;
                }

                if new_version_idx < 2 {
                    continue;
                }

                let new_version = rest_words[new_version_idx].replace("\x1b", "").replace("[0m", "").replace("[32m", "").to_string();
                let current_version = rest_words[new_version_idx - 1].replace("\x1b", "").replace("[0m", "").replace("[32m", "").to_string();
                let id = rest_words[new_version_idx - 2].replace("\x1b", "").replace("[0m", "").replace("[32m", "").to_string();
                let name = rest_words[0..(new_version_idx - 2)].join(" ").replace("\x1b", "").replace("[0m", "").replace("[32m", "").trim().to_string();

                if id.to_lowercase().contains("upgrades") || current_version.to_lowercase().contains("available") || name.to_lowercase().contains("source requires") {
                    continue;
                }

                if !name.is_empty() && !id.is_empty() && current_version != "unknown" && new_version != "unknown" {
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
            }
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
        let mut managers = Vec::new();
        if Self::is_command_available("winget") {
            managers.push(PackageManager::Winget);
        }
        if Self::is_command_available("choco") {
            managers.push(PackageManager::Chocolatey);
        }
        if Self::is_command_available("scoop") {
            managers.push(PackageManager::Scoop);
        }
        if Self::is_command_available("npm") {
            managers.push(PackageManager::Npm);
        }
        if Self::is_command_available("gem") {
            managers.push(PackageManager::Gem);
        }
        if Self::is_command_available("pip") || Self::is_command_available("pip3") {
            managers.push(PackageManager::Pip);
        }
        managers
    }

    async fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Winget => {
                if let Ok(output) = Self::silent_process("winget").args(&["list", "--accept-source-agreements"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(self.parse_winget_list(&stdout));
                }
                Ok(Vec::new())
            }
            PackageManager::Npm => {
                if let Ok(output) = Self::silent_cmd().args(&["/C", "npm list -g --depth=0 --json"]).output() {
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
            PackageManager::Chocolatey => {
                if let Ok(output) = Self::silent_process("choco").args(&["list", "--local-only"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut packages = Vec::new();
                    for line in stdout.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() == 2 && !line.contains("packages installed") {
                            packages.push(Package {
                                id: parts[0].to_string(),
                                name: parts[0].to_string(),
                                version: parts[1].to_string(),
                                manager: PackageManager::Chocolatey,
                                description: None,
                                installed_date: None,
                            });
                        }
                    }
                    return Ok(packages);
                }
                Ok(Vec::new())
            }
            PackageManager::Gem => {
                if let Ok(output) = Self::silent_cmd().args(&["/C", "gem list --local"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut packages = Vec::new();
                    for line in stdout.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with("***") {
                            continue;
                        }
                        // format: name (version1, version2)
                        if let Some(paren_pos) = line.find('(') {
                            let name = line[..paren_pos].trim().to_string();
                            let versions_str = &line[paren_pos + 1..];
                            let version = versions_str
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
            PackageManager::Pip => {
                let pip_cmd = if Self::is_command_available("pip3") { "pip3" } else { "pip" };
                if let Ok(output) = Self::silent_cmd()
                    .args(&["/C", &format!("{} list --format=json", pip_cmd)])
                    .output()
                {
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
            PackageManager::Scoop => {
                if let Ok(output) = Self::silent_cmd().args(&["/C", "scoop list"]).output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut packages = Vec::new();
                    let mut header_passed = false;
                    for line in stdout.lines() {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }
                        if line.starts_with("Name") && line.contains("Version") {
                            header_passed = true;
                            continue;
                        }
                        if line.starts_with("----") {
                            continue;
                        }
                        if !header_passed {
                            continue;
                        }
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            packages.push(Package {
                                id: parts[0].to_string(),
                                name: parts[0].to_string(),
                                version: parts[1].to_string(),
                                manager: PackageManager::Scoop,
                                description: None,
                                installed_date: None,
                            });
                        }
                    }
                    return Ok(packages);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        }
    }

    async fn check_package_updates(&self, _packages: &[Package]) -> Result<Vec<PackageUpdate>> {
        let mut all_updates = Vec::new();

        // Winget
        if let Ok(output) = Self::silent_process("winget").args(&["upgrade", "--include-unknown", "--accept-source-agreements"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            all_updates.extend(self.parse_winget_upgrades(&stdout));
        }

        // NPM
        if let Ok(output) = Self::silent_cmd().args(&["/C", "npm outdated -g --json"]).output() {
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

        // Chocolatey
        if Self::is_command_available("choco") {
            if let Ok(output) = Self::silent_process("choco").args(&["outdated"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut dash_found = false;
                for line in stdout.lines() {
                    if line.starts_with('-') {
                        dash_found = true;
                        continue;
                    }
                    if dash_found && !line.trim().is_empty() && line.contains('|') {
                        let parts: Vec<&str> = line.split('|').collect();
                        if parts.len() >= 3 {
                            let name = parts[0].trim();
                            let current = parts[1].trim();
                            let latest = parts[2].trim();
                            if !name.is_empty() && !current.is_empty() && !latest.is_empty() {
                                all_updates.push(PackageUpdate {
                                    package: Package {
                                        id: name.to_string(),
                                        name: name.to_string(),
                                        version: current.to_string(),
                                        manager: PackageManager::Chocolatey,
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

        // Gem (Ruby)
        if Self::is_command_available("gem") {
            if let Ok(output) = Self::silent_cmd().args(&["/C", "gem outdated"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    // format: "name (current < latest)"
                    if let Some(paren_pos) = line.find('(') {
                        let name = line[..paren_pos].trim().to_string();
                        let inside = &line[paren_pos + 1..].trim_end_matches(')');
                        // inside: "current < latest"
                        if let Some(arrow_pos) = inside.find('<') {
                            let current = inside[..arrow_pos].trim().to_string();
                            let latest = inside[arrow_pos + 1..].trim().to_string();
                            if !name.is_empty() && !current.is_empty() && !latest.is_empty() {
                                all_updates.push(PackageUpdate {
                                    package: Package {
                                        id: name.clone(),
                                        name: name.clone(),
                                        version: current,
                                        manager: PackageManager::Gem,
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
            }
        }

        // Pip
        if Self::is_command_available("pip") || Self::is_command_available("pip3") {
            let pip_cmd = if Self::is_command_available("pip3") { "pip3" } else { "pip" };
            if let Ok(output) = Self::silent_cmd()
                .args(&["/C", &format!("{} list --outdated --format=json", pip_cmd)])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
                    for item in arr {
                        if let (Some(name), Some(version), Some(latest)) = (
                            item.get("name").and_then(|v| v.as_str()),
                            item.get("version").and_then(|v| v.as_str()),
                            item.get("latest_version").and_then(|v| v.as_str()),
                        ) {
                            all_updates.push(PackageUpdate {
                                package: Package {
                                    id: name.to_string(),
                                    name: name.to_string(),
                                    version: version.to_string(),
                                    manager: PackageManager::Pip,
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

        // Scoop
        if Self::is_command_available("scoop") {
            if let Ok(output) = Self::silent_cmd().args(&["/C", "scoop status"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut header_passed = false;
                for line in stdout.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    if line.starts_with("Name") && line.contains("Installed") {
                        header_passed = true;
                        continue;
                    }
                    if line.starts_with("----") {
                        continue;
                    }
                    if !header_passed {
                        continue;
                    }
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let name = parts[0];
                        let current = parts[1];
                        let latest = parts[2];
                        if current != latest && !latest.is_empty() {
                            all_updates.push(PackageUpdate {
                                package: Package {
                                    id: name.to_string(),
                                    name: name.to_string(),
                                    version: current.to_string(),
                                    manager: PackageManager::Scoop,
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

        Ok(all_updates)
    }

    async fn apply_package_update(&self, update: &PackageUpdate) -> Result<UpdateResult> {
        let output = match update.package.manager {
            PackageManager::Winget => {
                Self::silent_process("winget")
                    .args(&["upgrade", "--id", &update.package.id, "--silent", "--accept-source-agreements", "--accept-package-agreements"])
                    .output()?
            }
            PackageManager::Npm => {
                Self::silent_cmd()
                    .args(&["/C", &format!("npm install -g {}@latest", update.package.id)])
                    .output()?
            }
            PackageManager::Chocolatey => {
                Self::silent_process("choco")
                    .args(&["upgrade", &update.package.id, "-y"])
                    .output()?
            }
            PackageManager::Gem => {
                Self::silent_cmd()
                    .args(&["/C", &format!("gem update {}", update.package.id)])
                    .output()?
            }
            PackageManager::Pip => {
                let pip_cmd = if Self::is_command_available("pip3") { "pip3" } else { "pip" };
                Self::silent_cmd()
                    .args(&["/C", &format!("{} install --upgrade {}", pip_cmd, update.package.id)])
                    .output()?
            }
            PackageManager::Scoop => {
                Self::silent_cmd()
                    .args(&["/C", &format!("scoop update {}", update.package.id)])
                    .output()?
            }
            _ => {
                return Ok(UpdateResult {
                    status: crate::backend::models::UpdateStatus::Failed,
                    error: Some("Unsupported package manager for update".to_string()),
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
            let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
            let out_msg = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(UpdateResult {
                status: crate::backend::models::UpdateStatus::Failed,
                error: Some(format!("{} | {}", error_msg, out_msg)),
                duration: chrono::Duration::seconds(0),
            })
        }
    }

    async fn rollback_package(&self, package: &Package, target_version: &str) -> Result<UpdateResult> {
        let output = match package.manager {
            PackageManager::Winget => {
                Self::silent_process("winget")
                    .args(&["install", "--id", &package.id, "-v", target_version, "--silent", "--force", "--accept-source-agreements", "--accept-package-agreements"])
                    .output()?
            }
            PackageManager::Npm => {
                Self::silent_cmd()
                    .args(&["/C", &format!("npm install -g {}@{}", package.id, target_version)])
                    .output()?
            }
            PackageManager::Chocolatey => {
                Self::silent_process("choco")
                    .args(&["install", &package.id, "--version", target_version, "-y", "--allow-downgrade"])
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
            let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
            let out_msg = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(UpdateResult {
                status: crate::backend::models::UpdateStatus::Failed,
                error: Some(format!("{} | {}", error_msg, out_msg)),
                duration: chrono::Duration::seconds(0),
            })
        }
    }

    async fn scan_drivers(&self) -> Result<Vec<Driver>> {
        Ok(Vec::new())
    }

    async fn check_driver_updates(&self, _drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        Ok(Vec::new())
    }

    async fn apply_driver_update(&self, _update: &DriverUpdate) -> Result<UpdateResult> {
        Err(crate::backend::error::Error::NotImplemented)
    }

    async fn backup_driver(&self, _driver: &Driver) -> Result<std::path::PathBuf> {
        Err(crate::backend::error::Error::NotImplemented)
    }

    async fn restore_driver(&self, _backup_path: &std::path::PathBuf) -> Result<()> {
        Err(crate::backend::error::Error::NotImplemented)
    }

    fn requires_elevation(&self, _operation: &str) -> bool {
        true
    }

    async fn request_elevation(&self) -> Result<()> {
        Ok(())
    }

    async fn get_package_details(&self, package_id: &str) -> Result<Option<PackageDetails>> {
        let output = Self::silent_process("winget")
            .args(&["show", "--id", package_id, "--accept-source-agreements"])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_winget_show(&stdout))
    }
}
