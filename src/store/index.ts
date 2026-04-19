// Zustand store for global state management
import { create } from "zustand";

interface AppState {
  // Platform info
  platform: string | null;
  setPlatform: (platform: string) => void;

  // Loading states
  isScanning: boolean;
  setIsScanning: (isScanning: boolean) => void;

  // Theme
  theme: "light" | "dark";
  setTheme: (theme: "light" | "dark") => void;
}

const savedTheme =
  (localStorage.getItem("theme") as "light" | "dark") || "dark";

// Apply theme to <html> immediately (before React renders)
if (savedTheme === "dark") {
  document.documentElement.classList.add("dark");
} else {
  document.documentElement.classList.remove("dark");
}

export const useAppStore = create<AppState>((set) => ({
  // Platform
  platform: null,
  setPlatform: (platform) => set({ platform }),

  // Loading
  isScanning: false,
  setIsScanning: (isScanning) => set({ isScanning }),

  // Theme
  theme: savedTheme,
  setTheme: (theme) => {
    localStorage.setItem("theme", theme);
    if (theme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
    set({ theme });
  },
}));
