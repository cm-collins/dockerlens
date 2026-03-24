use std::time::{SystemTime, UNIX_EPOCH};

use bollard::models::ContainerSummary as BollardContainer;
use bollard::query_parameters::{
    ListContainersOptionsBuilder, RemoveContainerOptions, RestartContainerOptions, StatsOptions,
    StopContainerOptions,
};
use bollard::service::PortSummary;
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::docker::client::DockerClient;

const ENRICHMENT_CONCURRENCY: usize = 8;

/// Query contract for the main containers list.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct ContainerListQuery {
    pub all: bool,
    pub only_running: bool,
    pub search: Option<String>,
    pub limit: Option<u32>,
}

impl Default for ContainerListQuery {
    fn default() -> Self {
        Self {
            all: true,
            only_running: false,
            search: None,
            limit: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContainerListResponse {
    pub items: Vec<ContainerSummary>,
    pub overview: ContainersOverviewSummary,
    pub total_count: u32,
    pub filtered_count: u32,
    pub generated_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContainersOverviewSummary {
    pub total: u32,
    pub running: u32,
    pub paused: u32,
    pub exited: u32,
    pub total_cpu_percent: f64,
    pub total_memory_usage_bytes: u64,
    pub total_memory_limit_bytes: u64,
    pub generated_at_ms: u64,
}

/// Lightweight container summary sent to the frontend over IPC.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContainerSummary {
    /// Full container ID for stable identity across commands.
    pub id: String,
    /// First 12 characters of the container ID.
    pub short_id: String,
    /// Container name without the leading slash.
    pub name: String,
    pub image: String,
    /// Human-readable status string (e.g. "Up 3 hours", "Exited (0) 2 days ago").
    pub status: String,
    /// Lifecycle state (running, stopped, paused, restarting, exited).
    pub state: String,
    pub state_reason: Option<String>,
    pub health: Option<String>,
    pub ports: Vec<PortBinding>,
    pub platform: PlatformInfo,
    pub stats: Option<ContainerStatsSnapshot>,
    pub actions: ContainerActionCapabilities,
    /// Unix timestamp of container creation.
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContainerDetail {
    pub id: String,
    pub short_id: String,
    pub name: String,
    pub image: String,
    pub image_id: String,
    pub state: String,
    pub status: String,
    pub health: Option<String>,
    pub restart_policy: Option<String>,
    pub ports: Vec<PortBinding>,
    pub env: Vec<KeyValue>,
    pub labels: Vec<KeyValue>,
    pub mounts: Vec<MountInfo>,
    pub networks: Vec<NetworkAttachment>,
    pub platform: PlatformInfo,
    pub actions: ContainerActionCapabilities,
    pub stats: Option<ContainerStatsSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PortBinding {
    pub host_ip: String,
    pub host_port: String,
    pub container_port: String,
    pub protocol: String,
    pub browser_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MountInfo {
    pub source: String,
    pub destination: String,
    pub mode: Option<String>,
    pub rw: Option<bool>,
    pub mount_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct NetworkAttachment {
    pub name: String,
    pub ip_address: Option<String>,
    pub gateway: Option<String>,
    pub mac_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlatformInfo {
    pub os: Option<String>,
    pub architecture: Option<String>,
    pub image_architecture_mismatch: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ContainerActionCapabilities {
    pub can_start: bool,
    pub can_stop: bool,
    pub can_restart: bool,
    pub can_pause: bool,
    pub can_unpause: bool,
    pub can_remove: bool,
    pub can_open_port: bool,
    pub can_inspect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContainerStatsSnapshot {
    pub cpu_percent: Option<f64>,
    pub memory_usage_bytes: Option<u64>,
    pub memory_limit_bytes: Option<u64>,
    pub memory_percent: Option<f64>,
    pub network_rx_bytes: Option<u64>,
    pub network_tx_bytes: Option<u64>,
    pub collected_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContainerBulkAction {
    Start,
    Stop,
    Restart,
    Pause,
    Unpause,
    Remove {
        force: Option<bool>,
        remove_volumes: Option<bool>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkContainerActionResult {
    pub id: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Shared input validation for all container ID parameters.
/// DRY — called by every command that takes a container ID.
#[doc(hidden)]
pub fn validate_container_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("Container ID cannot be empty".to_string());
    }
    if id.len() > 128 {
        return Err("Container ID exceeds maximum length".to_string());
    }
    if !id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Container ID contains invalid characters".to_string());
    }
    Ok(())
}

#[doc(hidden)]
pub fn humanize_docker_error(action: &str, subject: &str, raw_error: &str) -> String {
    let normalized = raw_error.to_lowercase();

    if let Some(path) = extract_missing_bind_mount_path(raw_error) {
        return format!(
            "Failed to {action} {subject}: host bind mount path does not exist: {path}. Create the folder or recreate the container with a valid mount."
        );
    }

    if normalized.contains("permission denied") {
        return format!(
            "Failed to {action} {subject}: permission denied while talking to Docker. Ensure your user can access the Docker socket."
        );
    }

    if normalized.contains("no such container") || normalized.contains("404 page not found") {
        return format!("Failed to {action} {subject}: the container no longer exists in Docker.");
    }

    if normalized.contains("no such file or directory")
        || normalized.contains("cannot connect to the docker daemon")
        || normalized.contains("error while fetching server api version")
        || normalized.contains("connection refused")
    {
        return format!(
            "Failed to {action} {subject}: Docker is unavailable. Ensure the daemon is running and the socket is reachable."
        );
    }

    format!("Failed to {action} {subject}: {raw_error}")
}

fn extract_missing_bind_mount_path(raw_error: &str) -> Option<&str> {
    raw_error
        .split("bind source path does not exist:")
        .nth(1)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

/// Shared action-capability mapping so list and detail stay consistent.
#[doc(hidden)]
pub fn action_capabilities_for_state(
    state: &str,
    can_open_port: bool,
) -> ContainerActionCapabilities {
    let normalized = state.to_lowercase();

    let mut actions = ContainerActionCapabilities {
        can_remove: true,
        can_inspect: true,
        can_open_port,
        ..Default::default()
    };

    match normalized.as_str() {
        "running" => {
            actions.can_stop = true;
            actions.can_restart = true;
            actions.can_pause = true;
        }
        "paused" => {
            actions.can_stop = true;
            actions.can_unpause = true;
        }
        "exited" | "dead" | "created" => {
            actions.can_start = true;
        }
        "restarting" => {
            actions.can_stop = true;
        }
        _ => {}
    }

    actions
}

#[doc(hidden)]
pub fn browser_url_for_binding(host_ip: &str, host_port: &str, protocol: &str) -> Option<String> {
    if host_port.is_empty() {
        return None;
    }

    let host = match host_ip {
        "" | "0.0.0.0" | "::" | ":::" => "localhost",
        other => other,
    };

    let scheme = match protocol {
        "https" => "https",
        _ => "http",
    };

    Some(format!("{scheme}://{host}:{host_port}"))
}

/// Returns all containers — running, stopped and paused.
pub async fn list_all(client: &DockerClient) -> Result<Vec<ContainerSummary>, String> {
    Ok(list_response(client, ContainerListQuery::default())
        .await?
        .items)
}

/// Returns the typed response backing the main containers table.
pub async fn list_response(
    client: &DockerClient,
    query: ContainerListQuery,
) -> Result<ContainerListResponse, String> {
    let normalized_query = query;
    let options = ListContainersOptionsBuilder::default()
        .all(normalized_query.all)
        .build();

    let raw = client
        .inner()
        .list_containers(Some(options))
        .await
        .map_err(|e| humanize_docker_error("list", "containers", &e.to_string()))?;

    let total_count = raw.len() as u32;

    let mut items: Vec<_> = raw.into_iter().filter_map(into_summary).collect();
    items = apply_query_filters(items, &normalized_query);
    let filtered_count = items.len() as u32;

    if let Some(limit) = normalized_query.limit {
        items.truncate(limit as usize);
    }

    let items = enrich_summaries_with_stats(client, items).await;
    let overview = build_overview(&items);

    Ok(ContainerListResponse {
        items,
        overview,
        total_count,
        filtered_count,
        generated_at_ms: now_ms(),
    })
}

/// Returns the top-of-screen container overview.
pub async fn get_overview(client: &DockerClient) -> Result<ContainersOverviewSummary, String> {
    Ok(list_response(client, ContainerListQuery::default())
        .await?
        .overview)
}

/// Returns typed detail data for a selected container.
pub async fn get_detail(client: &DockerClient, id: &str) -> Result<ContainerDetail, String> {
    validate_container_id(id)?;
    let inspect_value = inspect(client, id).await?;
    let stats = get_stats(client, id).await.ok();
    Ok(container_detail_from_value(&inspect_value, stats))
}

/// Starts a stopped container.
pub async fn start(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    client
        .inner()
        .start_container(id, None)
        .await
        .map_err(|e| humanize_docker_error("start", &format!("container {id}"), &e.to_string()))
}

/// Stops a running container gracefully.
/// Waits up to 10 seconds before forcing termination.
pub async fn stop(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    let options = StopContainerOptions {
        t: Some(10),
        signal: None,
    };

    client
        .inner()
        .stop_container(id, Some(options))
        .await
        .map_err(|e| humanize_docker_error("stop", &format!("container {id}"), &e.to_string()))
}

/// Restarts a container.
pub async fn restart(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    let options = RestartContainerOptions {
        t: Some(10),
        signal: None,
    };

    client
        .inner()
        .restart_container(id, Some(options))
        .await
        .map_err(|e| humanize_docker_error("restart", &format!("container {id}"), &e.to_string()))
}

/// Pauses a running container (freezes all processes).
pub async fn pause(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    client
        .inner()
        .pause_container(id)
        .await
        .map_err(|e| humanize_docker_error("pause", &format!("container {id}"), &e.to_string()))
}

/// Unpauses a paused container.
pub async fn unpause(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    client
        .inner()
        .unpause_container(id)
        .await
        .map_err(|e| humanize_docker_error("unpause", &format!("container {id}"), &e.to_string()))
}

/// Removes a container. Force-removes even if running.
pub async fn remove(
    client: &DockerClient,
    id: &str,
    force: bool,
    remove_volumes: bool,
) -> Result<(), String> {
    validate_container_id(id)?;
    let options = RemoveContainerOptions {
        force,
        v: remove_volumes,
        ..Default::default()
    };

    client
        .inner()
        .remove_container(id, Some(options))
        .await
        .map_err(|e| humanize_docker_error("remove", &format!("container {id}"), &e.to_string()))
}

/// Returns per-container outcomes for bulk operations.
pub async fn apply_bulk_action(
    client: &DockerClient,
    ids: &[String],
    action: &ContainerBulkAction,
) -> Result<Vec<BulkContainerActionResult>, String> {
    let mut results = Vec::with_capacity(ids.len());

    for id in ids {
        let outcome = match action {
            ContainerBulkAction::Start => start(client, id).await,
            ContainerBulkAction::Stop => stop(client, id).await,
            ContainerBulkAction::Restart => restart(client, id).await,
            ContainerBulkAction::Pause => pause(client, id).await,
            ContainerBulkAction::Unpause => unpause(client, id).await,
            ContainerBulkAction::Remove {
                force,
                remove_volumes,
            } => {
                remove(
                    client,
                    id,
                    force.unwrap_or(true),
                    remove_volumes.unwrap_or(false),
                )
                .await
            }
        };

        match outcome {
            Ok(()) => results.push(BulkContainerActionResult {
                id: id.clone(),
                success: true,
                error: None,
            }),
            Err(error) => results.push(BulkContainerActionResult {
                id: id.clone(),
                success: false,
                error: Some(error),
            }),
        }
    }

    Ok(results)
}

/// Returns full Docker inspect output for a container.
pub async fn inspect(client: &DockerClient, id: &str) -> Result<serde_json::Value, String> {
    validate_container_id(id)?;
    client
        .inner()
        .inspect_container(id, None)
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| humanize_docker_error("inspect", &format!("container {id}"), &e.to_string()))
}

/// One-shot stats snapshot — current CPU%, memory, network I/O.
/// For live streaming use subscribe_stats (Phase 6).
pub async fn get_stats(client: &DockerClient, id: &str) -> Result<ContainerStatsSnapshot, String> {
    validate_container_id(id)?;
    let opts = StatsOptions {
        stream: false,
        // Docker needs two collection cycles to populate precpu_stats, which we
        // rely on for the CPU percentage delta calculation.
        one_shot: false,
    };
    let stats = client
        .inner()
        .stats(id, Some(opts))
        .next()
        .await
        .ok_or_else(|| "No stats returned".to_string())?
        .map_err(|e| {
            humanize_docker_error("get stats for", &format!("container {id}"), &e.to_string())
        })?;

    let stats_value = serde_json::to_value(stats).unwrap_or(Value::Null);
    Ok(stats_snapshot_from_value(&stats_value))
}

#[doc(hidden)]
pub fn stats_snapshot_from_value(value: &Value) -> ContainerStatsSnapshot {
    let cpu_total = value
        .pointer("/cpu_stats/cpu_usage/total_usage")
        .and_then(Value::as_u64);
    let precpu_total = value
        .pointer("/precpu_stats/cpu_usage/total_usage")
        .and_then(Value::as_u64);
    let system_total = value
        .pointer("/cpu_stats/system_cpu_usage")
        .and_then(Value::as_u64);
    let presystem_total = value
        .pointer("/precpu_stats/system_cpu_usage")
        .and_then(Value::as_u64);

    let online_cpus = value
        .pointer("/cpu_stats/online_cpus")
        .and_then(Value::as_u64)
        .or_else(|| {
            value
                .pointer("/cpu_stats/cpu_usage/percpu_usage")
                .and_then(Value::as_array)
                .map(|cpus| cpus.len() as u64)
        });

    let cpu_percent = match (
        cpu_total,
        precpu_total,
        system_total,
        presystem_total,
        online_cpus,
    ) {
        (Some(cpu), Some(precpu), Some(system), Some(presystem), Some(cpus))
            if system > presystem && cpu >= precpu && cpus > 0 =>
        {
            let cpu_delta = (cpu - precpu) as f64;
            let system_delta = (system - presystem) as f64;
            Some((cpu_delta / system_delta) * cpus as f64 * 100.0)
        }
        _ => None,
    };

    let memory_usage_bytes = value.pointer("/memory_stats/usage").and_then(Value::as_u64);
    let memory_limit_bytes = value.pointer("/memory_stats/limit").and_then(Value::as_u64);
    let memory_percent = match (memory_usage_bytes, memory_limit_bytes) {
        (Some(usage), Some(limit)) if limit > 0 => Some((usage as f64 / limit as f64) * 100.0),
        _ => None,
    };

    let (network_rx_bytes, network_tx_bytes) = sum_network_bytes(value);

    ContainerStatsSnapshot {
        cpu_percent,
        memory_usage_bytes,
        memory_limit_bytes,
        memory_percent,
        network_rx_bytes,
        network_tx_bytes,
        collected_at_ms: now_ms(),
    }
}

#[doc(hidden)]
pub fn container_detail_from_value(
    value: &Value,
    stats: Option<ContainerStatsSnapshot>,
) -> ContainerDetail {
    let id = string_at(value, "/Id").unwrap_or_default();
    let short_id = id.chars().take(12).collect();
    let name = string_at(value, "/Name")
        .map(|name| name.trim_start_matches('/').to_string())
        .unwrap_or_default();
    let image = string_at(value, "/Config/Image").unwrap_or_default();
    let image_id = string_at(value, "/Image").unwrap_or_default();
    let state = string_at(value, "/State/Status").unwrap_or_default();
    let status = state.clone();
    let health = string_at(value, "/State/Health/Status");
    let restart_policy = string_at(value, "/HostConfig/RestartPolicy/Name");
    let platform = platform_info_from_inspect(value);
    let ports = ports_from_inspect(value);
    let actions = action_capabilities_for_state(&state, ports.iter().any(has_browser_url));

    ContainerDetail {
        id,
        short_id,
        name,
        image,
        image_id,
        state,
        status,
        health,
        restart_policy,
        ports,
        env: env_from_inspect(value),
        labels: labels_from_inspect(value),
        mounts: mounts_from_inspect(value),
        networks: networks_from_inspect(value),
        platform,
        actions,
        stats,
    }
}

async fn enrich_summaries_with_stats(
    client: &DockerClient,
    items: Vec<ContainerSummary>,
) -> Vec<ContainerSummary> {
    let mut enriched = stream::iter(items.into_iter().enumerate().map(
        |(index, mut item)| async move {
            if item.state == "running" {
                item.stats = get_stats(client, &item.id).await.ok();
            }

            item.actions =
                action_capabilities_for_state(&item.state, item.ports.iter().any(has_browser_url));

            (index, item)
        },
    ))
    .buffer_unordered(ENRICHMENT_CONCURRENCY)
    .collect::<Vec<_>>()
    .await;

    enriched.sort_by_key(|(index, _)| *index);
    enriched.into_iter().map(|(_, item)| item).collect()
}

fn apply_query_filters(
    items: Vec<ContainerSummary>,
    query: &ContainerListQuery,
) -> Vec<ContainerSummary> {
    let search = query.search.as_ref().map(|value| value.to_lowercase());

    items
        .into_iter()
        .filter(|item| {
            if query.only_running && item.state != "running" {
                return false;
            }

            if let Some(search) = &search {
                let haystack =
                    format!("{} {} {} {}", item.id, item.short_id, item.name, item.image)
                        .to_lowercase();
                haystack.contains(search)
            } else {
                true
            }
        })
        .collect()
}

fn build_overview(items: &[ContainerSummary]) -> ContainersOverviewSummary {
    let mut overview = ContainersOverviewSummary {
        total: items.len() as u32,
        generated_at_ms: now_ms(),
        ..Default::default()
    };

    for item in items {
        match item.state.as_str() {
            "running" => overview.running += 1,
            "paused" => overview.paused += 1,
            "exited" => overview.exited += 1,
            _ => {}
        }

        if let Some(stats) = &item.stats {
            overview.total_cpu_percent += stats.cpu_percent.unwrap_or(0.0);
            overview.total_memory_usage_bytes += stats.memory_usage_bytes.unwrap_or(0);
            overview.total_memory_limit_bytes += stats.memory_limit_bytes.unwrap_or(0);
        }
    }

    overview
}

/// Maps a bollard container model into our richer ContainerSummary.
/// Returns None for containers with missing required fields (malformed responses).
fn into_summary(c: BollardContainer) -> Option<ContainerSummary> {
    let id = c.id?;
    let short_id = id.chars().take(12).collect::<String>();

    // Docker prefixes container names with '/' — strip it for display.
    let name = c
        .names?
        .into_iter()
        .next()
        .map(|n| n.trim_start_matches('/').to_owned())
        .unwrap_or_default();

    let ports = c
        .ports
        .unwrap_or_default()
        .into_iter()
        .map(into_port_binding)
        .collect::<Vec<_>>();

    let state = c
        .state
        .map(|state| format!("{state:?}").to_lowercase())
        .unwrap_or_else(|| "unknown".to_string());
    let status = c.status.unwrap_or_default();

    Some(ContainerSummary {
        id,
        short_id,
        name,
        image: c.image.unwrap_or_default(),
        status: status.clone(),
        state: state.clone(),
        state_reason: Some(state.clone()),
        health: health_from_status(&status),
        ports: ports.clone(),
        platform: PlatformInfo::default(),
        stats: None,
        actions: action_capabilities_for_state(&state, ports.iter().any(has_browser_url)),
        created: c.created.unwrap_or_default(),
    })
}

fn into_port_binding(p: PortSummary) -> PortBinding {
    let host_ip = p.ip.unwrap_or_default();
    let host_port = p.public_port.map(|n| n.to_string()).unwrap_or_default();
    let protocol = match &p.typ {
        Some(typ) => format!("{typ:?}").to_lowercase(),
        None => "tcp".to_string(),
    };

    PortBinding {
        host_ip: host_ip.clone(),
        host_port: host_port.clone(),
        container_port: p.private_port.to_string(),
        browser_url: browser_url_for_binding(&host_ip, &host_port, &protocol),
        protocol,
    }
}

fn ports_from_inspect(value: &Value) -> Vec<PortBinding> {
    let mut ports = Vec::new();

    let Some(bindings) = value
        .pointer("/NetworkSettings/Ports")
        .and_then(Value::as_object)
    else {
        return ports;
    };

    for (container_binding, host_mappings) in bindings {
        let (container_port, protocol) = container_binding
            .split_once('/')
            .map(|(port, proto)| (port.to_string(), proto.to_string()))
            .unwrap_or_else(|| (container_binding.clone(), "tcp".to_string()));

        if let Some(mappings) = host_mappings.as_array() {
            for mapping in mappings {
                let host_ip = string_at(mapping, "/HostIp").unwrap_or_default();
                let host_port = string_at(mapping, "/HostPort").unwrap_or_default();

                ports.push(PortBinding {
                    host_ip: host_ip.clone(),
                    host_port: host_port.clone(),
                    container_port: container_port.clone(),
                    browser_url: browser_url_for_binding(&host_ip, &host_port, &protocol),
                    protocol: protocol.clone(),
                });
            }
        } else {
            ports.push(PortBinding {
                host_ip: String::new(),
                host_port: String::new(),
                container_port,
                browser_url: None,
                protocol,
            });
        }
    }

    ports
}

fn env_from_inspect(value: &Value) -> Vec<KeyValue> {
    value
        .pointer("/Config/Env")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|entry| match entry.split_once('=') {
            Some((key, value)) => KeyValue {
                key: key.to_string(),
                value: value.to_string(),
            },
            None => KeyValue {
                key: entry.to_string(),
                value: String::new(),
            },
        })
        .collect()
}

fn labels_from_inspect(value: &Value) -> Vec<KeyValue> {
    value
        .pointer("/Config/Labels")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|labels| labels.iter())
        .map(|(key, value)| KeyValue {
            key: key.clone(),
            value: value.as_str().unwrap_or_default().to_string(),
        })
        .collect()
}

fn mounts_from_inspect(value: &Value) -> Vec<MountInfo> {
    value
        .pointer("/Mounts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|mount| MountInfo {
            source: string_at(mount, "/Source").unwrap_or_default(),
            destination: string_at(mount, "/Destination").unwrap_or_default(),
            mode: string_at(mount, "/Mode"),
            rw: mount.pointer("/RW").and_then(Value::as_bool),
            mount_type: string_at(mount, "/Type"),
        })
        .collect()
}

fn networks_from_inspect(value: &Value) -> Vec<NetworkAttachment> {
    value
        .pointer("/NetworkSettings/Networks")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|networks| networks.iter())
        .map(|(name, network)| NetworkAttachment {
            name: name.clone(),
            ip_address: string_at(network, "/IPAddress"),
            gateway: string_at(network, "/Gateway"),
            mac_address: string_at(network, "/MacAddress"),
        })
        .collect()
}

fn platform_info_from_inspect(value: &Value) -> PlatformInfo {
    let platform_raw = string_at(value, "/Platform");
    let os = string_at(value, "/Os").or_else(|| {
        platform_raw
            .as_ref()
            .and_then(|platform| platform.split('/').next().map(str::to_string))
    });
    let architecture = string_at(value, "/Architecture").or_else(|| {
        platform_raw
            .as_ref()
            .and_then(|platform| platform.split('/').nth(1).map(str::to_string))
    });

    let image_architecture_mismatch = architecture
        .as_deref()
        .map(is_architecture_mismatch)
        .unwrap_or(false);

    PlatformInfo {
        os,
        architecture,
        image_architecture_mismatch,
    }
}

fn health_from_status(status: &str) -> Option<String> {
    let normalized = status.to_lowercase();
    if normalized.contains("unhealthy") {
        Some("unhealthy".to_string())
    } else if normalized.contains("healthy") {
        Some("healthy".to_string())
    } else {
        None
    }
}

fn has_browser_url(binding: &PortBinding) -> bool {
    binding.browser_url.is_some()
}

fn sum_network_bytes(value: &Value) -> (Option<u64>, Option<u64>) {
    let Some(networks) = value.pointer("/networks").and_then(Value::as_object) else {
        return (None, None);
    };

    let mut total_rx = 0_u64;
    let mut total_tx = 0_u64;
    let mut saw_value = false;

    for network in networks.values() {
        if let Some(rx) = network.get("rx_bytes").and_then(Value::as_u64) {
            total_rx += rx;
            saw_value = true;
        }
        if let Some(tx) = network.get("tx_bytes").and_then(Value::as_u64) {
            total_tx += tx;
            saw_value = true;
        }
    }

    if saw_value {
        (Some(total_rx), Some(total_tx))
    } else {
        (None, None)
    }
}

fn string_at(value: &Value, pointer: &str) -> Option<String> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn is_architecture_mismatch(architecture: &str) -> bool {
    normalize_architecture(architecture) != normalize_architecture(std::env::consts::ARCH)
}

fn normalize_architecture(architecture: &str) -> &str {
    match architecture {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        value => value,
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}
