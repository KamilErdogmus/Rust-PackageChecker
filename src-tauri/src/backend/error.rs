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
