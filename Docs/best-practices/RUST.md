# Rust — Best Practices

> **Applies to:** `src-tauri/src/` — all Rust backend code
> **Rust version:** 1.75+
> **Last reviewed:** March 2026
> **References:** RustSec Advisory DB · Corgea Security Guide 2025 · Microsoft Security Response Center

---

## Table of Contents

1. [Memory Safety](#1-memory-safety)
2. [Error Handling](#2-error-handling)
3. [Unsafe Code](#3-unsafe-code)
4. [Dependency Management](#4-dependency-management)
5. [Concurrency with Tokio](#5-concurrency-with-tokio)
6. [Performance](#6-performance)
7. [Code Quality](#7-code-quality)
8. [Testing](#8-testing)
9. [Logging](#9-logging)
10. [Security Checklist](#10-security-checklist)

---

## 1. Memory Safety

Rust eliminates entire classes of memory bugs at compile time. Never fight the borrow checker — learn what it is trying to tell you.

### Ownership rules — never violate these
```rust
// ✅ Good — clear ownership, no cloning unless necessary
fn process(data: &[u8]) -> Result<String, Error> {
    // borrow data, don't take ownership
}

// ❌ Bad — unnecessary clone wastes memory
fn process(data: Vec<u8>) -> Result<String, Error> {
    let _copy = data.clone(); // pointless
}
```

### Avoid unnecessary clones

Clone only when ownership is truly required. Prefer borrowing (`&T`) over owned values (`T`) in function signatures wherever possible.
```rust
// ✅ Borrow for read-only access
fn log_container_name(name: &str) { }

// ✅ Take ownership only when storing or transforming
fn store_name(name: String) { self.name = name; }
```

### Use `Arc` and `Mutex` carefully
```rust
use std::sync::{Arc, Mutex};

// ✅ Wrap shared mutable state in Arc<Mutex<T>>
let shared_state: Arc<Mutex<DockerState>> = Arc::new(Mutex::new(state));

// ✅ Lock as briefly as possible — drop guard before await
{
    let guard = shared_state.lock().unwrap();
    // use guard here
} // guard dropped here — lock released before any .await
```

### Prefer `Arc<RwLock<T>>` for read-heavy state
```rust
use tokio::sync::RwLock;

// ✅ Multiple concurrent readers, exclusive writer
let state = Arc::new(RwLock::new(DockerState::new()));

// Readers (concurrent)
let read_guard = state.read().await;

// Writer (exclusive)
let mut write_guard = state.write().await;
```

---

## 2. Error Handling

### Never use `unwrap()` or `expect()` in production paths
```rust
// ❌ Bad — panics on error, crashes the app
let client = Docker::connect_with_unix("/var/run/docker.sock", 120, API_DEFAULT_VERSION).unwrap();

// ✅ Good — propagate errors to the caller
let client = Docker::connect_with_unix("/var/run/docker.sock", 120, API_DEFAULT_VERSION)
    .map_err(|e| format!("Failed to connect to Docker: {e}"))?;
```

`unwrap()` is acceptable only in:
- Test code
- `main()` during startup where failure is truly unrecoverable
- Cases where the value provably cannot be `None`/`Err` — add a comment explaining why

### Use `anyhow` for application errors, `thiserror` for library errors
```rust
// In library/module code — use thiserror for typed errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Container {0} not found")]
    NotFound(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(#[from] bollard::errors::Error),
}

// In command handlers — use anyhow for flexible error propagation
use anyhow::{Context, Result};

pub async fn start_container(id: &str) -> Result<()> {
    docker.start_container(id, None)
        .await
        .context("Failed to start container")?;
    Ok(())
}
```

### All `#[tauri::command]` functions must return `Result<T, String>`
```rust
// ✅ Tauri commands must return Result<T, String> — the error becomes a JS rejection
#[tauri::command]
pub async fn list_containers(docker: State<'_, DockerState>) -> Result<Vec<Container>, String> {
    docker.client
        .list_containers(Some(ListContainersOptions { all: true, ..Default::default() }))
        .await
        .map_err(|e| e.to_string())
}
```

---

## 3. Unsafe Code

### Minimize `unsafe` blocks — every one is a security boundary
```rust
// ❌ Never use unsafe without documented justification
unsafe {
    // Why is this unsafe? What invariants are being maintained?
}

// ✅ Always document unsafe code
/// SAFETY: The pointer is guaranteed non-null because it comes from
/// a Box::into_raw() call on the same thread, and no other code
/// can access it between these two calls.
unsafe {
    let _ = Box::from_raw(ptr);
}
```

### Rules for `unsafe` in DockerLens

- No `unsafe` in `docker/` or `system/` modules — these deal with user data
- Any `unsafe` block requires a code review from a second contributor
- Run `cargo geiger` in CI to track unsafe usage across all dependencies
```bash
cargo install cargo-geiger
cargo geiger --all-features
```

---

## 4. Dependency Management

### Audit dependencies regularly
```bash
# Install cargo-audit
cargo install cargo-audit

# Run on every CI build
cargo audit

# Fix automatically where possible
cargo audit fix
```

### Use `cargo-deny` for supply chain security
```bash
cargo install cargo-deny
cargo deny check
```

Create a `deny.toml` at the repo root:
```toml
[advisories]
db-path = "~/.cargo/advisory-db"
db-urls = ["https://github.com/rustsec/advisory-db"]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unicode-DFS-2016"]
deny = ["GPL-2.0", "GPL-3.0", "AGPL-3.0"]

[bans]
multiple-versions = "warn"
```

### Pin versions in `Cargo.toml`
```toml
# ✅ Pin to minor version — allows patch updates only
bollard = "0.17"
tokio = { version = "1", features = ["full"] }
tauri = { version = "2", features = [] }

# ❌ Never use wildcard versions in production
some-crate = "*"
```

### `Cargo.lock` must be committed

`Cargo.lock` is committed for binary applications (not libraries). This ensures reproducible builds across all machines and CI runners.

---

## 5. Concurrency with Tokio

### Never block the async runtime
```rust
// ❌ Bad — blocking call inside async context, starves the runtime
#[tauri::command]
pub async fn read_config() -> Result<String, String> {
    std::fs::read_to_string("config.toml") // BLOCKING
        .map_err(|e| e.to_string())
}

// ✅ Good — use tokio's async file I/O
#[tauri::command]
pub async fn read_config() -> Result<String, String> {
    tokio::fs::read_to_string("config.toml")
        .await
        .map_err(|e| e.to_string())
}

// ✅ For unavoidable blocking work — use spawn_blocking
pub async fn heavy_computation() -> Result<String, String> {
    tokio::task::spawn_blocking(|| {
        // blocking work here
        expensive_sync_operation()
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}
```

### Cancel streams properly

All streaming tasks (log tails, stats streams) must be cancellable. Use a `CancellationToken`:
```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();
let child_token = token.clone();

tokio::spawn(async move {
    tokio::select! {
        _ = child_token.cancelled() => {
            // stream cancelled — clean up
        }
        _ = stream_logs(container_id) => {
            // stream ended naturally
        }
    }
});

// To cancel from another task:
token.cancel();
```

### Avoid holding locks across `.await` points
```rust
// ❌ Deadlock risk — lock held across await
async fn bad_example(state: Arc<Mutex<State>>) {
    let guard = state.lock().unwrap();
    some_async_call().await; // lock held during await — deadlock risk
    drop(guard);
}

// ✅ Safe — clone the data, release the lock before awaiting
async fn good_example(state: Arc<Mutex<State>>) {
    let data = {
        let guard = state.lock().unwrap();
        guard.data.clone()
    }; // lock released here
    process_async(data).await;
}
```

---

## 6. Performance

### Enable overflow checks in release builds
```toml
# Cargo.toml
[profile.release]
overflow-checks = true   # prevent silent integer overflow in production
lto = true               # link-time optimisation — reduces binary size
codegen-units = 1        # slower compile, better runtime performance
strip = true             # strip debug symbols from release binary
```

### Use `checked_` arithmetic for security-sensitive calculations
```rust
// ❌ Bad — can overflow silently
let total = a + b;

// ✅ Good — explicit overflow handling
let total = a.checked_add(b)
    .ok_or("Integer overflow in calculation")?;
```

### Avoid unnecessary allocations in hot paths
```rust
// ❌ Allocates a new String on every call
fn format_id(id: &str) -> String {
    format!("container-{}", id)
}

// ✅ Return a reference where possible
fn container_prefix() -> &'static str {
    "container-"
}
```

---

## 7. Code Quality

### `clippy` must pass with zero warnings
```bash
# Run with all warnings as errors
cargo clippy --all-targets --all-features -- -D warnings
```

Add to CI:
```yaml
- name: Clippy
  run: cargo clippy --all-targets --all-features -- -D warnings
```

### `rustfmt` must pass
```bash
cargo fmt --all -- --check
```

### Document all public functions
```rust
/// Lists all containers, including stopped and paused ones.
///
/// # Arguments
/// * `docker` - The Tauri-managed Docker client state
///
/// # Returns
/// A vector of container summaries, or an error string for Tauri
///
/// # Errors
/// Returns an error if the Docker socket is not accessible
#[tauri::command]
pub async fn list_containers(
    docker: State<'_, DockerState>,
) -> Result<Vec<ContainerSummary>, String> {
    // ...
}
```

---

## 8. Testing

### Unit test every Docker module function
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_socket_detection_finds_standard_path() {
        // arrange
        let expected = "/var/run/docker.sock";
        // act
        let result = detect_socket_path();
        // assert — only runs if Docker is available in CI
        if std::path::Path::new(expected).exists() {
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn test_detect_distro_ubuntu() {
        // mock /etc/os-release content
        let content = "ID=ubuntu\nVERSION_ID=\"22.04\"";
        let distro = parse_distro_id(content);
        assert_eq!(distro, Some("ubuntu".to_string()));
    }

    #[test]
    fn test_install_command_for_known_distros() {
        assert!(install_command("ubuntu").contains("apt"));
        assert!(install_command("fedora").contains("dnf"));
        assert!(install_command("arch").contains("pacman"));
    }
}
```

### Integration tests in `tests/` directory
```
src-tauri/
└── tests/
    ├── docker_integration.rs   ← requires live Docker socket
    └── config_tests.rs         ← config read/write tests
```

Mark integration tests so they're skipped in CI without Docker:
```rust
#[cfg(feature = "integration")]
#[tokio::test]
async fn test_list_containers_live() {
    // ...
}
```

---

## 9. Logging

### Use structured logging — never `println!` in production
```toml
# Cargo.toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```
```rust
use log::{debug, error, info, warn};

// ✅ Structured log with context
info!("Docker socket connected at {}", socket_path);
warn!("Daemon not running — retrying in {}s", retry_delay);
error!("Failed to start container {}: {}", id, err);
debug!("Container inspect response: {:?}", response);
```

Log levels in production:
- `error` — unexpected failures requiring user action
- `warn` — recoverable issues (daemon reconnect, socket not found on first try)
- `info` — normal lifecycle events (app start, daemon connect, user actions)
- `debug` — detailed state for development only (disabled in release)

---

## 10. Security Checklist

Run before every release:
```bash
# 1. Audit dependencies
cargo audit

# 2. Check for unsafe code
cargo geiger --all-features

# 3. Lint with all warnings as errors
cargo clippy --all-targets --all-features -- -D warnings

# 4. Check formatting
cargo fmt --all -- --check

# 5. Run all tests
cargo test --all-features

# 6. Check for denied licenses / banned crates
cargo deny check

# 7. Build in release mode and verify binary size
cargo tauri build
ls -lh src-tauri/target/release/bundle/
```

| Check | Command | Must pass |
|---|---|---|
| Dependency vulnerabilities | `cargo audit` | ✅ Zero critical/high |
| Unsafe code inventory | `cargo geiger` | ✅ Review any additions |
| Linter | `cargo clippy -- -D warnings` | ✅ Zero warnings |
| Formatting | `cargo fmt -- --check` | ✅ Clean |
| Tests | `cargo test` | ✅ All pass |
| License compliance | `cargo deny check` | ✅ Clean |