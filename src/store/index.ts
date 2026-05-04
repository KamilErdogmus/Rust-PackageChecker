import { create } from "zustand";

interface AppState {
  platform: string | null;
  setPlatform: (platform: string) => void;
  isScanning: boolean;
  setIsScanning: (isScanning: boolean) => void;
  theme: "light" | "dark";
  setTheme: (theme: "light" | "dark") => void;
}

const savedTheme =
  (localStorage.getItem("theme") as "light" | "dark") || "dark";

if (savedTheme === "dark") {
  document.documentElement.classList.add("dark");
} else {
  document.documentElement.classList.remove("dark");
}

export const useAppStore = create<AppState>((set) => ({
  platform: null,
  setPlatform: (platform) => set({ platform }),
  isScanning: false,
  setIsScanning: (isScanning) => set({ isScanning }),
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
