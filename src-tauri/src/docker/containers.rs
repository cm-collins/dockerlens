use bollard::models::ContainerSummary as BollardContainer;
use bollard::service::PortSummary;
use bollard::query_parameters::{ListContainersOptionsBuilder, RemoveContainerOptions, RestartContainerOptions, StopContainerOptions, StatsOptions};
use serde::Serialize;
use futures::StreamExt;

use crate::docker::client::DockerClient;

/// Lightweight container summary sent to the frontend over IPC.
#[derive(Debug, Serialize)]
pub struct ContainerSummary {
    /// First 12 characters of the container ID
    pub id: String,
    /// Container name without the leading slash
    pub name: String,
    pub image: String,
    /// Human-readable status string (e.g. "Up 3 hours", "Exited (0) 2 days ago")
    pub status: String,
    /// Lifecycle state (running, stopped, paused, restarting, exited)
    pub state: String,
    pub ports: Vec<PortBinding>,
    /// Unix timestamp of container creation
    pub created: i64,
}

#[derive(Debug, Serialize)]
pub struct PortBinding {
    pub host_port: String,
    pub container_port: String,
    pub protocol: String,
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
    if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("Container ID contains invalid characters".to_string());
    }
    Ok(())
}

/// Returns all containers — running, stopped and paused.
pub async fn list_all(client: &DockerClient) -> Result<Vec<ContainerSummary>, String> {
    let options = ListContainersOptionsBuilder::default()
        .all(true)
        .build();

    let raw = client
        .inner()
        .list_containers(Some(options))
        .await
        .map_err(|e| e.to_string())?;

    Ok(raw.into_iter().filter_map(into_summary).collect())
}

/// Starts a stopped container.
pub async fn start(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    client
        .inner()
        .start_container(id, None)
        .await
        .map_err(|e| format!("Failed to start container {}: {}", id, e))
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
        .map_err(|e| format!("Failed to stop container {}: {}", id, e))
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
        .map_err(|e| format!("Failed to restart container {}: {}", id, e))
}

/// Pauses a running container (freezes all processes).
pub async fn pause(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    client
        .inner()
        .pause_container(id)
        .await
        .map_err(|e| format!("Failed to pause container {}: {}", id, e))
}

/// Unpauses a paused container.
pub async fn unpause(client: &DockerClient, id: &str) -> Result<(), String> {
    validate_container_id(id)?;
    client
        .inner()
        .unpause_container(id)
        .await
        .map_err(|e| format!("Failed to unpause container {}: {}", id, e))
}

/// Removes a container. Force-removes even if running.
pub async fn remove(client: &DockerClient, id: &str, force: bool, remove_volumes: bool) -> Result<(), String> {
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
        .map_err(|e| format!("Failed to remove container {}: {}", id, e))
}

/// Returns full Docker inspect output for a container.
pub async fn inspect(client: &DockerClient, id: &str) -> Result<serde_json::Value, String> {
    validate_container_id(id)?;
    client
        .inner()
        .inspect_container(id, None)
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
        .map_err(|e| format!("Failed to inspect container {}: {}", id, e))
}

/// One-shot stats snapshot — current CPU%, memory, network I/O.
/// For live streaming use subscribe_stats (Phase 6).
pub async fn get_stats(client: &DockerClient, id: &str) -> Result<serde_json::Value, String> {
    validate_container_id(id)?;
    let opts = StatsOptions { stream: false, one_shot: true };
    let stats = client
        .inner()
        .stats(id, Some(opts))
        .next()
        .await
        .ok_or_else(|| "No stats returned".to_string())?
        .map_err(|e| format!("Failed to get stats for {}: {}", id, e))?;
    
    Ok(serde_json::to_value(stats).unwrap_or(serde_json::Value::Null))
}

/// Maps a bollard container model into our leaner ContainerSummary.
/// Returns None for containers with missing required fields (malformed responses).
fn into_summary(c: BollardContainer) -> Option<ContainerSummary> {
    let id = c.id?.chars().take(12).collect();

    // Docker prefixes container names with '/' — strip it for display
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
        .collect();

    Some(ContainerSummary {
        id,
        name,
        image: c.image.unwrap_or_default(),
        status: c.status.unwrap_or_default(),
        state: c.state.map(|s| format!("{:?}", s)).unwrap_or_default(),
        ports,
        created: c.created.unwrap_or_default(),
    })
}

fn into_port_binding(p: PortSummary) -> PortBinding {
    let protocol = match &p.typ {
        Some(t) => format!("{:?}", t).to_lowercase(),
        None => "tcp".to_string(),
    };
    PortBinding {
        host_port: p.public_port.map(|n| n.to_string()).unwrap_or_default(),
        container_port: p.private_port.to_string(),
        protocol,
    }
}
