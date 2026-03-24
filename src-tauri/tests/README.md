# Backend Tests

This directory contains the Rust backend test suite for DockerLens.

It covers:
- pure unit tests that do not require Docker
- integration tests that run against the host Docker setup when available
- Linux socket-detection behavior for rootful, rootless, and user-scoped Docker installs

## Layout

```text
tests/
├── unit/
│   ├── main.rs
│   ├── client.rs
│   ├── containers.rs
│   └── socket.rs
├── integration/
│   ├── main.rs
│   ├── commands.rs
│   ├── docker_client.rs
│   └── socket_detection.rs
└── README.md
```

## Current Test Inventory

### Unit tests

- `client.rs` — 4 tests
  - Docker client creation
  - clone behavior
  - inner client access
  - independence across cloned handles
- `socket.rs` — 6 tests
  - `DOCKER_HOST` Unix socket precedence
  - rootless socket candidates
  - Docker Desktop Linux socket candidate
  - standard Linux socket candidates
  - non-Unix `DOCKER_HOST` rejection
  - de-duplication of candidate paths
- `containers.rs` — 31 tests
  - container DTO serialization
  - port binding formatting
  - ID validation and security checks
  - action-capability mapping
  - browser URL derivation
  - stats snapshot transformation
  - typed detail extraction from inspect JSON
  - Docker error humanization for bind mounts, permissions, missing containers, and daemon availability

### Integration tests

- `socket_detection.rs` — 2 tests
  - real filesystem socket detection
  - `DOCKER_HOST` behavior on a live host
- `docker_client.rs` — 2 tests
  - list containers when Docker is running
  - include stopped containers in the real client flow
- `commands.rs` — 18 tests
  - typed list response
  - running/search filters
  - overview summary
  - typed detail payload
  - inspect payload
  - one-shot stats snapshot
  - invalid-ID handling for lifecycle commands
  - bulk action per-item result behavior

## Totals

- Unit tests: 41
- Integration tests: 22
- Total backend tests: 63

## Running The Suite

From the repo root:

```bash
# All backend tests
cargo test --manifest-path src-tauri/Cargo.toml

# Unit tests only
cargo test --manifest-path src-tauri/Cargo.toml --test unit

# Integration tests only
cargo test --manifest-path src-tauri/Cargo.toml --test integration

# Lint + compile quality gate
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings

# Optional verbose output
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture
```

From `src-tauri/`:

```bash
cargo test
cargo test --test unit
cargo test --test integration
cargo clippy --all-targets --all-features -- -D warnings
```

## What Is Covered Well

- Docker client creation and shared-state behavior
- Linux Docker socket detection for:
  - `DOCKER_HOST=unix://...`
  - rootless `XDG_RUNTIME_DIR/docker.sock`
  - `~/.docker/run/docker.sock`
  - `~/.docker/desktop/docker.sock`
  - `/run/docker.sock`
  - `/var/run/docker.sock`
- typed container list/detail/overview/stats transformations
- lifecycle input validation and invalid-ID failures
- bulk action result semantics
- search and running-state filtering

## Current Gaps

The backend suite is strong, but it is not exhaustive yet.

These areas still need more coverage if we want to say the backend is fully proven:

- successful lifecycle command paths against disposable fixture containers
  - start
  - stop
  - restart
  - pause
  - unpause
  - remove
- Docker daemon unavailable and permission-denied cases with stronger assertions
- distro matrix validation across Ubuntu, Debian, Fedora, Arch, and Docker Desktop Linux
- rootless Docker integration tests on a real host, not just candidate-path unit tests
- behavior against remote/TCP Docker hosts if we ever decide to support them

## Recommended Backend Verification Gate

Before merging backend changes, this is the expected minimum:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
```
