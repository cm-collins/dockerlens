use tauri::State;

use crate::docker::{client::DockerClient, containers};

/// Returns all containers from Docker Engine.
/// Called from the frontend via: invoke('list_containers')
#[tauri::command]
pub async fn list_containers(
    docker: State<'_, DockerClient>,
) -> Result<Vec<containers::ContainerSummary>, String> {
    containers::list_all(&docker).await
}