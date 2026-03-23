use bollard::models::ContainerSummary as BollardContainer;
use bollard::service::PortSummary;
use bollard::query_parameters::ListContainersOptionsBuilder;
use serde::Serialize;

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
