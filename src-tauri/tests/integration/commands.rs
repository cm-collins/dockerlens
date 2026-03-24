//! Integration tests for container commands and typed DTOs.

use dockerlens_lib::docker::client::DockerClient;
use dockerlens_lib::docker::containers::{self, ContainerBulkAction, ContainerListQuery};
use dockerlens_lib::system::socket;

/// Helper to get a Docker client for testing.
fn get_client() -> Option<DockerClient> {
    let path = socket::detect()?;
    DockerClient::connect(path.to_str()?).ok()
}

#[tokio::test]
async fn list_containers_response_returns_typed_payload() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let response = containers::list_response(&client, ContainerListQuery::default())
        .await
        .expect("list_response should succeed");

    assert!(response.total_count >= response.filtered_count);
    assert_eq!(response.overview.total, response.items.len() as u32);

    for container in &response.items {
        assert!(!container.id.is_empty(), "Container ID must not be empty");
        assert!(!container.short_id.is_empty(), "Short ID must not be empty");
        assert_eq!(
            container.short_id.len(),
            12,
            "Short ID should be 12 characters"
        );
    }

    println!(
        "✓ list_response returned {} container(s)",
        response.items.len()
    );
}

#[tokio::test]
async fn list_containers_only_running_filter_returns_running_rows() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let response = containers::list_response(
        &client,
        ContainerListQuery {
            only_running: true,
            ..Default::default()
        },
    )
    .await
    .expect("filtered list_response should succeed");

    assert!(
        response.items.iter().all(|item| item.state == "running"),
        "only_running filter should return running containers only"
    );
}

#[tokio::test]
async fn list_containers_search_filter_limits_matches() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let baseline = containers::list_response(&client, ContainerListQuery::default())
        .await
        .expect("baseline list should succeed");

    if baseline.items.is_empty() {
        println!("Skipping — no containers available for search test");
        return;
    }

    let search_term = baseline.items[0].name.chars().take(3).collect::<String>();

    let filtered = containers::list_response(
        &client,
        ContainerListQuery {
            search: Some(search_term.to_lowercase()),
            ..Default::default()
        },
    )
    .await
    .expect("search list should succeed");

    assert!(
        filtered.items.iter().all(|item| {
            format!("{} {} {} {}", item.id, item.short_id, item.name, item.image)
                .to_lowercase()
                .contains(&search_term.to_lowercase())
        }),
        "Every filtered row should match the search term"
    );
}

#[tokio::test]
async fn get_containers_overview_returns_consistent_counts() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let overview = containers::get_overview(&client)
        .await
        .expect("get_overview should succeed");

    assert!(overview.total >= overview.running);
    assert!(overview.total >= overview.paused);
    assert!(overview.total >= overview.exited);
}

#[tokio::test]
async fn list_containers_handles_no_containers() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let response = containers::list_response(&client, ContainerListQuery::default())
        .await
        .expect("list_response should succeed even with no containers");

    println!("✓ Returned {} container(s)", response.items.len());
}

#[tokio::test]
async fn start_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::start(&client, "invalid/id").await;
    assert!(
        result.is_err(),
        "start_container should fail for invalid ID"
    );

    let err = result.expect_err("error checked above");
    assert!(
        err.contains("invalid characters"),
        "Error should mention invalid characters"
    );
}

#[tokio::test]
async fn start_container_fails_for_empty_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::start(&client, "").await;
    assert!(result.is_err(), "start_container should fail for empty ID");
    assert_eq!(
        result.expect_err("error checked above"),
        "Container ID cannot be empty"
    );
}

#[tokio::test]
async fn stop_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::stop(&client, "../../../etc").await;
    assert!(
        result.is_err(),
        "stop_container should fail for path traversal attempt"
    );
}

#[tokio::test]
async fn restart_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::restart(&client, "abc def").await;
    assert!(
        result.is_err(),
        "restart_container should fail for ID with spaces"
    );
}

#[tokio::test]
async fn pause_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::pause(&client, "abc@def").await;
    assert!(
        result.is_err(),
        "pause_container should fail for ID with special chars"
    );
}

#[tokio::test]
async fn unpause_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let long_id = "a".repeat(129);
    let result = containers::unpause(&client, &long_id).await;
    assert!(
        result.is_err(),
        "unpause_container should fail for too long ID"
    );
}

#[tokio::test]
async fn remove_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::remove(&client, "invalid$id", true, false).await;
    assert!(
        result.is_err(),
        "remove_container should fail for invalid ID"
    );
}

#[tokio::test]
async fn inspect_container_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::inspect(&client, "").await;
    assert!(
        result.is_err(),
        "inspect_container should fail for empty ID"
    );
}

#[tokio::test]
async fn get_stats_fails_for_invalid_id() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let result = containers::get_stats(&client, "invalid/id").await;
    assert!(result.is_err(), "get_stats should fail for invalid ID");
}

#[tokio::test]
async fn get_container_detail_returns_typed_payload_for_existing_container() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let response = containers::list_response(&client, ContainerListQuery::default())
        .await
        .expect("list_response should succeed");

    if response.items.is_empty() {
        println!("Skipping — no containers available for detail testing");
        return;
    }

    let container_id = &response.items[0].id;
    let detail = containers::get_detail(&client, container_id)
        .await
        .expect("get_detail should succeed");

    assert_eq!(detail.id, *container_id);
    assert!(!detail.short_id.is_empty());
    assert!(!detail.name.is_empty() || detail.name.is_empty());
}

#[tokio::test]
async fn inspect_container_returns_json_for_existing_container() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let response = containers::list_response(&client, ContainerListQuery::default())
        .await
        .expect("list_response should succeed");

    if response.items.is_empty() {
        println!("Skipping — no containers available for inspect testing");
        return;
    }

    let container_id = &response.items[0].id;
    let json = containers::inspect(&client, container_id)
        .await
        .expect("inspect should succeed");

    assert!(json.is_object(), "inspect should return JSON object");
}

#[tokio::test]
async fn get_stats_returns_snapshot_for_running_container() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let response = containers::list_response(
        &client,
        ContainerListQuery {
            only_running: true,
            ..Default::default()
        },
    )
    .await
    .expect("running list should succeed");

    let Some(container) = response.items.first() else {
        println!("Skipping — no running containers available for stats testing");
        return;
    };

    let snapshot = containers::get_stats(&client, &container.id)
        .await
        .expect("get_stats should succeed for a running container");

    assert!(snapshot.collected_at_ms > 0);
}

#[tokio::test]
async fn apply_bulk_action_reports_invalid_ids_per_item() {
    let Some(client) = get_client() else {
        println!("Skipping — Docker not available");
        return;
    };

    let results = containers::apply_bulk_action(
        &client,
        &[String::from("invalid/id"), String::from("")],
        &ContainerBulkAction::Start,
    )
    .await
    .expect("bulk action should return per-item results");

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| !result.success));
    assert!(results.iter().all(|result| result.error.is_some()));
}
