import { create } from "zustand";
import type { ContainerSummary } from "@/types/docker";

interface ContainersStore {
    containers: ContainerSummary[];
    selectedId: string | null;
    activeTab: ContainerTab;
    setContainers: (containers: ContainerSummary[]) => void;
    setSelectedId: (id: string | null) => void;
    setActiveTab: (tab: ContainerTab) => void;
}

export type ContainerTab = "overview" | "logs" | "terminal" | "stats" | "inspect";

export const useContainersStore = create<ContainersStore>((set) => ({
    containers: [],
    selectedId: null,
    activeTab: "overview",
    setContainers: (containers) => set({ containers }),
    setSelectedId: (selectedId) => set({ selectedId, activeTab: "overview" }),
    setActiveTab: (activeTab) => set({ activeTab }),
}));