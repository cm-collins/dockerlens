import { invoke } from "@tauri-apps/api/core";
import type { ContainerSummary } from "@/types/docker";

// Typed wrappers around Tauri's invoke().
// All Docker API calls go through here — never use raw invoke() in components.
export const docker = {
    /** Returns all containers — running, stopped and paused. */
    listContainers: (): Promise<ContainerSummary[]> =>
        invoke("list_containers"),
};