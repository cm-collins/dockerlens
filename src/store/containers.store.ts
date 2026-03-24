import { create } from "zustand";

type ContainersStore = {
  selectedId: string | null;
  setSelectedId: (id: string | null) => void;
};

export const useContainersStore = create<ContainersStore>((set) => ({
  selectedId: null,
  setSelectedId: (selectedId) => set({ selectedId }),
}));
