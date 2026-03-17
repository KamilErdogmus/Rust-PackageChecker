use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub id: String,
    pub name: String,
    pub version: String,
    pub device_type: DeviceType,
    pub manufacturer: String,
    pub driver_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Graphics,
    Network,
    Audio,
    Storage,
    Input,
    Other(String),
}
