// Backend module - Core business logic
pub mod models;
pub mod services;
pub mod database;
pub mod error;
pub mod os_detector;

pub use error::{Error, Result};
pub use os_detector::OSDetector;
