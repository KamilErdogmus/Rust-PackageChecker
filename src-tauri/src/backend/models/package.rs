use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageManager {
    // Windows
    Winget,
    Chocolatey,
    Scoop,
    
    // Cross-platform
    Npm,
    Cargo,
    
    // macOS
    Homebrew,
    MacAppStore,  // mas-cli
    
    // Linux
    Apt,
    Dnf,
    Pacman,
    Flatpak,
    Snap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manager: PackageManager,
    pub description: Option<String>,
    pub installed_date: Option<DateTime<Utc>>,
}
