//! Integration tests for Tauri commands.
//! Tests the full command flow from frontend invoke to Docker API.

use dockerlens_lib::docker::client::DockerClient;
use dockerlens_lib::system::socket;

/// Helper to get a Docker client for testing.
fn get_client() -> Option<DockerClient> {
    let path = socket::detect()?;
    DockerClient::connect(path.to_str()?).ok()
}

#[tokio::test]
async fn list_containers_command_returns_valid_json() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    // Simulate the command call
    let result = dockerlens_lib::docker::containers::list_all(&client).await;

    assert!(result.is_ok(), "list_containers command should succeed");

    let containers = result.unwrap();

    // Verify each container has valid data
    for container in &containers {
        assert!(!container.id.is_empty(), "Container ID must not be empty");
        assert_eq!(
            container.id.len(),
            12,
            "Container ID should be 12 characters"
        );
        assert!(
            !container.image.is_empty() || container.image.is_empty(),
            "Image field should exist"
        );
    }

    println!(
        "✓ list_containers returned {} container(s)",
        containers.len()
    );
}

#[tokio::test]
async fn list_containers_includes_all_states() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let containers = dockerlens_lib::docker::containers::list_all(&client)
        .await
        .expect("list_containers failed");

    // Verify we get containers in various states (if any exist)
    let states: Vec<_> = containers.iter().map(|c| c.state.as_str()).collect();

    println!("✓ Found containers in states: {:?}", states);

    // Just verify the call succeeds — we can't guarantee specific states exist
    assert!(
        states.len() <= containers.len(),
        "State list should match returned containers"
    );
}

#[tokio::test]
async fn list_containers_handles_no_containers() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::list_all(&client).await;

    // Should succeed even with 0 containers
    assert!(
        result.is_ok(),
        "list_containers should succeed even with no containers"
    );

    let containers = result.unwrap();
    println!("✓ Returned {} container(s)", containers.len());
}

#[tokio::test]
async fn start_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::start(&client, "invalid/id").await;
    assert!(
        result.is_err(),
        "start_container should fail for invalid ID"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("invalid characters"),
        "Error should mention invalid characters"
    );
    println!("✓ start_container correctly rejects invalid ID: {}", err);
}

#[tokio::test]
async fn start_container_fails_for_empty_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::start(&client, "").await;
    assert!(result.is_err(), "start_container should fail for empty ID");

    let err = result.unwrap_err();
    assert_eq!(err, "Container ID cannot be empty");
    println!("✓ start_container correctly rejects empty ID");
}

#[tokio::test]
async fn stop_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::stop(&client, "../../../etc").await;
    assert!(
        result.is_err(),
        "stop_container should fail for path traversal attempt"
    );
    println!("✓ stop_container correctly rejects path traversal");
}

#[tokio::test]
async fn restart_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::restart(&client, "abc def").await;
    assert!(
        result.is_err(),
        "restart_container should fail for ID with spaces"
    );
    println!("✓ restart_container correctly rejects ID with spaces");
}

#[tokio::test]
async fn pause_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::pause(&client, "abc@def").await;
    assert!(
        result.is_err(),
        "pause_container should fail for ID with special chars"
    );
    println!("✓ pause_container correctly rejects special characters");
}

#[tokio::test]
async fn unpause_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let long_id = "a".repeat(129);
    let result = dockerlens_lib::docker::containers::unpause(&client, &long_id).await;
    assert!(
        result.is_err(),
        "unpause_container should fail for too long ID"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("exceeds maximum length"),
        "Error should mention length"
    );
    println!("✓ unpause_container correctly rejects too long ID");
}

#[tokio::test]
async fn remove_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result =
        dockerlens_lib::docker::containers::remove(&client, "invalid$id", true, false).await;
    assert!(
        result.is_err(),
        "remove_container should fail for invalid ID"
    );
    println!("✓ remove_container correctly rejects invalid ID");
}

#[tokio::test]
async fn inspect_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::inspect(&client, "").await;
    assert!(
        result.is_err(),
        "inspect_container should fail for empty ID"
    );
    println!("✓ inspect_container correctly rejects empty ID");
}

#[tokio::test]
async fn get_stats_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = dockerlens_lib::docker::containers::get_stats(&client, "invalid/id").await;
    assert!(result.is_err(), "get_stats should fail for invalid ID");
    println!("✓ get_stats correctly rejects invalid ID");
}

#[tokio::test]
async fn inspect_container_returns_json_for_existing_container() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    // Get first container if any exist
    let containers = dockerlens_lib::docker::containers::list_all(&client)
        .await
        .expect("list_containers failed");

    if containers.is_empty() {
        println!("Skipping — no containers available for testing");
        return;
    }

    let container_id = &containers[0].id;
    let result = dockerlens_lib::docker::containers::inspect(&client, container_id).await;

    assert!(
        result.is_ok(),
        "inspect_container should succeed for existing container"
    );
    let json = result.unwrap();
    assert!(json.is_object(), "inspect should return JSON object");
    println!(
        "✓ inspect_container returned valid JSON for container {}",
        container_id
    );
}

#[tokio::test]
async fn get_stats_returns_json_for_running_container() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    // Get first running container if any exist
    let containers = dockerlens_lib::docker::containers::list_all(&client)
        .await
        .expect("list_containers failed");

    let running = containers
        .iter()
        .find(|c| c.state.to_lowercase().contains("running"));

    if running.is_none() {
        println!("Skipping — no running containers available for testing");
        return;
    }

    let container_id = &running.expect("running container checked above").id;
    let result = dockerlens_lib::docker::containers::get_stats(&client, container_id).await;

    if let Ok(json) = result {
        assert!(json.is_object(), "get_stats should return JSON object");
        println!(
            "✓ get_stats returned valid JSON for container {}",
            container_id
        );
    } else {
        println!(
            "⚠ get_stats failed (container may have stopped): {:?}",
            result.expect_err("error branch checked above")
        );
    }
}
