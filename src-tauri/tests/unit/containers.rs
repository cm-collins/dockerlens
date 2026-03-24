//! Unit tests for container DTOs and transformation helpers.

use dockerlens_lib::docker::containers::{
    action_capabilities_for_state, browser_url_for_binding, container_detail_from_value,
    humanize_docker_error, stats_snapshot_from_value, validate_container_id,
    ContainerActionCapabilities, ContainerStatsSnapshot, ContainerSummary, PlatformInfo,
    PortBinding,
};
use serde_json::json;

fn sample_summary() -> ContainerSummary {
    ContainerSummary {
        id: "abcdef1234567890".to_string(),
        short_id: "abcdef123456".to_string(),
        name: "test-container".to_string(),
        image: "nginx:latest".to_string(),
        status: "Up 1 hour".to_string(),
        state: "running".to_string(),
        state_reason: Some("running".to_string()),
        health: Some("healthy".to_string()),
        ports: vec![],
        platform: PlatformInfo::default(),
        stats: None,
        actions: ContainerActionCapabilities {
            can_stop: true,
            can_restart: true,
            can_pause: true,
            can_remove: true,
            can_inspect: true,
            ..Default::default()
        },
        created: 1234567890,
    }
}

// ========== DTO Serialization ==========

#[test]
fn container_summary_serializes_to_json() {
    let summary = sample_summary();

    let json = serde_json::to_string(&summary);
    assert!(json.is_ok(), "ContainerSummary should serialize to JSON");

    let json_str = json.expect("serialization checked above");
    assert!(json_str.contains("abcdef1234567890"));
    assert!(json_str.contains("test-container"));
    assert!(json_str.contains("short_id"));
}

#[test]
fn port_binding_serializes_to_json() {
    let binding = PortBinding {
        host_ip: "0.0.0.0".to_string(),
        host_port: "8080".to_string(),
        container_port: "80".to_string(),
        protocol: "tcp".to_string(),
        browser_url: Some("http://localhost:8080".to_string()),
    };

    let json = serde_json::to_string(&binding);
    assert!(json.is_ok(), "PortBinding should serialize to JSON");
}

#[test]
fn container_summary_has_correct_fields() {
    let summary = sample_summary();

    assert_eq!(summary.id, "abcdef1234567890");
    assert_eq!(summary.short_id, "abcdef123456");
    assert_eq!(summary.name, "test-container");
    assert_eq!(summary.image, "nginx:latest");
    assert_eq!(summary.state, "running");
    assert_eq!(summary.health.as_deref(), Some("healthy"));
}

#[test]
fn port_binding_has_correct_fields() {
    let binding = PortBinding {
        host_ip: "127.0.0.1".to_string(),
        host_port: "3000".to_string(),
        container_port: "3000".to_string(),
        protocol: "tcp".to_string(),
        browser_url: Some("http://127.0.0.1:3000".to_string()),
    };

    assert_eq!(binding.host_ip, "127.0.0.1");
    assert_eq!(binding.host_port, "3000");
    assert_eq!(binding.container_port, "3000");
    assert_eq!(binding.protocol, "tcp");
}

#[test]
fn container_summary_with_multiple_ports() {
    let mut summary = sample_summary();
    summary.ports = vec![
        PortBinding {
            host_ip: "0.0.0.0".to_string(),
            host_port: "8080".to_string(),
            container_port: "80".to_string(),
            protocol: "tcp".to_string(),
            browser_url: Some("http://localhost:8080".to_string()),
        },
        PortBinding {
            host_ip: "0.0.0.0".to_string(),
            host_port: "8443".to_string(),
            container_port: "443".to_string(),
            protocol: "tcp".to_string(),
            browser_url: Some("http://localhost:8443".to_string()),
        },
    ];

    assert_eq!(summary.ports.len(), 2);
    assert_eq!(summary.ports[0].host_port, "8080");
    assert_eq!(summary.ports[1].host_port, "8443");
}

#[test]
fn container_summary_with_empty_ports() {
    let summary = sample_summary();
    assert!(summary.ports.is_empty());
}

#[test]
fn container_summary_debug_format() {
    let summary = sample_summary();

    let debug_str = format!("{:?}", summary);
    assert!(debug_str.contains("abcdef1234567890"));
    assert!(debug_str.contains("test-container"));
}

// ========== Input Validation ==========

#[test]
fn validate_container_id_accepts_valid_ids() {
    assert!(validate_container_id("abc123").is_ok());
    assert!(validate_container_id("a1b2c3d4e5f6").is_ok());
    assert!(validate_container_id("my-container_1").is_ok());
    assert!(validate_container_id("ABC123").is_ok());
    assert!(validate_container_id("Container123").is_ok());
}

#[test]
fn validate_container_id_rejects_empty() {
    let result = validate_container_id("");
    assert!(result.is_err());
    assert_eq!(
        result.expect_err("error checked above"),
        "Container ID cannot be empty"
    );
}

#[test]
fn validate_container_id_rejects_too_long() {
    let long_id = "a".repeat(129);
    let result = validate_container_id(&long_id);
    assert!(result.is_err());
    assert_eq!(
        result.expect_err("error checked above"),
        "Container ID exceeds maximum length"
    );
}

#[test]
fn validate_container_id_rejects_special_chars() {
    assert!(validate_container_id("abc/def").is_err());
    assert!(validate_container_id("../../../etc").is_err());
    assert!(validate_container_id("abc def").is_err());
    assert!(validate_container_id("abc@def").is_err());
    assert!(validate_container_id("abc$def").is_err());
}

#[test]
fn validate_container_id_max_length_boundary() {
    let max_valid = "a".repeat(128);
    assert!(validate_container_id(&max_valid).is_ok());
}

#[test]
fn validate_container_id_allows_hyphens() {
    assert!(validate_container_id("my-container-name").is_ok());
    assert!(validate_container_id("test-123-abc").is_ok());
}

#[test]
fn validate_container_id_allows_underscores() {
    assert!(validate_container_id("my_container_name").is_ok());
    assert!(validate_container_id("test_123_abc").is_ok());
}

#[test]
fn validate_container_id_rejects_path_traversal() {
    assert!(validate_container_id("../etc/passwd").is_err());
    assert!(validate_container_id("../../root").is_err());
    assert!(validate_container_id("./../test").is_err());
}

#[test]
fn validate_container_id_rejects_command_injection() {
    assert!(validate_container_id("test; rm -rf /").is_err());
    assert!(validate_container_id("test && echo hack").is_err());
    assert!(validate_container_id("test | cat /etc/passwd").is_err());
}

#[test]
fn validate_container_id_rejects_shell_metacharacters() {
    assert!(validate_container_id("test$var").is_err());
    assert!(validate_container_id("test`whoami`").is_err());
    assert!(validate_container_id("test$(id)").is_err());
    assert!(validate_container_id("test&background").is_err());
    assert!(validate_container_id("test>output").is_err());
    assert!(validate_container_id("test<input").is_err());
}

#[test]
fn validate_container_id_single_character() {
    assert!(validate_container_id("a").is_ok());
    assert!(validate_container_id("1").is_ok());
}

#[test]
fn validate_container_id_exactly_128_chars() {
    let id = "a".repeat(128);
    assert_eq!(id.len(), 128);
    assert!(validate_container_id(&id).is_ok());
}

#[test]
fn validate_container_id_129_chars_fails() {
    let id = "a".repeat(129);
    assert_eq!(id.len(), 129);
    assert!(validate_container_id(&id).is_err());
}

// ========== New Phase 2 Helper Tests ==========

#[test]
fn action_capabilities_for_running_container_enable_expected_actions() {
    let actions = action_capabilities_for_state("running", true);

    assert!(actions.can_stop);
    assert!(actions.can_restart);
    assert!(actions.can_pause);
    assert!(actions.can_remove);
    assert!(actions.can_inspect);
    assert!(actions.can_open_port);
    assert!(!actions.can_start);
    assert!(!actions.can_unpause);
}

#[test]
fn action_capabilities_for_paused_container_enable_unpause() {
    let actions = action_capabilities_for_state("paused", false);

    assert!(actions.can_unpause);
    assert!(actions.can_stop);
    assert!(actions.can_remove);
    assert!(!actions.can_pause);
    assert!(!actions.can_open_port);
}

#[test]
fn browser_url_uses_localhost_for_wildcard_host() {
    let url = browser_url_for_binding("0.0.0.0", "8080", "tcp");
    assert_eq!(url.as_deref(), Some("http://localhost:8080"));
}

#[test]
fn browser_url_preserves_specific_host() {
    let url = browser_url_for_binding("127.0.0.1", "3000", "tcp");
    assert_eq!(url.as_deref(), Some("http://127.0.0.1:3000"));
}

#[test]
fn browser_url_is_none_without_public_port() {
    let url = browser_url_for_binding("", "", "tcp");
    assert!(url.is_none());
}

#[test]
fn humanize_docker_error_explains_missing_bind_mount_path() {
    let message = humanize_docker_error(
        "start",
        "container demo",
        "Docker responded with status code 400: invalid mount config for type \"bind\": bind source path does not exist: /tmp/demo",
    );

    assert!(message.contains("host bind mount path does not exist"));
    assert!(message.contains("/tmp/demo"));
}

#[test]
fn humanize_docker_error_explains_permission_denied() {
    let message = humanize_docker_error(
        "stop",
        "container demo",
        "error while connecting to Docker: permission denied",
    );

    assert!(message.contains("permission denied"));
    assert!(message.contains("access the Docker socket"));
}

#[test]
fn humanize_docker_error_explains_missing_container() {
    let message = humanize_docker_error(
        "inspect",
        "container demo",
        "Docker responded with status code 404: No such container: demo",
    );

    assert!(message.contains("no longer exists"));
}

#[test]
fn humanize_docker_error_explains_daemon_unavailable() {
    let message = humanize_docker_error(
        "list",
        "containers",
        "Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?",
    );

    assert!(message.contains("Docker is unavailable"));
    assert!(message.contains("daemon is running"));
}

#[test]
fn stats_snapshot_from_value_calculates_cpu_memory_and_network_totals() {
    let snapshot = stats_snapshot_from_value(&json!({
        "cpu_stats": {
            "cpu_usage": { "total_usage": 200, "percpu_usage": [100, 100] },
            "system_cpu_usage": 1000,
            "online_cpus": 2
        },
        "precpu_stats": {
            "cpu_usage": { "total_usage": 100 },
            "system_cpu_usage": 500
        },
        "memory_stats": {
            "usage": 256,
            "limit": 1024
        },
        "networks": {
            "eth0": { "rx_bytes": 10, "tx_bytes": 20 },
            "eth1": { "rx_bytes": 5, "tx_bytes": 15 }
        }
    }));

    assert_eq!(snapshot.cpu_percent, Some(40.0));
    assert_eq!(snapshot.memory_usage_bytes, Some(256));
    assert_eq!(snapshot.memory_limit_bytes, Some(1024));
    assert_eq!(snapshot.memory_percent, Some(25.0));
    assert_eq!(snapshot.network_rx_bytes, Some(15));
    assert_eq!(snapshot.network_tx_bytes, Some(35));
}

#[test]
fn container_detail_from_value_extracts_typed_fields() {
    let detail = container_detail_from_value(
        &json!({
            "Id": "abcdef1234567890",
            "Name": "/demo",
            "Image": "sha256:123",
            "Platform": "linux/amd64",
            "State": {
                "Status": "running",
                "Health": { "Status": "healthy" }
            },
            "Config": {
                "Image": "nginx:latest",
                "Env": ["FOO=bar", "EMPTY"],
                "Labels": { "com.example.service": "api" }
            },
            "HostConfig": {
                "RestartPolicy": { "Name": "always" }
            },
            "Mounts": [
                {
                    "Source": "/host",
                    "Destination": "/data",
                    "Mode": "z",
                    "RW": true,
                    "Type": "bind"
                }
            ],
            "NetworkSettings": {
                "Networks": {
                    "bridge": {
                        "IPAddress": "172.17.0.2",
                        "Gateway": "172.17.0.1",
                        "MacAddress": "02:42:ac:11:00:02"
                    }
                },
                "Ports": {
                    "80/tcp": [{ "HostIp": "0.0.0.0", "HostPort": "8080" }],
                    "443/tcp": null
                }
            }
        }),
        Some(ContainerStatsSnapshot {
            cpu_percent: Some(12.5),
            ..Default::default()
        }),
    );

    assert_eq!(detail.id, "abcdef1234567890");
    assert_eq!(detail.short_id, "abcdef123456");
    assert_eq!(detail.name, "demo");
    assert_eq!(detail.image, "nginx:latest");
    assert_eq!(detail.image_id, "sha256:123");
    assert_eq!(detail.health.as_deref(), Some("healthy"));
    assert_eq!(detail.restart_policy.as_deref(), Some("always"));
    assert_eq!(detail.env.len(), 2);
    assert_eq!(detail.labels.len(), 1);
    assert_eq!(detail.mounts.len(), 1);
    assert_eq!(detail.networks.len(), 1);
    assert_eq!(detail.ports.len(), 2);
    assert!(detail.actions.can_open_port);
    assert_eq!(detail.stats.and_then(|stats| stats.cpu_percent), Some(12.5));
}
