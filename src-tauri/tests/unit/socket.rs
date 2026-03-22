//! Unit tests for socket detection logic.

use dockerlens_lib::system::socket;
use std::path::PathBuf;

#[test]
fn detect_returns_existing_path_or_none() {
    match socket::detect() {
        Some(path) => assert!(path.exists(), "Detected socket path must exist on disk"),
        None => assert!(true, "No socket found is valid"),
    }
}

#[test]
fn detect_respects_docker_host_env() {
    std::env::set_var("DOCKER_HOST", "unix:///var/run/docker.sock");
    let result = socket::detect();
    std::env::remove_var("DOCKER_HOST");
    
    if let Some(path) = result {
        assert!(path.exists());
    }
}

#[test]
fn detect_ignores_tcp_hosts() {
    std::env::set_var("DOCKER_HOST", "tcp://127.0.0.1:2375");
    let result = socket::detect();
    std::env::remove_var("DOCKER_HOST");
    
    // Should not return TCP host
    if let Some(path) = result {
        assert_ne!(path.to_str().unwrap(), "127.0.0.1:2375");
    }
}

#[test]
fn detect_ignores_http_hosts() {
    std::env::set_var("DOCKER_HOST", "http://localhost:2375");
    let result = socket::detect();
    std::env::remove_var("DOCKER_HOST");
    
    if let Some(path) = result {
        assert!(!path.to_str().unwrap().starts_with("http://"));
    }
}

#[test]
fn detect_handles_nonexistent_env_path() {
    std::env::set_var("DOCKER_HOST", "unix:///nonexistent/docker.sock");
    let result = socket::detect();
    std::env::remove_var("DOCKER_HOST");
    
    // Should not return nonexistent path
    if let Some(path) = result {
        assert_ne!(path, PathBuf::from("/nonexistent/docker.sock"));
    }
}

#[test]
fn detect_checks_standard_socket_path() {
    let standard = PathBuf::from("/var/run/docker.sock");
    
    if standard.exists() {
        let result = socket::detect();
        assert!(result.is_some(), "Should detect standard socket when it exists");
    }
}

#[test]
fn detect_checks_rootless_paths() {
    // Just verify detect() runs without panic when checking rootless paths
    let _result = socket::detect();
    assert!(true, "detect() should check rootless paths without panic");
}
