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
}