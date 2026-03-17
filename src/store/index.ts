// Zustand store for global state management
import { create } from 'zustand';

interface AppState {
  // Platform info
  platform: string | null;
  setPlatform: (platform: string) => void;
  
  // Loading states
  isScanning: boolean;
  setIsScanning: (isScanning: boolean) => void;
  
  // Theme
  theme: 'light' | 'dark';
  setTheme: (theme: 'light' | 'dark') => void;
}

export const useAppStore = create<AppState>((set) => ({
  // Platform
  platform: null,
  setPlatform: (platform) => set({ platform }),
  
  // Loading
  isScanning: false,
  setIsScanning: (isScanning) => set({ isScanning }),
  
  // Theme
  theme: 'light',
  setTheme: (theme) => set({ theme }),
}));
