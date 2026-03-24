import { invoke } from "@tauri-apps/api/core";
import type {
    BulkContainerAction,
    BulkContainerActionResult,
    ContainerDetail,
    ContainerListQuery,
    ContainerListResponse,
    ContainerStatsSnapshot,
    ContainerSummary,
    ContainersOverviewSummary,
} from "@/types/docker";

// All Tauri IPC calls go through here.
// Never use raw invoke() in components.
export const docker = {
    listContainersResponse: (query?: ContainerListQuery): Promise<ContainerListResponse> =>
        invoke("list_containers", query ? { query } : {}),

    listContainers: async (): Promise<ContainerSummary[]> =>
        (await docker.listContainersResponse()).items,

    getContainersOverview: (): Promise<ContainersOverviewSummary> =>
        invoke("get_containers_overview"),

    getContainerDetail: (id: string): Promise<ContainerDetail> =>
        invoke("get_container_detail", { id }),

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

    applyContainerAction: (
        ids: string[],
        action: BulkContainerAction,
    ): Promise<BulkContainerActionResult[]> =>
        invoke("apply_container_action", { ids, action }),

    inspectContainer: (id: string): Promise<any> =>
        invoke("inspect_container", { id }),

    getContainerStats: (id: string): Promise<ContainerStatsSnapshot> =>
        invoke("get_container_stats", { id }),
};
