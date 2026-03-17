use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDetails {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub install_date: Option<String>,
    pub size: Option<String>,
    pub homepage: Option<String>,
    pub category: PackageCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PackageCategory {
    Development,
    System,
    Media,
    Productivity,
    Gaming,
    Security,
    Network,
    Database,
    Other,
}

impl PackageCategory {
    pub fn from_package_id(id: &str) -> Self {
        let id_lower = id.to_lowercase();
        
        if id_lower.contains("visual") || id_lower.contains("code") || id_lower.contains("git") 
            || id_lower.contains("python") || id_lower.contains("node") || id_lower.contains("rust") {
            PackageCategory::Development
        } else if id_lower.contains("driver") || id_lower.contains("runtime") || id_lower.contains("framework") {
            PackageCategory::System
        } else if id_lower.contains("media") || id_lower.contains("vlc") || id_lower.contains("spotify") {
            PackageCategory::Media
        } else if id_lower.contains("office") || id_lower.contains("adobe") || id_lower.contains("notion") {
            PackageCategory::Productivity
        } else if id_lower.contains("steam") || id_lower.contains("epic") || id_lower.contains("game") {
            PackageCategory::Gaming
        } else if id_lower.contains("antivirus") || id_lower.contains("firewall") || id_lower.contains("security") {
            PackageCategory::Security
        } else if id_lower.contains("chrome") || id_lower.contains("firefox") || id_lower.contains("network") {
            PackageCategory::Network
        } else if id_lower.contains("mongo") || id_lower.contains("postgres") || id_lower.contains("mysql") || id_lower.contains("redis") {
            PackageCategory::Database
        } else {
            PackageCategory::Other
        }
    }
}
