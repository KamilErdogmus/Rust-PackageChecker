import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import type {
  ScanResult,
  UpdateInfo,
  UpdateResult,
  UpdateHistoryEntry,
  PackageCategory,
} from "../types";

export function usePackageManager() {
  const [scanResult, setScanResult] = useState<ScanResult | null>(null);
  const [updates, setUpdates] = useState<UpdateInfo[]>([]);
  const [isCheckingUpdates, setIsCheckingUpdates] = useState(false);
  const [isUpdating, setIsUpdating] = useState(false);
  const [selectedUpdates, setSelectedUpdates] = useState<Set<number>>(
    new Set(),
  );
  const [searchQuery, setSearchQuery] = useState("");
  const [hasCheckedUpdates, setHasCheckedUpdates] = useState(false);
  const [currentlyUpdating, setCurrentlyUpdating] = useState<string>("");
  const [updateHistory, setUpdateHistory] = useState<UpdateHistoryEntry[]>([]);
  const [ignoredPackages, setIgnoredPackages] = useState<Set<string>>(() => {
    const saved = localStorage.getItem("ignoredPackages");
    return saved ? new Set(JSON.parse(saved)) : new Set();
  });
  const [categoryFilter, setCategoryFilter] = useState<PackageCategory | "All">(
    "All",
  );
  const [sortBy, setSortBy] = useState<"name" | "size" | "priority">("name");

  const getUpdateName = (update: UpdateInfo): string => {
    if ("Package" in update) return update.Package.package.name;
    if ("Driver" in update) return update.Driver.driver.name;
    return "Unknown";
  };

  const getUpdateId = (update: UpdateInfo): string => {
    if ("Package" in update) return update.Package.package.id;
    if ("Driver" in update) return update.Driver.driver.id;
    return "";
  };

  const getUpdateVersion = (
    update: UpdateInfo,
  ): { current: string; new: string } => {
    if ("Package" in update) {
      return {
        current: update.Package.package.version,
        new: update.Package.new_version,
      };
    }
    if ("Driver" in update) {
      return {
        current: update.Driver.driver.version,
        new: update.Driver.new_version,
      };
    }
    return { current: "?", new: "?" };
  };

  const getUpdateManager = (update: UpdateInfo) => {
    if ("Package" in update) {
      return update.Package.package.manager;
    }
    return "Winget";
  };

  const getUpdateCategory = (update: UpdateInfo): PackageCategory => {
    if ("Package" in update) {
      const id = update.Package.package.id.toLowerCase();
      if (
        id.includes("visual") ||
        id.includes("code") ||
        id.includes("git") ||
        id.includes("python") ||
        id.includes("node")
      )
        return "Development";
      if (
        id.includes("driver") ||
        id.includes("runtime") ||
        id.includes("framework")
      )
        return "System";
      if (id.includes("media") || id.includes("vlc") || id.includes("spotify"))
        return "Media";
      if (id.includes("office") || id.includes("adobe")) return "Productivity";
      if (id.includes("steam") || id.includes("epic") || id.includes("game"))
        return "Gaming";
      if (
        id.includes("mongo") ||
        id.includes("postgres") ||
        id.includes("mysql")
      )
        return "Database";
    }
    return "Other";
  };

  const handleCheckUpdates = async () => {
    try {
      setIsCheckingUpdates(true);
      toast.info("Scanning system for packages...");

      const result = await invoke<ScanResult>("scan_system");
      setScanResult(result);

      toast.info("Checking for updates...");
      const updateList = await invoke<UpdateInfo[]>("check_updates", {
        scanResult: result,
      });
      setUpdates(updateList);
      setHasCheckedUpdates(true);

      if (updateList.length > 0) {
        toast.success(`Found ${updateList.length} updates`);
      } else {
        toast.success("System is up to date");
      }
    } catch (error) {
      toast.error(`Scan error: ${error}`);
    } finally {
      setIsCheckingUpdates(false);
    }
  };

  const handleUpdateSingle = async (update: UpdateInfo, index: number) => {
    const packageName = getUpdateName(update);
    try {
      setIsUpdating(true);
      setCurrentlyUpdating(packageName);
      toast.loading(`Installing update for ${packageName}...`);
      const result = await invoke<UpdateResult>("apply_update", { update });

      if (result.status === "Completed") {
        toast.dismiss();
        toast.success(`${packageName} updated successfully`);
        setUpdates((prev) => prev.filter((_, i) => i !== index));

        const versions = getUpdateVersion(update);
        setUpdateHistory((prev) => [
          {
            package_id: getUpdateId(update),
            package_name: packageName,
            old_version: versions.current,
            new_version: versions.new,
            timestamp: new Date().toISOString(),
            status: "Success",
            manager: getUpdateManager(update),
          },
          ...prev,
        ]);
      } else {
        toast.dismiss();
        toast.error(
          `Failed to update ${packageName}: ${result.error || "Unknown error"}`,
        );
      }
    } catch (error) {
      toast.dismiss();
      toast.error(`Update error: ${error}`);
    } finally {
      setIsUpdating(false);
      setCurrentlyUpdating("");
    }
  };

  const handleUpdateSelected = async () => {
    if (selectedUpdates.size === 0) return;

    try {
      setIsUpdating(true);
      const selectedUpdateList = Array.from(selectedUpdates).map(
        (i) => updates[i],
      );
      toast.loading(`Updating ${selectedUpdateList.length} packages...`);

      let successful = 0;
      let failed = 0;

      for (const update of selectedUpdateList) {
        const packageName = getUpdateName(update);
        try {
          setCurrentlyUpdating(packageName);
          const result = await invoke<UpdateResult>("apply_update", { update });

          if (result.status === "Completed") {
            successful++;
            const versions = getUpdateVersion(update);
            setUpdateHistory((prev) => [
              {
                package_id: getUpdateId(update),
                package_name: packageName,
                old_version: versions.current,
                new_version: versions.new,
                timestamp: new Date().toISOString(),
                status: "Success",
                manager: getUpdateManager(update),
              },
              ...prev,
            ]);
          } else {
            failed++;
          }
        } catch {
          failed++;
        }
      }

      toast.dismiss();
      if (failed > 0) {
        toast.warning(
          `Batch update completed: ${successful} successful, ${failed} failed`,
        );
      } else {
        toast.success(
          `Batch update completed successfully (${successful} packages)`,
        );
      }

      setUpdates((prev) => prev.filter((_, i) => !selectedUpdates.has(i)));
      setSelectedUpdates(new Set());
    } catch (error) {
      toast.dismiss();
      toast.error(`Batch update error: ${error}`);
    } finally {
      setIsUpdating(false);
      setCurrentlyUpdating("");
    }
  };

  const toggleUpdateSelection = (index: number) => {
    setSelectedUpdates((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(index)) {
        newSet.delete(index);
      } else {
        newSet.add(index);
      }
      return newSet;
    });
  };

  const toggleIgnorePackage = (id: string) => {
    setIgnoredPackages((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(id)) {
        newSet.delete(id);
        toast.info(`Removed package from ignored list`);
      } else {
        newSet.add(id);
        toast.success(`Added package to ignored list`);
      }
      localStorage.setItem(
        "ignoredPackages",
        JSON.stringify(Array.from(newSet)),
      );
      return newSet;
    });
  };

  const filteredUpdates = updates
    .filter((update) => {
      const name = getUpdateName(update).toLowerCase();
      const id = getUpdateId(update).toLowerCase();
      const query = searchQuery.toLowerCase();
      const matchesSearch = name.includes(query) || id.includes(query);
      const notIgnored = !ignoredPackages.has(id);

      if (categoryFilter === "All") return matchesSearch && notIgnored;
      return (
        matchesSearch &&
        notIgnored &&
        getUpdateCategory(update) === categoryFilter
      );
    })
    .sort((a, b) => {
      if (sortBy === "name")
        return getUpdateName(a).localeCompare(getUpdateName(b));
      if (sortBy === "priority") {
        const priorityA = "Package" in a ? a.Package.priority : "Normal";
        const priorityB = "Package" in b ? b.Package.priority : "Normal";
        const priorityOrder = { Critical: 3, Important: 2, Normal: 1 };
        return (
          priorityOrder[priorityB as keyof typeof priorityOrder] -
          priorityOrder[priorityA as keyof typeof priorityOrder]
        );
      }
      return 0;
    });

  const selectAll = () => {
    setSelectedUpdates(
      new Set(filteredUpdates.map((update) => updates.indexOf(update))),
    );
  };

  const deselectAll = () => {
    setSelectedUpdates(new Set());
  };

  const handleRollback = async (entry: UpdateHistoryEntry) => {
    if (
      !confirm(
        `Are you sure you want to rollback ${entry.package_name} to ${entry.old_version}?`,
      )
    )
      return;
    try {
      toast.loading(`Rolling back ${entry.package_name}...`);
      const pkg = {
        id: entry.package_id,
        name: entry.package_name,
        version: entry.new_version,
        manager: entry.manager,
      };
      const result = await invoke<UpdateResult>("rollback_update", {
        package: pkg,
        targetVersion: entry.old_version,
      });
      toast.dismiss();
      if (result.status === "Completed") {
        toast.success(
          `${entry.package_name} rolled back to ${entry.old_version} successfully.`,
        );
        setUpdateHistory((prev) => [
          {
            package_id: entry.package_id,
            package_name: entry.package_name,
            old_version: entry.new_version,
            new_version: entry.old_version,
            timestamp: new Date().toISOString(),
            status: "Rollback",
            manager: entry.manager,
          },
          ...prev,
        ]);
      } else {
        toast.error(`Rollback failed: ${result.error}`);
      }
    } catch (e) {
      toast.dismiss();
      toast.error(`Rollback error: ${e}`);
    }
  };

  return {
    scanResult,
    updates,
    isCheckingUpdates,
    isUpdating,
    selectedUpdates,
    searchQuery,
    setSearchQuery,
    hasCheckedUpdates,
    currentlyUpdating,
    updateHistory,
    ignoredPackages,
    categoryFilter,
    setCategoryFilter,
    sortBy,
    setSortBy,
    filteredUpdates,

    handleCheckUpdates,
    handleUpdateSingle,
    handleUpdateSelected,
    toggleUpdateSelection,
    toggleIgnorePackage,
    selectAll,
    deselectAll,
    handleRollback,

    getUpdateName,
    getUpdateId,
    getUpdateVersion,
    getUpdateCategory,
  };
}
