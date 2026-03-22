//! Unit tests for DockerClient.

use dockerlens_lib::docker::client::DockerClient;

#[test]
fn connect_succeeds_with_valid_socket_path() {
    let result = DockerClient::connect("/var/run/docker.sock");
    assert!(result.is_ok(), "connect should succeed with valid socket path");
}

#[test]
fn client_is_cloneable() {
    let client = DockerClient::connect("/var/run/docker.sock")
        .expect("Failed to create client");
    
    let cloned = client.clone();
    
    // Both should point to the same Arc
    assert!(
        std::ptr::eq(client.inner(), cloned.inner()),
        "Cloned client should share the same inner Docker instance"
    );
}

#[test]
fn inner_returns_docker_reference() {
    let client = DockerClient::connect("/var/run/docker.sock")
        .expect("Failed to create client");
    
    let _inner = client.inner();
    assert!(true, "inner() should return a valid Docker reference");
}

#[test]
fn multiple_clients_are_independent() {
    let client1 = DockerClient::connect("/var/run/docker.sock")
        .expect("Failed to create client1");
    
    let client2 = DockerClient::connect("/var/run/docker.sock")
        .expect("Failed to create client2");
    
    // Different Arc instances
    assert!(
        !std::ptr::eq(client1.inner(), client2.inner()),
        "Different clients should have different Arc instances"
    );
}
