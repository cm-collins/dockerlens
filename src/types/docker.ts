// These types mirror the Rust structs in src-tauri/src/docker/containers.rs exactly.
// Any change to the Rust structs must be reflected here.

export interface PortBinding {
    host_ip: string;
    host_port: string;
    container_port: string;
    protocol: "tcp" | "udp" | "sctp" | string;
    browser_url?: string | null;
}

export interface PlatformInfo {
    os?: string | null;
    architecture?: string | null;
    image_architecture_mismatch: boolean;
}

export interface ContainerActionCapabilities {
    can_start: boolean;
    can_stop: boolean;
    can_restart: boolean;
    can_pause: boolean;
    can_unpause: boolean;
    can_remove: boolean;
    can_open_port: boolean;
    can_inspect: boolean;
}

export interface ContainerStatsSnapshot {
    cpu_percent?: number | null;
    memory_usage_bytes?: number | null;
    memory_limit_bytes?: number | null;
    memory_percent?: number | null;
    network_rx_bytes?: number | null;
    network_tx_bytes?: number | null;
    collected_at_ms: number;
}

export interface ContainersOverviewSummary {
    total: number;
    running: number;
    paused: number;
    exited: number;
    total_cpu_percent: number;
    total_memory_usage_bytes: number;
    total_memory_limit_bytes: number;
    generated_at_ms: number;
}

export interface ContainerListQuery {
    all?: boolean;
    only_running?: boolean;
    search?: string | null;
    limit?: number | null;
}

export interface KeyValue {
    key: string;
    value: string;
}

export interface MountInfo {
    source: string;
    destination: string;
    mode?: string | null;
    rw?: boolean | null;
    mount_type?: string | null;
}

export interface NetworkAttachment {
    name: string;
    ip_address?: string | null;
    gateway?: string | null;
    mac_address?: string | null;
}

export interface ContainerSummary {
    /** Full container ID */
    id: string;
    /** First 12 characters of the container ID */
    short_id: string;
    /** Container name without the leading slash */
    name: string;
    image: string;
    /** Human-readable status string e.g. "Up 3 hours" */
    status: string;
    /** Lifecycle state: running | exited | paused | restarting */
    state: ContainerState;
    state_reason?: string | null;
    health?: string | null;
    ports: PortBinding[];
    platform: PlatformInfo;
    stats?: ContainerStatsSnapshot | null;
    actions: ContainerActionCapabilities;
    /** Unix timestamp of creation */
    created: number;
}

export interface ContainerListResponse {
    items: ContainerSummary[];
    overview: ContainersOverviewSummary;
    total_count: number;
    filtered_count: number;
    generated_at_ms: number;
}

export interface ContainerDetail {
    id: string;
    short_id: string;
    name: string;
    image: string;
    image_id: string;
    state: ContainerState;
    status: string;
    health?: string | null;
    restart_policy?: string | null;
    ports: PortBinding[];
    env: KeyValue[];
    labels: KeyValue[];
    mounts: MountInfo[];
    networks: NetworkAttachment[];
    platform: PlatformInfo;
    actions: ContainerActionCapabilities;
    stats?: ContainerStatsSnapshot | null;
}

export type BulkContainerAction =
    | { type: "start" }
    | { type: "stop" }
    | { type: "restart" }
    | { type: "pause" }
    | { type: "unpause" }
    | { type: "remove"; force?: boolean | null; remove_volumes?: boolean | null };

export interface BulkContainerActionResult {
    id: string;
    success: boolean;
    error?: string | null;
}

export type ContainerState =
    | "running"
    | "exited"
    | "paused"
    | "restarting"
    | "dead"
    | "created"
    | string; // fallback for unknown states
