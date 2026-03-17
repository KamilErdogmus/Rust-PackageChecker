use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Windows(WindowsVersion),
    MacOS(MacOSVersion),
    Linux(LinuxDistro),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowsVersion {
    Windows10,
    Windows11,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacOSVersion {
    Monterey,    // 12
    Ventura,     // 13
    Sonoma,      // 14
    Sequoia,     // 15
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinuxDistro {
    Debian,
    Ubuntu,
    Fedora,
    RHEL,
    Arch,
    Other(String),
}
