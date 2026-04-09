import {
  CheckCircle2,
  AlertTriangle,
  Package,
  RefreshCw,
  DownloadCloud,
} from "lucide-react";
import { Button } from "../ui/button";
import { Card } from "../ui/card";
import { motion, AnimatePresence } from "framer-motion";

interface OverviewProps {
  manager: any;
  goToUpdates: () => void;
}

export function Overview({ manager, goToUpdates }: OverviewProps) {
  const {
    scanResult,
    updates,
    isCheckingUpdates,
    handleCheckUpdates,
    hasCheckedUpdates,
  } = manager;

  return (
    <div className="space-y-8 duration-500 animate-in fade-in slide-in-from-bottom-4">
      <div>
        <h2 className="text-xl font-bold tracking-tight text-zinc-100">
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
                <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-800 border-zinc-700">
                  <RefreshCw className="w-5 h-5 text-zinc-400 animate-spin" />
                </div>
                <h3 className="mb-2 text-base font-semibold text-zinc-100">
                  Scanning System...
                </h3>
                <p className="max-w-sm text-sm text-zinc-500">
                  Looking for installed packages and checking for updates.
                </p>
                <div className="w-48 h-0.5 mt-8 overflow-hidden rounded-full bg-zinc-800">
                  <motion.div
                    className="h-full rounded-full bg-zinc-400"
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
                  <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-800 border-zinc-700">
                    <AlertTriangle className="w-5 h-5 text-amber-400" />
                  </div>
                  <h3 className="mb-2 text-base font-semibold text-zinc-100">
                    Updates Available
                  </h3>
                  <p className="mb-8 text-sm text-zinc-500">
                    <span className="font-semibold text-zinc-100">
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
                  <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-800 border-zinc-700">
                    <CheckCircle2 className="w-5 h-5 text-emerald-400" />
                  </div>
                  <h3 className="mb-2 text-base font-semibold text-zinc-100">
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
                <div className="flex items-center justify-center mb-6 border w-14 h-14 rounded-2xl bg-zinc-800 border-zinc-700">
                  <Package className="w-5 h-5 text-zinc-400" />
                </div>
                <h3 className="mb-2 text-base font-semibold text-zinc-100">
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
            <div className="flex items-center justify-center flex-shrink-0 w-12 h-12 border rounded-xl bg-zinc-800 border-zinc-700">
              <DownloadCloud className="w-5 h-5 text-zinc-300" />
            </div>
            <div className="min-w-0">
              <p className="mb-1 text-xs font-medium tracking-wider uppercase text-zinc-500">
                Available Updates
              </p>
              <h4 className="text-3xl font-bold text-zinc-100">
                {scanResult.updatable_count ?? 0}
              </h4>
            </div>
          </Card>
          <Card className="flex items-center gap-5 p-5 sm:p-6">
            <div className="flex items-center justify-center flex-shrink-0 w-12 h-12 border rounded-xl bg-zinc-800 border-zinc-700">
              <Package className="w-5 h-5 text-zinc-300" />
            </div>
            <div className="min-w-0">
              <p className="mb-1 text-xs font-medium tracking-wider uppercase text-zinc-500">
                Total Packages
              </p>
              <h4 className="text-3xl font-bold text-zinc-100">
                {scanResult.scanned_count ?? 0}
              </h4>
            </div>
          </Card>
        </div>
      )}
    </div>
  );
}
