# Tests

This directory contains all tests for DockerLens Phase 1 implementation.

## Test Organization

Tests are organized into two directories:

```
tests/
├── unit/              # Unit tests (no Docker required)
│   ├── main.rs
│   ├── socket.rs      # Socket detection logic
│   ├── client.rs      # DockerClient creation and cloning
│   └── containers.rs  # Data transformation and serialization
│
├── integration/       # Integration tests (requires Docker)
│   ├── main.rs
│   ├── socket_detection.rs  # Real filesystem socket detection
│   ├── docker_client.rs     # Real Docker API calls
│   └── commands.rs          # Tauri command handlers
│
└── README.md
```

### Unit Tests (`tests/unit/`)
Test individual functions and data transformations without requiring Docker to be running.

- **`socket.rs`** - Socket detection logic (env vars, rootless paths, standard paths)
- **`client.rs`** - DockerClient creation, cloning, and Arc sharing
- **`containers.rs`** - Container data transformation and serialization

### Integration Tests (`tests/integration/`)
Test the full flow from API calls to Docker Engine. These require Docker to be running but gracefully skip if unavailable.

- **`socket_detection.rs`** - Socket detection against real filesystem
- **`docker_client.rs`** - Real Docker API calls (list containers, all states)
- **`commands.rs`** - Tauri command handlers with real Docker client

## Running Tests

**From project root:**
```bash
# Run all tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run only unit tests (fast, no Docker required)
cargo test --manifest-path src-tauri/Cargo.toml --test unit

# Run only integration tests (requires Docker)
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

# Run all tests
cargo test

# Run only unit tests (fast, no Docker required)
cargo test --test unit

# Run only integration tests (requires Docker)
cargo test --test integration

# Run with output
cargo test -- --nocapture

# Run specific test file
cargo test --test unit socket
cargo test --test integration commands
```

## Test Coverage

**Phase 1 Coverage:**
- ✅ Socket detection (standard, rootless, env var)
- ✅ Docker client initialization
- ✅ Container listing and data transformation
- ✅ Port binding conversion
- ✅ JSON serialization
- ✅ Error handling for missing Docker

**Total:** 18 unit tests + 7 integration tests = 25 tests
