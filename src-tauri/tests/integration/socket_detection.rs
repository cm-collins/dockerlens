//! Integration tests for Docker socket detection.
//! These tests run against the host system — safe to run with or without Docker installed.

use dockerlens_lib::system::socket;
use std::path::PathBuf;

#[test]
fn detect_returns_none_or_existing_path() {
    match socket::detect() {
        Some(path) => {
            assert!(
                path.exists(),
                "detect() returned a path that does not exist: {}",
                path.display()
            );
            println!("✓ Docker socket found at: {}", path.display());
        }
        None => {
            println!("✓ No Docker socket found — returned None as expected");
        }
    }
}

#[test]
fn docker_host_env_var_is_respected() {
    // Set DOCKER_HOST to a nonexistent path — should not be returned (path doesn't exist)
    std::env::set_var("DOCKER_HOST", "unix:///tmp/nonexistent-docker.sock");
    let result = socket::detect();

    // Must not return the nonexistent path
    if let Some(ref path) = result {
        assert_ne!(
            path,
            &PathBuf::from("/tmp/nonexistent-docker.sock"),
            "detect() should not return a path that does not exist"
        );
    }

    std::env::remove_var("DOCKER_HOST");
}