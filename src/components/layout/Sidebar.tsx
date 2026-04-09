import {
  Package,
  DownloadCloud,
  History,
  Settings,
  Layers,
} from "lucide-react";

interface SidebarProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
  updateCount: number;
}

export function Sidebar({
  activeTab,
  setActiveTab,
  updateCount,
}: SidebarProps) {
  const navItems = [
    { id: "overview", label: "Overview", icon: Package },
    {
      id: "updates",
      label: "Updates",
      icon: DownloadCloud,
      badge: updateCount,
    },
    { id: "history", label: "History", icon: History },
    { id: "settings", label: "Settings", icon: Settings },
  ];

  return (
    <aside className="flex flex-col flex-shrink-0 w-64 h-full border-r border-slate-800 bg-slate-900">
      <div className="flex items-center flex-shrink-0 px-6 border-b h-16 border-slate-800">
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center p-2 rounded-lg bg-blue-600 shadow-sm shadow-blue-600/20">
            <Layers className="w-5 h-5 text-white" />
          </div>
          <h1 className="font-semibold tracking-tight text-base text-slate-100">
            Package Checker
          </h1>
        </div>
      </div>

      <nav className="flex-1 px-4 py-6 space-y-1.5 overflow-y-auto">
        <div className="px-2 mb-4 text-xs font-semibold tracking-wider uppercase text-slate-500">
          Menu
        </div>
        {navItems.map((item) => {
          const isActive = activeTab === item.id;
          const Icon = item.icon;
          return (
            <button
              key={item.id}
              onClick={() => setActiveTab(item.id)}
              className={`w-full flex items-center justify-between px-3 py-2.5 rounded-lg text-sm transition-all group ${
                isActive
                  ? "bg-blue-600/10 text-blue-500 font-medium shadow-sm border border-blue-500/10"
                  : "text-slate-400 hover:bg-slate-800/50 hover:text-slate-200 border border-transparent"
              }`}
            >
              <div className="flex items-center gap-3">
                <Icon
                  className={`w-5 h-5 ${
                    isActive
                      ? "text-blue-500"
                      : "text-slate-400 group-hover:text-slate-300"
                  }`}
                />
                {item.label}
              </div>
              {item.badge !== undefined && item.badge > 0 && (
                <span className={`px-2 py-0.5 text-xs font-medium rounded-full ${
                  isActive 
                    ? "bg-blue-500/20 text-blue-500" 
                    : "bg-slate-800 text-slate-300 group-hover:bg-slate-700"
                }`}>
                  {item.badge}
                </span>
              )}
            </button>
          );
        })}
      </nav>

      <div className="p-4 border-t border-slate-800">
        <div className="text-xs text-center text-slate-500 font-medium">
          v0.1.0
        </div>
      </div>
    </aside>
  );
}
