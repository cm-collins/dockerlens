import { create } from "zustand";

type Screen = "containers" | "images" | "volumes" | "networks" | "suggestions" | "settings";

interface AppStore {
    screen: Screen;
    daemonRunning: boolean;
    setScreen: (screen: Screen) => void;
    setDaemonRunning: (running: boolean) => void;
}

export const useAppStore = create<AppStore>((set) => ({
    screen: "containers",
    daemonRunning: true,
    setScreen: (screen) => set({ screen }),
    setDaemonRunning: (daemonRunning) => set({ daemonRunning }),
}));
