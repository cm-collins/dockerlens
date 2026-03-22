use thiserror::Error;

/// Single error type for all DockerLens operations.
/// Converted to String at the command boundary for Tauri's IPC.
#[derive(Debug, Error)]
pub enum DockerLensError {
    #[error("Docker socket not found. Is Docker Engine installed and running?")]
    SocketNotFound,

    #[error("Docker connection failed: {0}")]
    Connection(#[from] bollard::errors::Error),

    #[error("Docker API error: {0}")]
    Api(String),
}

/// Converts our error into a String so Tauri commands can return Result<T, String>.
impl From<DockerLensError> for String {
    fn from(e: DockerLensError) -> Self {
        e.to_string()
    }
}