// TypeScript type definitions for frontend

export interface Platform {
  type: 'Windows' | 'MacOS' | 'Linux';
  version?: string;
}

export interface Package {
  id: string;
  name: string;
  version: string;
  manager: PackageManager;
  description?: string;
  installed_date?: string;
}

export type PackageManager = 
  | 'Winget' 
  | 'Chocolatey' 
  | 'Homebrew' 
  | 'MacAppStore' 
  | 'Apt' 
  | 'Dnf' 
  | 'Pacman' 
  | 'Flatpak' 
  | 'Snap';

export interface Driver {
  id: string;
  name: string;
  version: string;
  device_type: DeviceType;
  manufacturer: string;
  driver_date?: string;
}

export type DeviceType = 
  | 'Graphics' 
  | 'Network' 
  | 'Audio' 
  | 'Storage' 
  | 'Input' 
  | { Other: string };

export interface Update {
  type: 'Package' | 'Driver';
  data: PackageUpdate | DriverUpdate;
}

export interface PackageUpdate {
  package: Package;
  new_version: string;
  size_bytes?: number;
  priority: UpdatePriority;
  changelog?: string;
}

export interface DriverUpdate {
  driver: Driver;
  new_version: string;
  size_bytes?: number;
  requires_reboot: boolean;
  release_notes?: string;
}

export type UpdatePriority = 'Critical' | 'Important' | 'Normal';

export type UpdateStatus = 'Completed' | 'Failed' | 'RequiresReboot';

// Scanner types
export interface ScanResult {
  platform: Platform;
  packages: Package[];
  drivers: Driver[];
}

export type UpdateInfo = 
  | { Package: PackageUpdate }
  | { Driver: DriverUpdate };

export interface UpdateResult {
  status: UpdateStatus;
  error?: string;
  duration: any; // chrono::Duration
}

export interface UpdateResultSummary {
  name: string;
  status: UpdateStatus;
  error?: string;
}

export interface BatchUpdateResult {
  successful: number;
  failed: number;
  results: UpdateResultSummary[];
}

// Package details
export interface PackageDetails {
  id: string;
  name: string;
  version: string;
  description?: string;
  publisher?: string;
  install_date?: string;
  size?: string;
  homepage?: string;
  category: PackageCategory;
}

export type PackageCategory = 
  | 'Development' 
  | 'System' 
  | 'Media' 
  | 'Productivity' 
  | 'Gaming' 
  | 'Security' 
  | 'Network' 
  | 'Database' 
  | 'Other';

// Update history
export interface UpdateHistoryEntry {
  package_id: string;
  package_name: string;
  old_version: string;
  new_version: string;
  timestamp: string;
  status: 'Success' | 'Failed' | 'Rollback';
  error_message?: string;
}

