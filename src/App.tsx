import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import type { ScanResult, UpdateInfo, UpdateResult, UpdateHistoryEntry, PackageCategory } from "./types";

function App() {
  const [scanResult, setScanResult] = useState<ScanResult | null>(null);
  const [updates, setUpdates] = useState<UpdateInfo[]>([]);
  const [isScanning, setIsScanning] = useState(false);
  const [isCheckingUpdates, setIsCheckingUpdates] = useState(false);
  const [isUpdating, setIsUpdating] = useState(false);
  const [selectedUpdates, setSelectedUpdates] = useState<Set<number>>(new Set());
  const [message, setMessage] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [hasCheckedUpdates, setHasCheckedUpdates] = useState(false);
  const [currentlyUpdating, setCurrentlyUpdating] = useState<string>("");
  const [updateHistory, setUpdateHistory] = useState<UpdateHistoryEntry[]>([]);
  const [categoryFilter, setCategoryFilter] = useState<PackageCategory | 'All'>('All');
  const [sortBy, setSortBy] = useState<'name' | 'size' | 'priority'>('name');
  const [showHistory, setShowHistory] = useState(false);

  const handleScan = async () => {
    try {
      setIsScanning(true);
      setMessage("Scanning system...");
      const result = await invoke<ScanResult>("scan_system");
      setScanResult(result);
      setMessage(`Found ${result.packages.length} packages`);
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setIsScanning(false);
    }
  };

  const handleCheckUpdates = async () => {
    if (!scanResult) return;
    
    try {
      setIsCheckingUpdates(true);
      setMessage("Checking for updates...");
      const updateList = await invoke<UpdateInfo[]>("check_updates", { scanResult });
      setUpdates(updateList);
      setHasCheckedUpdates(true);
      setMessage(updateList.length > 0 ? `${updateList.length} updates available` : "System is up to date");
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setIsCheckingUpdates(false);
    }
  };

  const handleUpdateSingle = async (update: UpdateInfo, index: number) => {
    const packageName = getUpdateName(update);
    try {
      setIsUpdating(true);
      setCurrentlyUpdating(packageName);
      setMessage("Installing update...");
      const result = await invoke<UpdateResult>("apply_update", { update });
      
      if (result.status === "Completed") {
        setMessage(`${packageName} updated successfully`);
        setUpdates(prev => prev.filter((_, i) => i !== index));
        
        // Add to history
        const versions = getUpdateVersion(update);
        setUpdateHistory(prev => [{
          package_id: getUpdateId(update),
          package_name: packageName,
          old_version: versions.current,
          new_version: versions.new,
          timestamp: new Date().toISOString(),
          status: 'Success',
        }, ...prev]);
      } else {
        setMessage(`Failed to update ${packageName}: ${result.error || "Unknown error"}`);
      }
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setIsUpdating(false);
      setCurrentlyUpdating("");
    }
  };

  const handleUpdateSelected = async () => {
    if (selectedUpdates.size === 0) return;
    
    try {
      setIsUpdating(true);
      const selectedUpdateList = Array.from(selectedUpdates).map(i => updates[i]);
      setMessage(`Updating ${selectedUpdateList.length} packages...`);
      
      // Update each package one by one to show progress
      let successful = 0;
      let failed = 0;
      
      for (const update of selectedUpdateList) {
        const packageName = getUpdateName(update);
        try {
          setCurrentlyUpdating(packageName);
          const result = await invoke<UpdateResult>("apply_update", { update });
          
          if (result.status === "Completed") {
            successful++;
          } else {
            failed++;
          }
        } catch {
          failed++;
        }
      }
      
      setMessage(`Batch update completed: ${successful} successful, ${failed} failed`);
      setUpdates(prev => prev.filter((_, i) => !selectedUpdates.has(i)));
      setSelectedUpdates(new Set());
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setIsUpdating(false);
      setCurrentlyUpdating("");
    }
  };

  const toggleUpdateSelection = (index: number) => {
    setSelectedUpdates(prev => {
      const newSet = new Set(prev);
      if (newSet.has(index)) {
        newSet.delete(index);
      } else {
        newSet.add(index);
      }
      return newSet;
    });
  };

  const selectAll = () => {
    setSelectedUpdates(new Set(filteredUpdates.map((update) => updates.indexOf(update))));
  };

  const deselectAll = () => {
    setSelectedUpdates(new Set());
  };

  const getUpdateName = (update: UpdateInfo): string => {
    if ('Package' in update) {
      return update.Package.package.name;
    } else if ('Driver' in update) {
      return update.Driver.driver.name;
    }
    return "Unknown";
  };

  const getUpdateId = (update: UpdateInfo): string => {
    if ('Package' in update) {
      return update.Package.package.id;
    } else if ('Driver' in update) {
      return update.Driver.driver.id;
    }
    return "";
  };

  const getUpdateVersion = (update: UpdateInfo): { current: string; new: string } => {
    if ('Package' in update) {
      return {
        current: update.Package.package.version,
        new: update.Package.new_version
      };
    } else if ('Driver' in update) {
      return {
        current: update.Driver.driver.version,
        new: update.Driver.new_version
      };
    }
    return { current: "?", new: "?" };
  };

  const getUpdateCategory = (update: UpdateInfo): PackageCategory => {
    if ('Package' in update) {
      const id = update.Package.package.id.toLowerCase();
      if (id.includes('visual') || id.includes('code') || id.includes('git') || id.includes('python') || id.includes('node')) {
        return 'Development';
      } else if (id.includes('driver') || id.includes('runtime') || id.includes('framework')) {
        return 'System';
      } else if (id.includes('media') || id.includes('vlc') || id.includes('spotify')) {
        return 'Media';
      } else if (id.includes('office') || id.includes('adobe')) {
        return 'Productivity';
      } else if (id.includes('steam') || id.includes('epic') || id.includes('game')) {
        return 'Gaming';
      } else if (id.includes('mongo') || id.includes('postgres') || id.includes('mysql')) {
        return 'Database';
      }
    }
    return 'Other';
  };

  const filteredUpdates = updates
    .filter(update => {
      const name = getUpdateName(update).toLowerCase();
      const id = getUpdateId(update).toLowerCase();
      const query = searchQuery.toLowerCase();
      const matchesSearch = name.includes(query) || id.includes(query);
      
      if (categoryFilter === 'All') {
        return matchesSearch;
      }
      
      return matchesSearch && getUpdateCategory(update) === categoryFilter;
    })
    .sort((a, b) => {
      if (sortBy === 'name') {
        return getUpdateName(a).localeCompare(getUpdateName(b));
      } else if (sortBy === 'priority') {
        const priorityA = 'Package' in a ? a.Package.priority : 'Normal';
        const priorityB = 'Package' in b ? b.Package.priority : 'Normal';
        const priorityOrder = { Critical: 3, Important: 2, Normal: 1 };
        return priorityOrder[priorityB as keyof typeof priorityOrder] - priorityOrder[priorityA as keyof typeof priorityOrder];
      }
      return 0;
    });

  // Auto-scan on mount removed - user should manually trigger scan

  return (
    <div className="app">
      <header className="header">
        <div className="header-content">
          <div className="logo">
            <svg className="logo-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
            </svg>
            <h1>System Update Manager</h1>
          </div>
          <div className="header-actions">
            <button onClick={() => setShowHistory(!showHistory)} className="btn-ghost">
              <svg className="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <circle cx="12" cy="12" r="10"/>
                <polyline points="12 6 12 12 16 14"/>
              </svg>
              {showHistory ? "Show Updates" : "History"}
            </button>
            <button onClick={handleScan} disabled={isScanning} className="btn-secondary">
              <svg className="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <circle cx="11" cy="11" r="8"/>
                <path d="m21 21-4.35-4.35"/>
              </svg>
              {isScanning ? "Scanning..." : "Scan System"}
            </button>
            <button onClick={handleCheckUpdates} disabled={!scanResult || isCheckingUpdates} className="btn-primary">
              <svg className="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
              </svg>
              {isCheckingUpdates ? "Checking..." : "Check Updates"}
            </button>
          </div>
        </div>
      </header>

      <main className="main-content">
        {message && (
          <div className={`notification ${message.includes('Error') ? 'error' : message.includes('success') ? 'success' : 'info'}`}>
            <svg className="notification-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              {message.includes('Error') ? (
                <><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></>
              ) : message.includes('success') ? (
                <><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></>
              ) : (
                <><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></>
              )}
            </svg>
            <span>{message}</span>
          </div>
        )}

        {currentlyUpdating && (
          <div className="update-progress">
            <div className="progress-spinner"></div>
            <div className="progress-text">
              <div className="progress-title">Updating Package</div>
              <div className="progress-package">{currentlyUpdating}</div>
            </div>
          </div>
        )}

        {scanResult && (
          <div className="stats-grid">
            <div className="stat-card">
              <svg className="stat-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
              </svg>
              <div className="stat-content">
                <div className="stat-value">{scanResult.packages.length}</div>
                <div className="stat-label">Total Packages</div>
              </div>
            </div>
            <div className="stat-card">
              <svg className="stat-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
              </svg>
              <div className="stat-content">
                <div className="stat-value">{updates.length}</div>
                <div className="stat-label">Updates Available</div>
              </div>
            </div>
            <div className="stat-card">
              <svg className="stat-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                <polyline points="22 4 12 14.01 9 11.01"/>
              </svg>
              <div className="stat-content">
                <div className="stat-value">{selectedUpdates.size}</div>
                <div className="stat-label">Selected</div>
              </div>
            </div>
          </div>
        )}

        {scanResult && updates.length === 0 && !hasCheckedUpdates && (
          <div className="updates-container">
            <div className="updates-toolbar">
              <div className="search-box">
                <svg className="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <circle cx="11" cy="11" r="8"/>
                  <path d="m21 21-4.35-4.35"/>
                </svg>
                <input
                  type="text"
                  placeholder="Search packages..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="search-input"
                />
              </div>
            </div>

            <div className="updates-list">
              {scanResult.packages
                .filter(pkg => {
                  const query = searchQuery.toLowerCase();
                  return pkg.name.toLowerCase().includes(query) || pkg.id.toLowerCase().includes(query);
                })
                .map((pkg, index) => (
                  <div key={index} className="update-card">
                    <div className="update-details">
                      <div className="update-header">
                        <h3 className="update-name">{pkg.name}</h3>
                        <span className="update-id">{pkg.id}</span>
                      </div>
                      <div className="update-version">
                        <span className="version-badge current">{pkg.version}</span>
                      </div>
                    </div>
                  </div>
                ))}
            </div>
          </div>
        )}

        {updates.length > 0 && !showHistory && (
          <div className="updates-container">
            <div className="updates-toolbar">
              <div className="search-box">
                <svg className="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <circle cx="11" cy="11" r="8"/>
                  <path d="m21 21-4.35-4.35"/>
                </svg>
                <input
                  type="text"
                  placeholder="Search packages..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="search-input"
                />
              </div>
              <div className="filter-group">
                <select 
                  value={categoryFilter} 
                  onChange={(e) => setCategoryFilter(e.target.value as PackageCategory | 'All')}
                  className="filter-select"
                >
                  <option value="All">All Categories</option>
                  <option value="Development">Development</option>
                  <option value="System">System</option>
                  <option value="Media">Media</option>
                  <option value="Productivity">Productivity</option>
                  <option value="Gaming">Gaming</option>
                  <option value="Security">Security</option>
                  <option value="Network">Network</option>
                  <option value="Database">Database</option>
                  <option value="Other">Other</option>
                </select>
                <select 
                  value={sortBy} 
                  onChange={(e) => setSortBy(e.target.value as 'name' | 'size' | 'priority')}
                  className="filter-select"
                >
                  <option value="name">Sort by Name</option>
                  <option value="priority">Sort by Priority</option>
                </select>
              </div>
              <div className="toolbar-actions">
                <button onClick={selectAll} disabled={isUpdating} className="btn-ghost">
                  Select All
                </button>
                <button onClick={deselectAll} disabled={isUpdating} className="btn-ghost">
                  Clear
                </button>
                <button 
                  onClick={handleUpdateSelected} 
                  disabled={selectedUpdates.size === 0 || isUpdating}
                  className="btn-success"
                >
                  <svg className="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <line x1="12" y1="19" x2="12" y2="5"/>
                    <polyline points="5 12 12 5 19 12"/>
                  </svg>
                  {isUpdating ? "Updating..." : `Update Selected (${selectedUpdates.size})`}
                </button>
              </div>
            </div>

            <div className="updates-list">
              {filteredUpdates.map((update) => {
                const index = updates.indexOf(update);
                const versions = getUpdateVersion(update);
                const name = getUpdateName(update);
                const id = getUpdateId(update);
                
                return (
                  <div key={index} className={`update-card ${selectedUpdates.has(index) ? 'selected' : ''}`}>
                    <div className="update-checkbox">
                      <input
                        type="checkbox"
                        checked={selectedUpdates.has(index)}
                        onChange={() => toggleUpdateSelection(index)}
                        disabled={isUpdating}
                      />
                    </div>
                    <div className="update-details">
                      <div className="update-header">
                        <h3 className="update-name">{name}</h3>
                        <span className="update-id">{id}</span>
                      </div>
                      <div className="update-version">
                        <span className="version-badge current">{versions.current}</span>
                        <svg className="version-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                          <line x1="5" y1="12" x2="19" y2="12"/>
                          <polyline points="12 5 19 12 12 19"/>
                        </svg>
                        <span className="version-badge new">{versions.new}</span>
                      </div>
                    </div>
                    <button
                      onClick={() => handleUpdateSingle(update, index)}
                      disabled={isUpdating}
                      className="btn-update"
                    >
                      <svg className="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <line x1="12" y1="19" x2="12" y2="5"/>
                        <polyline points="5 12 12 5 19 12"/>
                      </svg>
                      Update
                    </button>
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {updates.length === 0 && scanResult && hasCheckedUpdates && !isCheckingUpdates && (
          <div className="empty-state">
            <svg className="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
              <polyline points="22 4 12 14.01 9 11.01"/>
            </svg>
            <h2>System is Up to Date</h2>
            <p>All packages are running the latest versions</p>
          </div>
        )}

        {!scanResult && !isScanning && (
          <div className="empty-state">
            <svg className="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <circle cx="11" cy="11" r="8"/>
              <path d="m21 21-4.35-4.35"/>
            </svg>
            <h2>Ready to Scan</h2>
            <p>Click "Scan System" to discover installed packages</p>
          </div>
        )}

        {showHistory && (
          <div className="updates-container">
            <div className="updates-toolbar">
              <h2 style={{ margin: 0, fontSize: '1.25rem' }}>Update History</h2>
            </div>
            <div className="updates-list">
              {updateHistory.length === 0 ? (
                <div className="empty-state" style={{ padding: '2rem' }}>
                  <p>No update history yet</p>
                </div>
              ) : (
                updateHistory.map((entry, index) => (
                  <div key={index} className="update-card">
                    <div className="update-details">
                      <div className="update-header">
                        <h3 className="update-name">{entry.package_name}</h3>
                        <span className="update-id">{entry.package_id}</span>
                        <span className={`status-badge ${entry.status.toLowerCase()}`}>
                          {entry.status}
                        </span>
                      </div>
                      <div className="update-version">
                        <span className="version-badge current">{entry.old_version}</span>
                        <svg className="version-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                          <line x1="5" y1="12" x2="19" y2="12"/>
                          <polyline points="12 5 19 12 12 19"/>
                        </svg>
                        <span className="version-badge new">{entry.new_version}</span>
                        <span style={{ marginLeft: '1rem', fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                          {new Date(entry.timestamp).toLocaleString()}
                        </span>
                      </div>
                      {entry.error_message && (
                        <div style={{ marginTop: '0.5rem', fontSize: '0.875rem', color: 'var(--error)' }}>
                          {entry.error_message}
                        </div>
                      )}
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
