# Tests

This directory contains all tests for DockerLens Phase 1 and Phase 2 implementation.

## Test Organization

Tests are organized into two directories:

```
tests/
├── unit/              # Unit tests (no Docker required)
│   ├── main.rs
│   ├── socket.rs      # Socket detection logic (8 tests)
│   ├── client.rs      # DockerClient creation and cloning (4 tests)
│   └── containers.rs  # Data transformation, serialization, validation (23 tests)
│
├── integration/       # Integration tests (requires Docker)
│   ├── main.rs
│   ├── socket_detection.rs  # Real filesystem socket detection (2 tests)
│   ├── docker_client.rs     # Real Docker API calls (2 tests)
│   └── commands.rs          # Tauri command handlers (14 tests)
│
└── README.md
```

### Unit Tests (`tests/unit/`)
Test individual functions and data transformations without requiring Docker to be running.

- **`socket.rs`** — Socket detection logic (env vars, rootless paths, standard paths) — 8 tests
- **`client.rs`** — DockerClient creation, cloning, and Arc sharing — 4 tests
- **`containers.rs`** — Container data transformation, serialization, and input validation — 23 tests
  - Phase 1: Data serialization (8 tests)
  - Phase 2: Input validation and security (15 tests)

### Integration Tests (`tests/integration/`)
Test the full flow from API calls to Docker Engine. These require Docker to be running but gracefully skip if unavailable.

- **`socket_detection.rs`** — Socket detection against real filesystem — 2 tests
- **`docker_client.rs`** — Real Docker API calls (list containers, all states) — 2 tests
- **`commands.rs`** — Tauri command handlers with real Docker client — 14 tests
  - Phase 1: List containers (3 tests)
  - Phase 2: Container lifecycle commands (11 tests)

## Running Tests

**From project root:**
```bash
# Run all tests (53 tests)
cargo test --manifest-path src-tauri/Cargo.toml

# Run only unit tests (35 tests - fast, no Docker required)
cargo test --manifest-path src-tauri/Cargo.toml --test unit

# Run only integration tests (18 tests - requires Docker)
cargo test --manifest-path src-tauri/Cargo.toml --test integration

# Run with output
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture

# Run specific test file
cargo test --manifest-path src-tauri/Cargo.toml --test unit socket
cargo test --manifest-path src-tauri/Cargo.toml --test integration commands
```

**From `src-tauri/` directory:**
```bash
cd src-tauri

# Run all tests (53 tests)
cargo test

# Run only unit tests (35 tests - fast, no Docker required)
cargo test --test unit

# Run only integration tests (18 tests - requires Docker)
cargo test --test integration

# Run with output
cargo test -- --nocapture

# Run specific test file
cargo test --test unit socket
cargo test --test integration commands

# Run specific test by name
cargo test validate_container_id
cargo test start_container
```

## Test Coverage

**Phase 1 Coverage:**
- ✅ Socket detection (standard, rootless, env var)
- ✅ Docker client initialization
- ✅ Container listing and data transformation
- ✅ Port binding conversion
- ✅ JSON serialization
- ✅ Error handling for missing Docker

**Phase 2 Coverage:**
- ✅ Input validation (container IDs)
- ✅ Security validation (path traversal, command injection, shell metacharacters)
- ✅ Start/stop/restart containers
- ✅ Pause/unpause containers
- ✅ Remove containers (with force and volume options)
- ✅ Inspect containers (full JSON)
- ✅ Get container stats (one-shot)
- ✅ Error handling for invalid inputs
- ✅ Boundary testing (empty IDs, max length, special characters)

**Total:** 35 unit tests + 18 integration tests = **53 tests**

### Test Breakdown by Phase

| Phase | Unit Tests | Integration Tests | Total |
|-------|------------|-------------------|-------|
| Phase 1 | 20 | 7 | 27 |
| Phase 2 | 15 | 11 | 26 |
| **Total** | **35** | **18** | **53** |
