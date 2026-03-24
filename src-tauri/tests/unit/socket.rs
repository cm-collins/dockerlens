//! Unit tests for socket detection logic.

use dockerlens_lib::system::socket;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

#[test]
fn detect_returns_existing_socket_or_none() {
    if let Some(path) = socket::detect() {
        let metadata = std::fs::metadata(&path).expect("detected path should be readable");
        assert!(
            metadata.file_type().is_socket(),
            "Detected path must be a Unix socket"
        );
    }
}

#[test]
fn candidate_paths_prioritize_docker_host_override() {
    let paths = socket::candidate_paths(
        Some("unix:///tmp/docker-from-env.sock"),
        Some("/run/user/1000"),
        Some(Path::new("/home/tester")),
    );

    assert_eq!(
        paths.first(),
        Some(&PathBuf::from("/tmp/docker-from-env.sock"))
    );
}

#[test]
fn candidate_paths_include_rootless_and_desktop_locations() {
    let paths = socket::candidate_paths(
        None,
        Some("/run/user/1000"),
        Some(Path::new("/home/tester")),
    );

    assert!(paths.contains(&PathBuf::from("/run/user/1000/docker.sock")));
    assert!(paths.contains(&PathBuf::from("/home/tester/.docker/run/docker.sock")));
    assert!(paths.contains(&PathBuf::from("/home/tester/.docker/desktop/docker.sock")));
}

#[test]
fn candidate_paths_include_standard_linux_locations() {
    let paths = socket::candidate_paths(None, None, None);

    assert!(paths.contains(&PathBuf::from("/run/docker.sock")));
    assert!(paths.contains(&PathBuf::from("/var/run/docker.sock")));
}

#[test]
fn candidate_paths_ignore_non_unix_docker_host_values() {
    let tcp_paths = socket::candidate_paths(Some("tcp://127.0.0.1:2375"), None, None);
    let http_paths = socket::candidate_paths(Some("http://localhost:2375"), None, None);

    assert_eq!(tcp_paths.first(), Some(&PathBuf::from("/run/docker.sock")));
    assert_eq!(http_paths.first(), Some(&PathBuf::from("/run/docker.sock")));
}

#[test]
fn candidate_paths_do_not_duplicate_entries() {
    let paths = socket::candidate_paths(
        Some("unix:///var/run/docker.sock"),
        None,
        Some(Path::new("/home/tester")),
    );

    let unique_count = paths
        .iter()
        .filter(|path| path.as_path() == Path::new("/var/run/docker.sock"))
        .count();

    assert_eq!(
        unique_count, 1,
        "Socket candidates should stay de-duplicated"
    );
}
