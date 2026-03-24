use tauri::State;

use crate::docker::{client::DockerClient, containers};

/// Returns the main typed containers list response.
#[tauri::command]
pub async fn list_containers(
    query: Option<containers::ContainerListQuery>,
    docker: State<'_, DockerClient>,
) -> Result<containers::ContainerListResponse, String> {
    containers::list_response(&docker, query.unwrap_or_default()).await
}

#[tauri::command]
pub async fn get_containers_overview(
    docker: State<'_, DockerClient>,
) -> Result<containers::ContainersOverviewSummary, String> {
    containers::get_overview(&docker).await
}

#[tauri::command]
pub async fn get_container_detail(
    id: String,
    docker: State<'_, DockerClient>,
) -> Result<containers::ContainerDetail, String> {
    containers::get_detail(&docker, &id).await
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
pub async fn apply_container_action(
    ids: Vec<String>,
    action: containers::ContainerBulkAction,
    docker: State<'_, DockerClient>,
) -> Result<Vec<containers::BulkContainerActionResult>, String> {
    containers::apply_bulk_action(&docker, &ids, &action).await
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
) -> Result<containers::ContainerStatsSnapshot, String> {
    containers::get_stats(&docker, &id).await
}
