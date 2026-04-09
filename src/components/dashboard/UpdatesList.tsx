import {
  Search,
  ArrowRight,
  Download,
  SlidersHorizontal,
  CheckSquare,
  RefreshCw,
  PackageX,
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "../ui/button";
import { Card } from "../ui/card";

interface UpdatesListProps {
  manager: any;
}

export function UpdatesList({ manager }: UpdatesListProps) {
  const {
    updates,
    filteredUpdates,
    isUpdating,
    currentlyUpdating,
    selectedUpdates,
    searchQuery,
    setSearchQuery,
    categoryFilter,
    setCategoryFilter,
    sortBy,
    setSortBy,
    handleUpdateSingle,
    handleUpdateSelected,
    toggleUpdateSelection,
    selectAll,
    deselectAll,
    getUpdateName,
    getUpdateId,
    getUpdateVersion,
  } = manager;

  if (updates.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-[60vh] text-center">
        <div className="flex items-center justify-center mb-4 border w-14 h-14 rounded-2xl bg-zinc-800 border-zinc-700">
          <PackageX className="w-5 h-5 text-zinc-500" />
        </div>
        <h2 className="mb-2 text-base font-semibold text-zinc-100">
          No Updates Found
        </h2>
        <p className="max-w-sm text-sm text-zinc-500">
          Your system is fully up to date or you haven't scanned yet.
        </p>
      </div>
    );
  }

  return (
    <div className="pb-24 space-y-8 duration-500 animate-in fade-in slide-in-from-bottom-4">
      <div>
        <h2 className="text-xl font-bold tracking-tight text-zinc-100">
          Available Updates
        </h2>
        <p className="mt-1 text-sm text-zinc-500">
          Select packages to update individually or all at once.
        </p>
      </div>

      {/* Filters */}
      <Card className="flex flex-col gap-3 p-5 sm:flex-row sm:items-center">
        <div className="relative flex-1">
          <Search className="absolute w-4 h-4 -translate-y-1/2 left-3 top-1/2 text-zinc-600" />
          <input
            placeholder="Search packages..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pr-4 text-sm border rounded-lg h-9 pl-9 bg-zinc-950 border-zinc-800 focus:border-zinc-600 focus:ring-1 focus:ring-zinc-600 focus:outline-none text-zinc-100 placeholder:text-zinc-600"
          />
        </div>
        <div className="flex gap-3">
          <div className="relative flex-1 min-w-[140px]">
            <select
              value={categoryFilter}
              onChange={(e) => setCategoryFilter(e.target.value)}
              className="w-full py-0 pl-3 pr-8 text-sm border rounded-lg appearance-none h-9 border-zinc-800 bg-zinc-950 focus:outline-none focus:border-zinc-600 focus:ring-1 focus:ring-zinc-600 text-zinc-300"
            >
              <option value="All">All Categories</option>
              <option value="Development">Development</option>
              <option value="System">System</option>
              <option value="Media">Media</option>
              <option value="Productivity">Productivity</option>
              <option value="Gaming">Gaming</option>
              <option value="Other">Other</option>
            </select>
            <SlidersHorizontal className="absolute w-3.5 h-3.5 -translate-y-1/2 pointer-events-none right-2.5 top-1/2 text-zinc-600" />
          </div>
          <div className="relative flex-1 min-w-[130px]">
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value)}
              className="w-full py-0 pl-3 pr-8 text-sm border rounded-lg appearance-none h-9 border-zinc-800 bg-zinc-950 focus:outline-none focus:border-zinc-600 focus:ring-1 focus:ring-zinc-600 text-zinc-300"
            >
              <option value="name">Sort by Name</option>
              <option value="priority">Sort by Priority</option>
            </select>
            <SlidersHorizontal className="absolute w-3.5 h-3.5 -translate-y-1/2 pointer-events-none right-2.5 top-1/2 text-zinc-600" />
          </div>
        </div>
      </Card>

      {/* Bulk actions */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="flex items-center gap-2">
          <Button
            variant="secondary"
            size="sm"
            onClick={selectAll}
            disabled={isUpdating}
            className="min-w-[110px]"
          >
            <CheckSquare className="w-4 h-4 mr-2" />
            Select All
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={deselectAll}
            disabled={isUpdating}
            className="min-w-[80px]"
          >
            Clear
          </Button>
        </div>
        <Button
          onClick={handleUpdateSelected}
          disabled={selectedUpdates.size === 0 || isUpdating}
          className="w-full sm:w-auto"
        >
          <Download className="w-4 h-4 mr-2" />
          Update Selected ({selectedUpdates.size})
        </Button>
      </div>

      {/* Table */}
      <div className="overflow-x-auto border border-zinc-800 rounded-xl bg-zinc-900">
        <div className="min-w-[480px]">
          {/* Header */}
          <div className="grid grid-cols-[3rem_1fr_auto] sm:grid-cols-[3rem_1fr_16rem_7rem] items-center gap-4 px-4 py-3 border-b border-zinc-800 bg-zinc-950/50">
            <div />
            <div className="text-xs font-semibold tracking-wider uppercase text-zinc-600">
              Package
            </div>
            <div className="hidden text-xs font-semibold tracking-wider text-center uppercase text-zinc-600 sm:block">
              Version
            </div>
            <div className="pr-2 text-xs font-semibold tracking-wider text-right uppercase text-zinc-600">
              Action
            </div>
          </div>

          {/* Rows */}
          <div className="divide-y divide-zinc-800/60">
            <AnimatePresence>
              {filteredUpdates.map((update: any) => {
                const index = updates.indexOf(update);
                const versions = getUpdateVersion(update);
                const name = getUpdateName(update);
                const id = getUpdateId(update);
                const isSelected = selectedUpdates.has(index);

                return (
                  <motion.div
                    key={id}
                    initial={{ opacity: 0, y: 5 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, height: 0 }}
                    className={`group grid grid-cols-[3rem_1fr_auto] sm:grid-cols-[3rem_1fr_16rem_7rem] items-center gap-4 px-4 py-4 transition-colors hover:bg-zinc-800/30 ${
                      isSelected ? "bg-zinc-800/20" : ""
                    }`}
                  >
                    <div className="flex justify-center">
                      <input
                        type="checkbox"
                        checked={isSelected}
                        onChange={() => toggleUpdateSelection(index)}
                        disabled={isUpdating}
                        className="w-4 h-4 rounded cursor-pointer border-zinc-700 bg-zinc-950 text-zinc-100 focus:ring-zinc-500 focus:ring-offset-zinc-900 accent-white"
                      />
                    </div>

                    <div className="flex flex-col min-w-0">
                      <span className="text-sm font-medium truncate text-zinc-100">
                        {name}
                      </span>
                      <span
                        className="text-xs truncate text-zinc-600 mt-0.5"
                        title={id}
                      >
                        {id}
                      </span>
                    </div>

                    <div className="items-center justify-center hidden gap-2 sm:flex">
                      <span className="font-mono text-xs text-zinc-500">
                        {versions.current}
                      </span>
                      <ArrowRight className="w-3.5 h-3.5 text-zinc-700 flex-shrink-0" />
                      <span className="font-mono text-xs font-semibold text-zinc-100">
                        {versions.new}
                      </span>
                    </div>

                    <div className="pr-2 text-right">
                      <button
                        onClick={() => handleUpdateSingle(update, index)}
                        disabled={isUpdating}
                        className="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 text-zinc-100 rounded-lg font-medium text-xs w-full sm:w-auto transition-colors disabled:opacity-40 disabled:cursor-not-allowed border border-zinc-700"
                      >
                        Update
                      </button>
                    </div>
                  </motion.div>
                );
              })}
            </AnimatePresence>
          </div>
        </div>
      </div>

      {/* Updating toast */}
      {currentlyUpdating && (
        <div className="fixed z-50 duration-300 bottom-4 right-4 sm:bottom-8 sm:right-8 animate-in slide-in-from-bottom-8 fade-in">
          <div className="flex items-center gap-4 px-4 py-3 border shadow-xl border-zinc-700 bg-zinc-900 text-zinc-100 rounded-xl">
            <RefreshCw className="flex-shrink-0 w-4 h-4 text-zinc-400 animate-spin" />
            <div>
              <p className="text-xs font-medium text-zinc-500 mb-0.5">
                Updating
              </p>
              <p className="text-sm font-medium truncate max-w-[200px] sm:max-w-[240px] text-zinc-100">
                {currentlyUpdating}
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
