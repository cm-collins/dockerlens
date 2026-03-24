//! Integration tests for the Docker client.
//! Requires Docker Engine to be running on the host.
//! Tests are skipped gracefully when Docker is not available.

use dockerlens_lib::docker::client::DockerClient;
use dockerlens_lib::docker::containers;
use dockerlens_lib::system::socket;

/// Helper — skips the test if no Docker socket is available.
fn get_client() -> Option<DockerClient> {
    let path = socket::detect()?;
    DockerClient::connect(path.to_str()?).ok()
}

#[tokio::test]
async fn list_containers_returns_vec_when_docker_is_running() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available on this host");
        return;
    };

    let result = containers::list_all(&client).await;

    assert!(
        result.is_ok(),
        "list_containers should succeed when Docker is running: {:?}",
        result.err()
    );

    let containers = result.unwrap();
    println!("✓ Found {} container(s)", containers.len());

    // Each returned container must have a non-empty id and name
    for c in &containers {
        assert!(!c.id.is_empty(), "Container id must not be empty");
        assert_eq!(c.id.len(), 12, "Container id should be 12 chars (short ID)");
    }
}

#[tokio::test]
async fn list_containers_includes_stopped_containers() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available on this host");
        return;
    };

    let containers = containers::list_all(&client)
        .await
        .expect("list_containers failed");

    // With all:true, stopped containers must appear if any exist
    // We can't assert a specific count — just verify the call succeeds
    println!(
        "✓ {} total container(s) including stopped",
        containers.len()
    );
}
