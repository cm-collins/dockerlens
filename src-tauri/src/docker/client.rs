use bollard::Docker;
use std::sync::Arc;

/// Thread-safe Docker API client.
/// Wraps bollard and is stored as Tauri managed state — one instance per app lifetime.
#[derive(Clone)]
pub struct DockerClient {
    inner: Arc<Docker>,
}

impl DockerClient {
    /// Connects to Docker at the given Unix socket path.
    /// bollard is lazy — the actual handshake happens on the first API call.
    pub fn connect(socket_path: &str) -> Result<Self, bollard::errors::Error> {
        let client = Docker::connect_with_unix(
            socket_path,
            120, // request timeout in seconds
            bollard::API_DEFAULT_VERSION,
        )?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Returns the inner bollard client for API calls.
    pub fn inner(&self) -> &Docker {
        &self.inner
    }
}