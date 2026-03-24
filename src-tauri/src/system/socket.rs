use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

/// Finds the Docker socket path using a priority-ordered waterfall.
/// Returns None if no readable Unix socket is found.
pub fn detect() -> Option<PathBuf> {
    candidate_paths(
        std::env::var("DOCKER_HOST").ok().as_deref(),
        std::env::var("XDG_RUNTIME_DIR").ok().as_deref(),
        dirs::home_dir().as_deref(),
    )
    .into_iter()
    .find(|path| is_unix_socket(path))
}

#[doc(hidden)]
pub fn candidate_paths(
    docker_host: Option<&str>,
    xdg_runtime_dir: Option<&str>,
    home_dir: Option<&Path>,
) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(path) = parse_docker_host(docker_host) {
        push_unique(&mut paths, path);
    }

    if let Some(runtime_dir) = xdg_runtime_dir {
        push_unique(&mut paths, PathBuf::from(runtime_dir).join("docker.sock"));
    }

    if let Some(home) = home_dir {
        // Rootless Docker fallback used by Docker's own installer.
        push_unique(&mut paths, home.join(".docker/run/docker.sock"));
        // Docker Desktop for Linux uses a user-scoped socket beneath ~/.docker.
        push_unique(&mut paths, home.join(".docker/desktop/docker.sock"));
    }

    // Some Linux packaging choices expose Docker from /run rather than /var/run.
    push_unique(&mut paths, PathBuf::from("/run/docker.sock"));
    push_unique(&mut paths, PathBuf::from("/var/run/docker.sock"));

    paths
}

fn parse_docker_host(docker_host: Option<&str>) -> Option<PathBuf> {
    let host = docker_host?;
    let raw_path = host.strip_prefix("unix://")?;
    Some(PathBuf::from(raw_path))
}

fn push_unique(paths: &mut Vec<PathBuf>, candidate: PathBuf) {
    if !paths.iter().any(|existing| existing == &candidate) {
        paths.push(candidate);
    }
}

fn is_unix_socket(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|metadata| metadata.file_type().is_socket())
        .unwrap_or(false)
}
