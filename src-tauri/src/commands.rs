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

#[tauri::command]
pub async fn start_container(id: String, docker: State<'_, DockerClient>) -> Result<(), String> {
    containers::start(&docker, &id).await
}

#[tauri::command]
pub async fn stop_container(id: String, docker: State<'_, DockerClient>) -> Result<(), String> {
    containers::stop(&docker, &id).await
}

#[tauri::command]
pub async fn restart_container(id: String, docker: State<'_, DockerClient>) -> Result<(), String> {
    containers::restart(&docker, &id).await
}

#[tauri::command]
pub async fn pause_container(id: String, docker: State<'_, DockerClient>) -> Result<(), String> {
    containers::pause(&docker, &id).await
}

#[tauri::command]
pub async fn unpause_container(id: String, docker: State<'_, DockerClient>) -> Result<(), String> {
    containers::unpause(&docker, &id).await
}

#[tauri::command]
pub async fn remove_container(
    id: String,
    force: Option<bool>,
    remove_volumes: Option<bool>,
    docker: State<'_, DockerClient>,
) -> Result<(), String> {
    containers::remove(
        &docker,
        &id,
        force.unwrap_or(true),
        remove_volumes.unwrap_or(false),
    )
    .await
}

#[tauri::command]
pub async fn inspect_container(
    id: String,
    docker: State<'_, DockerClient>,
) -> Result<serde_json::Value, String> {
    containers::inspect(&docker, &id).await
}

#[tauri::command]
pub async fn get_container_stats(
    id: String,
    docker: State<'_, DockerClient>,
) -> Result<serde_json::Value, String> {
    containers::get_stats(&docker, &id).await
}
