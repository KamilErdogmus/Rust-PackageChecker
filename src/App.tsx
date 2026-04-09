import { useState } from "react";
import { Toaster } from "sonner";
import { usePackageManager } from "./hooks/usePackageManager";
import { Overview } from "./components/dashboard/Overview";
import { UpdatesList } from "./components/dashboard/UpdatesList";
import { History as HistoryView } from "./components/dashboard/History";
import { Settings as SettingsView } from "./components/dashboard/Settings";
import {
  Package,
  DownloadCloud,
  History,
  Settings,
  Layers,
} from "lucide-react";

function App() {
  const manager = usePackageManager();
  const [activeTab, setActiveTab] = useState("overview");

  const renderContent = () => {
    switch (activeTab) {
      case "overview":
        return (
          <Overview
            manager={manager}
            goToUpdates={() => setActiveTab("updates")}
          />
        );
      case "updates":
        return <UpdatesList manager={manager} />;
      case "history":
        return <HistoryView manager={manager} />;
      case "settings":
        return <SettingsView manager={manager} />;
      default:
        return (
          <Overview
            manager={manager}
            goToUpdates={() => setActiveTab("updates")}
          />
        );
    }
  };

  const navItems = [
    { id: "overview", label: "Overview", icon: Package },
    {
      id: "updates",
      label: "Updates",
      icon: DownloadCloud,
      badge: manager.updates.length,
    },
    { id: "history", label: "History", icon: History },
    { id: "settings", label: "Settings", icon: Settings },
  ];

  return (
    <div className="flex flex-col w-screen h-screen overflow-hidden font-sans bg-zinc-950 text-zinc-200">
      {/* Top Navbar */}
      <header className="flex-shrink-0 border-b border-zinc-800 bg-zinc-900">
        <div className="px-6 sm:px-10 lg:px-16">
          <div className="flex items-center justify-between max-w-5xl mx-auto h-14">
            {/* Logo */}
            <div className="flex items-center gap-3">
              <div className="flex items-center justify-center w-8 h-8 border rounded-lg bg-zinc-800 border-zinc-700">
                <Layers className="w-4 h-4 text-zinc-300" />
              </div>
              <span className="hidden text-sm font-semibold tracking-tight text-zinc-100 sm:block">
                Package Checker
              </span>
            </div>

            {/* Nav Items */}
            <nav className="flex items-center h-full gap-1 sm:gap-2">
              {navItems.map((item) => {
                const isActive = activeTab === item.id;
                const Icon = item.icon;

                return (
                  <button
                    key={item.id}
                    onClick={() => setActiveTab(item.id)}
                    className={`group relative flex items-center h-full gap-2 px-3 text-sm font-medium transition-colors ${
                      isActive
                        ? "text-zinc-100"
                        : "text-zinc-500 hover:text-zinc-300"
                    }`}
                  >
                    <Icon className="w-4 h-4" />
                    <span className="hidden md:inline">{item.label}</span>
                    {item.badge !== undefined && item.badge > 0 && (
                      <span
                        className={`inline-flex items-center justify-center min-w-[18px] h-[18px] px-1 text-xs font-bold rounded-full ${
                          isActive
                            ? "bg-zinc-700 text-zinc-100"
                            : "bg-zinc-800 text-zinc-400"
                        }`}
                      >
                        {item.badge}
                      </span>
                    )}
                    {isActive && (
                      <span className="absolute bottom-0 left-0 right-0 h-px bg-zinc-100" />
                    )}
                  </button>
                );
              })}
            </nav>
          </div>
        </div>
      </header>

      <main className="flex-1 min-h-0 overflow-y-auto">
        <div className="px-6 py-8 sm:px-10 lg:px-16 lg:py-12">
          <div className="max-w-5xl mx-auto">{renderContent()}</div>
        </div>
      </main>

      <Toaster
        position="bottom-right"
        expand={true}
        richColors
        toastOptions={{
          className:
            "border border-zinc-800 bg-zinc-900 text-sm text-zinc-100 font-sans rounded-xl shadow-lg",
        }}
      />
    </div>
  );
}

export default App;
