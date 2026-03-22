// These types mirror the Rust structs in src-tauri/src/docker/containers.rs exactly.
// Any change to the Rust structs must be reflected here.

export interface PortBinding {
    host_port: string;
    container_port: string;
    protocol: "tcp" | "udp" | "sctp";
}

export interface ContainerSummary {
    /** First 12 characters of the container ID */
    id: string;
    /** Container name without the leading slash */
    name: string;
    image: string;
    /** Human-readable status string e.g. "Up 3 hours" */
    status: string;
    /** Lifecycle state: running | exited | paused | restarting */
    state: ContainerState;
    ports: PortBinding[];
    /** Unix timestamp of creation */
    created: number;
}

export type ContainerState =
    | "running"
    | "exited"
    | "paused"
    | "restarting"
    | "dead"
    | "created"
    | string; // fallback for unknown states