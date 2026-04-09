import { Settings2, ShieldOff, Trash2, Moon, Sun } from "lucide-react";
import { Button } from "../ui/button";
import { Card } from "../ui/card";
import { useAppStore } from "../../store";
import { useEffect } from "react";

interface SettingsProps {
  manager: any;
}

export function Settings({ manager }: SettingsProps) {
  const { ignoredPackages, toggleIgnorePackage } = manager;
  const { theme, setTheme } = useAppStore();

  useEffect(() => {
    if (theme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }, [theme]);

  const ignoredList = Array.from(ignoredPackages);

  return (
    <div className="pb-24 space-y-8 duration-500 animate-in fade-in slide-in-from-bottom-4">
      <div>
        <h2 className="text-xl font-bold tracking-tight text-zinc-100">
          Settings
        </h2>
        <p className="mt-1 text-sm text-zinc-500">
          Manage preferences and ignored packages.
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        {/* Appearance */}
        <Card className="p-6">
          <div className="flex items-center gap-3 mb-5">
            <div className="flex items-center justify-center border rounded-lg w-9 h-9 bg-zinc-800 border-zinc-700">
              <Settings2 className="w-4 h-4 text-zinc-400" />
            </div>
            <div>
              <h3 className="text-sm font-semibold text-zinc-100">
                Appearance
              </h3>
              <p className="text-xs text-zinc-500">Choose your theme</p>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => setTheme("light")}
              className={`flex items-center gap-3 px-4 py-3 rounded-lg border text-sm font-medium transition-all ${
                theme === "light"
                  ? "bg-zinc-100 text-zinc-900 border-zinc-200"
                  : "bg-zinc-800/50 text-zinc-400 border-zinc-800 hover:bg-zinc-800 hover:text-zinc-200"
              }`}
            >
              <Sun className="flex-shrink-0 w-4 h-4" />
              Light
            </button>
            <button
              onClick={() => setTheme("dark")}
              className={`flex items-center gap-3 px-4 py-3 rounded-lg border text-sm font-medium transition-all ${
                theme === "dark"
                  ? "bg-zinc-100 text-zinc-900 border-zinc-200"
                  : "bg-zinc-800/50 text-zinc-400 border-zinc-800 hover:bg-zinc-800 hover:text-zinc-200"
              }`}
            >
              <Moon className="flex-shrink-0 w-4 h-4" />
              Dark
            </button>
          </div>
        </Card>

        {/* Ignored Packages */}
        <Card className="p-6">
          <div className="flex items-center gap-3 mb-5">
            <div className="flex items-center justify-center border rounded-lg w-9 h-9 bg-zinc-800 border-zinc-700">
              <ShieldOff className="w-4 h-4 text-zinc-400" />
            </div>
            <div>
              <h3 className="text-sm font-semibold text-zinc-100">
                Ignored Packages
              </h3>
              <p className="text-xs text-zinc-500">Excluded from scans</p>
            </div>
          </div>

          {ignoredList.length === 0 ? (
            <div className="py-8 text-center border border-dashed rounded-lg border-zinc-800">
              <p className="text-sm text-zinc-600">No packages ignored.</p>
            </div>
          ) : (
            <div className="space-y-2 max-h-[260px] overflow-y-auto">
              {ignoredList.map((id) => (
                <div
                  key={id as string}
                  className="flex items-center justify-between gap-2 px-3 py-2.5 border rounded-lg border-zinc-800 bg-zinc-800/30"
                >
                  <span className="text-sm truncate text-zinc-300">
                    {id as string}
                  </span>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="flex-shrink-0 w-8 h-8 text-zinc-600 hover:text-red-400 hover:bg-red-950"
                    onClick={() => toggleIgnorePackage(id as string)}
                  >
                    <Trash2 className="w-3.5 h-3.5" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </Card>
      </div>
    </div>
  );
}
