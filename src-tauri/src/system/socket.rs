use std::path::{Path, PathBuf};

/// Finds the Docker socket path using a priority-ordered waterfall.
/// Returns None if no readable socket is found.
pub fn detect() -> Option<PathBuf> {
    // 1. Respect explicit user override via environment variable
    if let Some(path) = from_env() {
        return Some(path);
    }

    // 2. Rootless Docker — user-scoped socket paths
    if let Some(path) = rootless_paths().into_iter().find(|p| exists(p)) {
        return Some(path);
    }

    // 3. Standard root Docker socket
    let standard = PathBuf::from("/var/run/docker.sock");
    if exists(&standard) {
        return Some(standard);
    }

    None
}

/// Reads DOCKER_HOST env var and extracts the Unix socket path if set.
fn from_env() -> Option<PathBuf> {
    let host = std::env::var("DOCKER_HOST").ok()?;
    let raw_path = host.strip_prefix("unix://")?;
    let path = PathBuf::from(raw_path);
    exists(&path).then_some(path)
}

/// Returns candidate paths for rootless Docker installations.
fn rootless_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // XDG_RUNTIME_DIR is the standard location for rootless Docker
    if let Ok(dir) = std::env::var("XDG_RUNTIME_DIR") {
        paths.push(PathBuf::from(dir).join("docker.sock"));
    }

    // Home directory fallback used by Docker's rootless installer
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".docker/run/docker.sock"));
    }

    paths
}

fn exists(path: &Path) -> bool {
    path.exists()
}