import { History as HistoryIcon, ArrowRight, RefreshCcw } from "lucide-react";
import { Badge } from "../ui/badge";
import { Button } from "../ui/button";
import { Card } from "../ui/card";
import { motion, AnimatePresence } from "framer-motion";

interface HistoryProps {
  manager: any;
}

export function History({ manager }: HistoryProps) {
  const { updateHistory, handleRollback } = manager;

  const statusClass = (status: string) => {
    switch (status.toLowerCase()) {
      case "success":
        return "bg-emerald-950 text-emerald-400 border-emerald-900";
      case "rollback":
        return "bg-amber-950 text-amber-400 border-amber-900";
      default:
        return "bg-red-950 text-red-400 border-red-900";
    }
  };

  return (
    <div className="pb-24 space-y-8 duration-500 animate-in fade-in slide-in-from-bottom-4">
      <div>
        <h2 className="text-xl font-bold tracking-tight text-zinc-100">
          Update History
        </h2>
        <p className="mt-1 text-sm text-zinc-500">
          Review past updates and perform rollbacks if needed.
        </p>
      </div>

      {updateHistory.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-20 text-center border border-dashed border-zinc-800 rounded-xl bg-zinc-900/30">
          <div className="flex items-center justify-center mb-4 border w-14 h-14 rounded-2xl bg-zinc-800 border-zinc-700">
            <HistoryIcon className="w-5 h-5 text-zinc-500" />
          </div>
          <h3 className="mb-2 text-base font-medium text-zinc-100">
            No Update History
          </h3>
          <p className="max-w-xs text-sm text-zinc-500">
            You haven't updated any packages yet.
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          <AnimatePresence>
            {updateHistory.map((entry: any, index: number) => (
              <motion.div
                key={entry.package_id + entry.timestamp}
                initial={{ opacity: 0, y: 5 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: index * 0.04 }}
              >
                <Card className="p-5 transition-colors sm:p-6 hover:bg-zinc-800/20 group">
                  <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    {/* Left: Package info */}
                    <div className="flex-1 min-w-0">
                      <div className="flex flex-wrap items-center gap-2 mb-1.5">
                        <h3 className="text-sm font-semibold text-zinc-100">
                          {entry.package_name}
                        </h3>
                        <Badge
                          variant="outline"
                          className={`text-xs px-2 py-0.5 flex-shrink-0 border ${statusClass(entry.status)}`}
                        >
                          {entry.status}
                        </Badge>
                        <Badge
                          variant="outline"
                          className="text-xs px-2 py-0.5 bg-zinc-800 border-zinc-700 text-zinc-400 flex-shrink-0"
                        >
                          {entry.manager}
                        </Badge>
                      </div>

                      <p className="mb-3 text-xs truncate text-zinc-600">
                        {entry.package_id}
                      </p>

                      <div className="flex items-center gap-2">
                        <span className="font-mono text-xs text-zinc-500">
                          {entry.old_version}
                        </span>
                        <ArrowRight className="flex-shrink-0 w-3 h-3 text-zinc-700" />
                        <span className="font-mono text-xs font-semibold text-zinc-100">
                          {entry.new_version}
                        </span>
                      </div>

                      {entry.error_message && (
                        <div className="p-3 mt-3 text-xs font-medium text-red-400 border border-red-900 rounded-lg bg-red-950">
                          {entry.error_message}
                        </div>
                      )}
                    </div>

                    {/* Right: Timestamp + Rollback */}
                    <div className="flex flex-row items-center justify-between gap-3 sm:flex-col sm:items-end sm:justify-start">
                      <span className="text-xs text-zinc-600 whitespace-nowrap">
                        {new Date(entry.timestamp).toLocaleString(undefined, {
                          dateStyle: "medium",
                          timeStyle: "short",
                        })}
                      </span>

                      {entry.status === "Success" && (
                        <Button
                          variant="outline"
                          size="sm"
                          className="h-8 text-xs transition-opacity opacity-100 sm:opacity-0 sm:group-hover:opacity-100 border-zinc-700 text-zinc-400 hover:text-amber-400 hover:border-amber-800 hover:bg-amber-950"
                          onClick={() => handleRollback(entry)}
                        >
                          <RefreshCcw className="w-3 h-3 mr-1.5" />
                          Rollback
                        </Button>
                      )}
                    </div>
                  </div>
                </Card>
              </motion.div>
            ))}
          </AnimatePresence>
        </div>
      )}
    </div>
  );
}
