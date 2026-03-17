// Platform adapters - Platform-specific implementations
pub mod platform_adapter;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

pub use platform_adapter::PlatformAdapter;

#[cfg(target_os = "windows")]
pub use windows::WindowsAdapter;

// Factory function to create platform adapter
pub fn create_platform_adapter() -> std::sync::Arc<dyn PlatformAdapter> {
    #[cfg(target_os = "windows")]
    {
        std::sync::Arc::new(WindowsAdapter::new())
    }
    
    #[cfg(target_os = "macos")]
    {
        std::sync::Arc::new(macos::MacOSAdapter::new())
    }
    
    #[cfg(target_os = "linux")]
    {
        std::sync::Arc::new(linux::LinuxAdapter::new())
    }
}
