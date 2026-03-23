import { invoke } from "@tauri-apps/api/core";
import type { ContainerSummary } from "@/types/docker";

// All Tauri IPC calls go through here.
// Never use raw invoke() in components.
export const docker = {
    listContainers: (): Promise<ContainerSummary[]> =>
        invoke("list_containers"),

    startContainer: (id: string): Promise<void> =>
        invoke("start_container", { id }),

    stopContainer: (id: string): Promise<void> =>
        invoke("stop_container", { id }),

    restartContainer: (id: string): Promise<void> =>
        invoke("restart_container", { id }),

    pauseContainer: (id: string): Promise<void> =>
        invoke("pause_container", { id }),

    unpauseContainer: (id: string): Promise<void> =>
        invoke("unpause_container", { id }),

    removeContainer: (id: string, force = true, removeVolumes = false): Promise<void> =>
        invoke("remove_container", { id, force, removeVolumes }),

    inspectContainer: (id: string): Promise<any> =>
        invoke("inspect_container", { id }),

    getContainerStats: (id: string): Promise<any> =>
        invoke("get_container_stats", { id }),
};