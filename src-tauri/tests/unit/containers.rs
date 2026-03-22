//! Unit tests for container data transformation logic.

use dockerlens_lib::docker::containers::{ContainerSummary, PortBinding};

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
