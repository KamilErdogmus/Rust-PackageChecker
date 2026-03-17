use crate::backend::{
    models::Platform,
    Error, Result,
};

#[cfg(target_os = "windows")]
use crate::backend::models::WindowsVersion;

#[cfg(target_os = "macos")]
use crate::backend::models::MacOSVersion;

#[cfg(target_os = "linux")]
use crate::backend::models::LinuxDistro;

#[cfg(target_os = "linux")]
use std::fs;

pub struct OSDetector;

impl OSDetector {
    /// Detect the current platform and version
    pub fn detect() -> Result<Platform> {
        #[cfg(target_os = "windows")]
        {
            let version = Self::get_windows_version()?;
            Ok(Platform::Windows(version))
        }

        #[cfg(target_os = "macos")]
        {
            let version = Self::get_macos_version()?;
            Ok(Platform::MacOS(version))
        }

        #[cfg(target_os = "linux")]
        {
            let distro = Self::detect_linux_distro()?;
            Ok(Platform::Linux(distro))
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformDetectionFailed(
                "Unsupported operating system".to_string(),
            ))
        }
    }

    #[cfg(target_os = "windows")]
    fn get_windows_version() -> Result<WindowsVersion> {
        use std::process::Command;

        // Use systeminfo command to get Windows version
        let output = Command::new("cmd")
            .args(&["/C", "ver"])
            .output()
            .map_err(|e| Error::PlatformDetectionFailed(format!("Failed to run ver command: {}", e)))?;

        let version_str = String::from_utf8_lossy(&output.stdout);

        // Windows 11 has build number >= 22000
        // Windows 10 has build number < 22000
        if version_str.contains("10.0.22") || version_str.contains("10.0.23") || version_str.contains("10.0.24") {
            Ok(WindowsVersion::Windows11)
        } else if version_str.contains("10.0") {
            Ok(WindowsVersion::Windows10)
        } else {
            // Default to Windows 10 if we can't determine
            Ok(WindowsVersion::Windows10)
        }
    }

    #[cfg(target_os = "macos")]
    fn get_macos_version() -> Result<MacOSVersion> {
        use std::process::Command;

        // Use sw_vers command to get macOS version
        let output = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .map_err(|e| Error::PlatformDetectionFailed(format!("Failed to run sw_vers: {}", e)))?;

        let version_str = String::from_utf8_lossy(&output.stdout);
        let version_str = version_str.trim();

        // Parse major version number
        let major_version = version_str
            .split('.')
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| {
                Error::PlatformDetectionFailed(format!("Invalid version format: {}", version_str))
            })?;

        match major_version {
            12 => Ok(MacOSVersion::Monterey),
            13 => Ok(MacOSVersion::Ventura),
            14 => Ok(MacOSVersion::Sonoma),
            15 => Ok(MacOSVersion::Sequoia),
            _ => {
                // Default to latest known version for newer versions
                if major_version > 15 {
                    Ok(MacOSVersion::Sequoia)
                } else {
                    Err(Error::PlatformDetectionFailed(format!(
                        "Unsupported macOS version: {}",
                        major_version
                    )))
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn detect_linux_distro() -> Result<LinuxDistro> {
        // Try to read /etc/os-release file
        let os_release_content = fs::read_to_string("/etc/os-release")
            .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
            .map_err(|e| {
                Error::PlatformDetectionFailed(format!("Failed to read os-release file: {}", e))
            })?;

        Self::parse_os_release(&os_release_content)
    }

    #[cfg(target_os = "linux")]
    fn parse_os_release(content: &str) -> Result<LinuxDistro> {
        let mut id = None;
        let mut name = None;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("ID=") {
                id = Some(line[3..].trim_matches('"').to_lowercase());
            } else if line.starts_with("NAME=") {
                name = Some(line[5..].trim_matches('"').to_string());
            }
        }

        let id = id.ok_or_else(|| {
            Error::PlatformDetectionFailed("ID field not found in os-release".to_string())
        })?;

        match id.as_str() {
            "debian" => Ok(LinuxDistro::Debian),
            "ubuntu" => Ok(LinuxDistro::Ubuntu),
            "fedora" => Ok(LinuxDistro::Fedora),
            "rhel" | "centos" | "rocky" | "almalinux" => Ok(LinuxDistro::RHEL),
            "arch" | "manjaro" => Ok(LinuxDistro::Arch),
            _ => Ok(LinuxDistro::Other(
                name.unwrap_or_else(|| id.clone()),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_platform() {
        // This test will run on the actual platform
        let result = OSDetector::detect();
        assert!(result.is_ok(), "Platform detection should succeed");

        let platform = result.unwrap();
        
        #[cfg(target_os = "windows")]
        {
            assert!(matches!(platform, Platform::Windows(_)));
        }

        #[cfg(target_os = "macos")]
        {
            assert!(matches!(platform, Platform::MacOS(_)));
        }

        #[cfg(target_os = "linux")]
        {
            assert!(matches!(platform, Platform::Linux(_)));
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_os_release_ubuntu() {
        let content = r#"
NAME="Ubuntu"
VERSION="22.04.1 LTS (Jammy Jellyfish)"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Ubuntu 22.04.1 LTS"
VERSION_ID="22.04"
"#;
        let result = OSDetector::parse_os_release(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LinuxDistro::Ubuntu);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_os_release_fedora() {
        let content = r#"
NAME="Fedora Linux"
VERSION="38 (Workstation Edition)"
ID=fedora
VERSION_ID=38
"#;
        let result = OSDetector::parse_os_release(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LinuxDistro::Fedora);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_os_release_arch() {
        let content = r#"
NAME="Arch Linux"
ID=arch
PRETTY_NAME="Arch Linux"
"#;
        let result = OSDetector::parse_os_release(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LinuxDistro::Arch);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_os_release_other() {
        let content = r#"
NAME="Custom Linux"
ID=customlinux
VERSION_ID=1.0
"#;
        let result = OSDetector::parse_os_release(content);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), LinuxDistro::Other(_)));
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests;
