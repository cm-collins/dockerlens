mod commands;
mod error;

pub mod docker;
pub mod system;

use docker::client::DockerClient;
use system::socket;

pub fn run() {
    // Detect Docker socket before building the Tauri app.
    // If no socket is found the app still launches — the frontend handles the empty state.
    let client = socket::detect()
        .and_then(|path| {
            log::info!("Docker socket detected at: {}", path.display());
            DockerClient::connect(path.to_str()?).ok()
        })
        .unwrap_or_else(|| {
            log::warn!("No Docker socket found. App will show connection error.");
            // Connect to a placeholder path — bollard is lazy, no panic here
            DockerClient::connect("/var/run/docker.sock")
                .expect("bollard failed to initialise client")
        });

    tauri::Builder::default()
        .manage(client)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_containers,
            commands::start_container,
            commands::stop_container,
            commands::restart_container,
            commands::pause_container,
            commands::unpause_container,
            commands::remove_container,
            commands::inspect_container,
            commands::get_container_stats,
        ])
        .run(tauri::generate_context!())
        .expect("DockerLens failed to start");
}
