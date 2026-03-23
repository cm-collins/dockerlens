//! Unit tests for container data transformation logic.

use dockerlens_lib::docker::containers::{ContainerSummary, PortBinding, validate_container_id};

// ========== Phase 1 Tests: Data Serialization ==========

#[test]
fn container_summary_serializes_to_json() {
    let summary = ContainerSummary {
        id: "abc123def456".to_string(),
        name: "test-container".to_string(),
        image: "nginx:latest".to_string(),
        status: "Up 1 hour".to_string(),
        state: "running".to_string(),
        ports: vec![],
        created: 1234567890,
    };

    let json = serde_json::to_string(&summary);
    assert!(json.is_ok(), "ContainerSummary should serialize to JSON");
    
    let json_str = json.unwrap();
    assert!(json_str.contains("abc123def456"));
    assert!(json_str.contains("test-container"));
}

#[test]
fn port_binding_serializes_to_json() {
    let binding = PortBinding {
        host_port: "8080".to_string(),
        container_port: "80".to_string(),
        protocol: "tcp".to_string(),
    };

    let json = serde_json::to_string(&binding);
    assert!(json.is_ok(), "PortBinding should serialize to JSON");
}

#[test]
fn container_summary_has_correct_fields() {
    let summary = ContainerSummary {
        id: "abc123".to_string(),
        name: "test".to_string(),
        image: "nginx".to_string(),
        status: "Up".to_string(),
        state: "running".to_string(),
        ports: vec![],
        created: 0,
    };

    assert_eq!(summary.id, "abc123");
    assert_eq!(summary.name, "test");
    assert_eq!(summary.image, "nginx");
    assert_eq!(summary.state, "running");
}

#[test]
fn port_binding_has_correct_fields() {
    let binding = PortBinding {
        host_port: "3000".to_string(),
        container_port: "3000".to_string(),
        protocol: "tcp".to_string(),
    };

    assert_eq!(binding.host_port, "3000");
    assert_eq!(binding.container_port, "3000");
    assert_eq!(binding.protocol, "tcp");
}

#[test]
fn container_summary_with_multiple_ports() {
    let ports = vec![
        PortBinding {
            host_port: "8080".to_string(),
            container_port: "80".to_string(),
            protocol: "tcp".to_string(),
        },
        PortBinding {
            host_port: "8443".to_string(),
            container_port: "443".to_string(),
            protocol: "tcp".to_string(),
        },
    ];

    let summary = ContainerSummary {
        id: "abc123".to_string(),
        name: "web".to_string(),
        image: "nginx".to_string(),
        status: "Up".to_string(),
        state: "running".to_string(),
        ports,
        created: 0,
    };

    assert_eq!(summary.ports.len(), 2);
    assert_eq!(summary.ports[0].host_port, "8080");
    assert_eq!(summary.ports[1].host_port, "8443");
}

#[test]
fn container_summary_with_empty_ports() {
    let summary = ContainerSummary {
        id: "abc123".to_string(),
        name: "test".to_string(),
        image: "alpine".to_string(),
        status: "Up".to_string(),
        state: "running".to_string(),
        ports: vec![],
        created: 0,
    };

    assert!(summary.ports.is_empty());
}

#[test]
fn container_summary_debug_format() {
    let summary = ContainerSummary {
        id: "abc123".to_string(),
        name: "test".to_string(),
        image: "nginx".to_string(),
        status: "Up".to_string(),
        state: "running".to_string(),
        ports: vec![],
        created: 0,
    };

    let debug_str = format!("{:?}", summary);
    assert!(debug_str.contains("abc123"));
    assert!(debug_str.contains("test"));
}

// ========== Phase 2 Tests: Input Validation ==========

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
    assert_eq!(result.unwrap_err(), "Container ID cannot be empty");
}

#[test]
fn validate_container_id_rejects_too_long() {
    let long_id = "a".repeat(129);
    let result = validate_container_id(&long_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Container ID exceeds maximum length");
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

// ========== Phase 2 Tests: Port Binding Edge Cases ==========

#[test]
fn port_binding_with_udp_protocol() {
    let binding = PortBinding {
        host_port: "5353".to_string(),
        container_port: "53".to_string(),
        protocol: "udp".to_string(),
    };
    
    assert_eq!(binding.protocol, "udp");
}

#[test]
fn port_binding_with_empty_host_port() {
    let binding = PortBinding {
        host_port: "".to_string(),
        container_port: "80".to_string(),
        protocol: "tcp".to_string(),
    };
    
    assert_eq!(binding.host_port, "");
}

// ========== Phase 2 Tests: Container Summary Edge Cases ==========

#[test]
fn container_summary_with_exited_state() {
    let summary = ContainerSummary {
        id: "abc123".to_string(),
        name: "stopped".to_string(),
        image: "alpine".to_string(),
        status: "Exited (0) 2 days ago".to_string(),
        state: "exited".to_string(),
        ports: vec![],
        created: 1234567890,
    };
    
    assert_eq!(summary.state, "exited");
    assert!(summary.status.contains("Exited"));
}

#[test]
fn container_summary_with_paused_state() {
    let summary = ContainerSummary {
        id: "abc123".to_string(),
        name: "paused".to_string(),
        image: "nginx".to_string(),
        status: "Up 1 hour (Paused)".to_string(),
        state: "paused".to_string(),
        ports: vec![],
        created: 1234567890,
    };
    
    assert_eq!(summary.state, "paused");
}
