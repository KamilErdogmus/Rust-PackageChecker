use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Platform detection failed: {0}")]
    PlatformDetectionFailed(String),
    
    #[error("Package manager not found: {0}")]
    PackageManagerNotFound(String),
    
    #[error("Unsupported package manager")]
    UnsupportedPackageManager,
    
    #[error("Scan failed: {0}")]
    ScanFailed(String),
    
    #[error("Update failed: {0}")]
    UpdateFailed(String),
    
    #[error("Driver operation failed: {0}")]
    DriverOperationFailed(String),
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Other error: {0}")]
    Other(String),
    
    #[error("Not implemented")]
    NotImplemented,
}

impl Error {
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            Error::PlatformDetectionFailed(_) | Error::PermissionDenied | Error::DatabaseError(_)
        )
    }
    
    pub fn user_message(&self) -> String {
        match self {
            Error::PlatformDetectionFailed(_) => "Platform could not be detected".to_string(),
            Error::PackageManagerNotFound(pm) => format!("Package manager '{}' not found", pm),
            Error::UnsupportedPackageManager => "This package manager is not supported".to_string(),
            Error::ScanFailed(_) => "System scan failed".to_string(),
            Error::UpdateFailed(_) => "Update failed".to_string(),
            Error::DriverOperationFailed(_) => "Driver operation failed".to_string(),
            Error::PermissionDenied => "Permission denied. Administrator privileges required".to_string(),
            Error::NetworkError(_) => "Network connection error".to_string(),
            Error::DatabaseError(_) => "Database error occurred".to_string(),
            Error::IoError(_) => "File system error occurred".to_string(),
            Error::SerializationError(_) => "Data serialization error".to_string(),
            Error::Other(msg) => msg.clone(),
            Error::NotImplemented => "This feature is not yet implemented".to_string(),
        }
    }
}
