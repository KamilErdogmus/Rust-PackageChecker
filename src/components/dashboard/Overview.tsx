import {
  CheckCircle2,
  AlertTriangle,
  Package,
  RefreshCw,
  DownloadCloud,
  Box,
} from "lucide-react";
import { Button } from "../ui/button";
import { Card } from "../ui/card";
import { motion, AnimatePresence } from "framer-motion";

interface OverviewProps {
  manager: any;
  goToUpdates: () => void;
}

const PM_META: Record<string, { label: string; color: string; bg: string }> = {
  Winget: {
    label: "Winget",
    color: "text-blue-600 dark:text-blue-400",
    bg: "bg-blue-50 border-blue-200 dark:bg-blue-500/10 dark:border-blue-500/20",
  },
  Chocolatey: {
    label: "Chocolatey",
    color: "text-orange-600 dark:text-orange-400",
    bg: "bg-orange-50 border-orange-200 dark:bg-orange-500/10 dark:border-orange-500/20",
  },
  Scoop: {
    label: "Scoop",
    color: "text-cyan-600 dark:text-cyan-400",
    bg: "bg-cyan-50 border-cyan-200 dark:bg-cyan-500/10 dark:border-cyan-500/20",
  },
  Npm: {
    label: "npm",
    color: "text-red-600 dark:text-red-400",
    bg: "bg-red-50 border-red-200 dark:bg-red-500/10 dark:border-red-500/20",
  },
  Cargo: {
    label: "Cargo",
    color: "text-amber-600 dark:text-amber-400",
    bg: "bg-amber-50 border-amber-200 dark:bg-amber-500/10 dark:border-amber-500/20",
  },
  Gem: {
    label: "gem",
    color: "text-rose-600 dark:text-rose-400",
    bg: "bg-rose-50 border-rose-200 dark:bg-rose-500/10 dark:border-rose-500/20",
  },
  Pip: {
    label: "pip",
    color: "text-yellow-600 dark:text-yellow-400",
    bg: "bg-yellow-50 border-yellow-200 dark:bg-yellow-500/10 dark:border-yellow-500/20",
  },
  Homebrew: {
    label: "Homebrew",
    color: "text-amber-600 dark:text-amber-300",
    bg: "bg-amber-50 border-amber-200 dark:bg-amber-500/10 dark:border-amber-500/20",
  },
  MacAppStore: {
    label: "App Store",
    color: "text-blue-600 dark:text-blue-300",
    bg: "bg-blue-50 border-blue-200 dark:bg-blue-500/10 dark:border-blue-500/20",
  },
  Apt: {
    label: "apt",
    color: "text-green-600 dark:text-green-400",
    bg: "bg-green-50 border-green-200 dark:bg-green-500/10 dark:border-green-500/20",
  },
  Dnf: {
    label: "dnf",
    color: "text-blue-600 dark:text-blue-400",
    bg: "bg-blue-50 border-blue-200 dark:bg-blue-500/10 dark:border-blue-500/20",
  },
  Pacman: {
    label: "pacman",
    color: "text-teal-600 dark:text-teal-400",
    bg: "bg-teal-50 border-teal-200 dark:bg-teal-500/10 dark:border-teal-500/20",
  },
  Flatpak: {
    label: "Flatpak",
    color: "text-indigo-600 dark:text-indigo-400",
    bg: "bg-indigo-50 border-indigo-200 dark:bg-indigo-500/10 dark:border-indigo-500/20",
  },
  Snap: {
    label: "Snap",
    color: "text-orange-600 dark:text-orange-300",
    bg: "bg-orange-50 border-orange-200 dark:bg-orange-500/10 dark:border-orange-500/20",
  },
};

export function Overview({ manager, goToUpdates }: OverviewProps) {
  const {
    scanResult,
    updates,
    isCheckingUpdates,
    handleCheckUpdates,
    hasCheckedUpdates,
    installedManagers,
  } = manager;

  return (
    <div className="space-y-8 duration-500 animate-in fade-in slide-in-from-bottom-4">
      <div>
        <h2 className="text-xl font-bold tracking-tight text-zinc-800 dark:text-zinc-100">
          Overview
        </h2>
        <p className="mt-1 text-sm text-zinc-500">
          Monitor your system packages and updates.
        </p>
      </div>

      <Card className="p-8 sm:p-12">
        <div className="flex flex-col items-center justify-center py-4 text-center">
          <AnimatePresence mode="wait">
            {isCheckingUpdates ? (
              <motion.div
                key="checking"
                initial={{ opacity: 0, y: 5 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -5 }}
                className="flex flex-col items-center"
              >
                <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-100 border-zinc-200 dark:bg-zinc-800 dark:border-zinc-700">
                  <RefreshCw className="w-5 h-5 text-zinc-500 dark:text-zinc-400 animate-spin" />
                </div>
                <h3 className="mb-2 text-base font-semibold text-zinc-800 dark:text-zinc-100">
                  Scanning System...
                </h3>
                <p className="max-w-sm text-sm text-zinc-500">
                  Looking for installed packages and checking for updates.
                </p>
                <div className="w-48 h-0.5 mt-8 overflow-hidden rounded-full bg-zinc-200 dark:bg-zinc-800">
                  <motion.div
                    className="h-full rounded-full bg-zinc-500 dark:bg-zinc-400"
                    initial={{ width: "0%" }}
                    animate={{ width: "100%" }}
                    transition={{ duration: 2, repeat: Infinity }}
                  />
                </div>
              </motion.div>
            ) : hasCheckedUpdates ? (
              updates.length > 0 ? (
                <motion.div
                  key="updates"
                  initial={{ opacity: 0, y: 5 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -5 }}
                  className="flex flex-col items-center"
                >
                  <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-100 border-zinc-200 dark:bg-zinc-800 dark:border-zinc-700">
                    <AlertTriangle className="w-5 h-5 text-amber-400" />
                  </div>
                  <h3 className="mb-2 text-base font-semibold text-zinc-800 dark:text-zinc-100">
                    Updates Available
                  </h3>
                  <p className="mb-8 text-sm text-zinc-500">
                    <span className="font-semibold text-zinc-800 dark:text-zinc-100">
                      {updates.length}
                    </span>{" "}
                    {updates.length === 1 ? "package" : "packages"} waiting to
                    be updated.
                  </p>
                  <div className="flex flex-col w-full gap-3 sm:flex-row sm:w-auto">
                    <Button onClick={goToUpdates} size="lg">
                      View Updates
                    </Button>
                    <Button
                      variant="outline"
                      onClick={handleCheckUpdates}
                      size="lg"
                    >
                      Scan Again
                    </Button>
                  </div>
                </motion.div>
              ) : (
                <motion.div
                  key="updated"
                  initial={{ opacity: 0, y: 5 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -5 }}
                  className="flex flex-col items-center"
                >
                  <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-100 border-zinc-200 dark:bg-zinc-800 dark:border-zinc-700">
                    <CheckCircle2 className="w-5 h-5 text-emerald-400" />
                  </div>
                  <h3 className="mb-2 text-base font-semibold text-zinc-800 dark:text-zinc-100">
                    System is Up to Date
                  </h3>
                  <p className="mb-8 text-sm text-zinc-500">
                    All monitored packages are on their latest versions.
                  </p>
                  <Button
                    variant="outline"
                    onClick={handleCheckUpdates}
                    size="lg"
                  >
                    Check Again
                  </Button>
                </motion.div>
              )
            ) : (
              <motion.div
                key="idle"
                initial={{ opacity: 0, y: 5 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -5 }}
                className="flex flex-col items-center"
              >
                <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-100 border-zinc-200 dark:bg-zinc-800 dark:border-zinc-700">
                  <Package className="w-5 h-5 text-zinc-500 dark:text-zinc-400" />
                </div>
                <h3 className="mb-2 text-base font-semibold text-zinc-800 dark:text-zinc-100">
                  Ready to Scan
                </h3>
                <p className="mb-8 text-sm text-zinc-500">
                  Click below to check your system for available package
                  updates.
                </p>
                <Button onClick={handleCheckUpdates} size="lg">
                  <RefreshCw className="w-4 h-4 mr-2" />
                  Check for Updates
                </Button>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </Card>

      {scanResult && (
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <Card className="flex items-center gap-5 p-5 sm:p-6">
            <div className="flex items-center justify-center flex-shrink-0 w-12 h-12 border rounded-xl bg-zinc-100 border-zinc-200 dark:bg-zinc-800 dark:border-zinc-700">
              <DownloadCloud className="w-5 h-5 text-zinc-600 dark:text-zinc-300" />
            </div>
            <div className="min-w-0">
              <p className="mb-1 text-xs font-medium tracking-wider uppercase text-zinc-500">
                Available Updates
              </p>
              <h4 className="text-3xl font-bold text-zinc-800 dark:text-zinc-100">
                {scanResult.updatable_count ?? 0}
              </h4>
            </div>
          </Card>
          <Card className="flex items-center gap-5 p-5 sm:p-6">
            <div className="flex items-center justify-center flex-shrink-0 w-12 h-12 border rounded-xl bg-zinc-100 border-zinc-200 dark:bg-zinc-800 dark:border-zinc-700">
              <Package className="w-5 h-5 text-zinc-600 dark:text-zinc-300" />
            </div>
            <div className="min-w-0">
              <p className="mb-1 text-xs font-medium tracking-wider uppercase text-zinc-500">
                Total Packages
              </p>
              <h4 className="text-3xl font-bold text-zinc-800 dark:text-zinc-100">
                {scanResult.scanned_count ?? 0}
              </h4>
            </div>
          </Card>
        </div>
      )}

      <div>
        <div className="flex items-center gap-2 mb-3">
          <Box className="w-4 h-4 text-zinc-500" />
          <h3 className="text-sm font-semibold tracking-wider uppercase text-zinc-500">
            Detected Package Managers
          </h3>
        </div>
        {installedManagers.length === 0 ? (
          <Card className="p-4">
            <p className="text-sm text-zinc-500">
              No package managers detected yet. Run a scan to check.
            </p>
          </Card>
        ) : (
          <div className="flex flex-wrap gap-2">
            {installedManagers.map((pm: string) => {
              const meta = PM_META[pm] ?? {
                label: pm,
                color: "text-zinc-600 dark:text-zinc-300",
                bg: "bg-zinc-100 border-zinc-200 dark:bg-zinc-800 dark:border-zinc-700",
              };
              return (
                <span
                  key={pm}
                  className={`inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg border text-xs font-semibold ${meta.color} ${meta.bg}`}
                >
                  <span className="w-1.5 h-1.5 rounded-full bg-current opacity-80" />
                  {meta.label}
                </span>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
