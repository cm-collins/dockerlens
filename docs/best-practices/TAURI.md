# Tauri 2.0 — Best Practices

> **Applies to:** `src-tauri/tauri.conf.json` · `src-tauri/capabilities/` · `src-tauri/src/main.rs`
> **Tauri version:** 2.x (audited by Radically Open Security, funded by NLNet/NGI)
> **Last reviewed:** March 2026
> **References:** Tauri v2 Security Docs · Tauri 2.0 Stable Release Notes

---

## Table of Contents

1. [IPC Security](#1-ipc-security)
2. [Capabilities & Permissions](#2-capabilities--permissions)
3. [Command Design](#3-command-design)
4. [Deep Link Security](#4-deep-link-security)
5. [WebView Security](#5-webview-security)
6. [Plugin Security](#6-plugin-security)
7. [Build & Release](#7-build--release)
8. [Development Security](#8-development-security)
9. [Auto-Update Security](#9-auto-update-security)
10. [Security Checklist](#10-security-checklist)

---

## 1. IPC Security

Tauri's IPC bridge is the single point of contact between the React frontend and the Rust backend. Protecting it is the highest priority security concern in a Tauri application.

### Deny by default

Tauri 2.0 uses a deny-by-default permission model. No Rust command is accessible from the frontend unless explicitly allowed in a capabilities file. Never use `"all": true` in capabilities.
```json
// src-tauri/capabilities/main.json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Main window capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open",
    "notification:default",
    "docker:allow-list-containers",
    "docker:allow-start-container",
    "docker:allow-stop-container"
  ]
}
```

### Never expose commands that aren't needed
```rust
// ❌ Don't register all commands blindly
.invoke_handler(tauri::generate_handler![
    list_containers,
    start_container,
    stop_container,
    delete_container,
    some_debug_command, // ❌ Remove debug commands before release
])

// ✅ Register only what the UI needs
.invoke_handler(tauri::generate_handler![
    list_containers,
    start_container,
    stop_container,
    delete_container,
])
```

### Validate all inputs in Rust commands

Never trust data from the JavaScript frontend. Validate and sanitize every input on the Rust side.
```rust
#[tauri::command]
pub async fn start_container(
    id: String,
    docker: State<'_, DockerState>,
) -> Result<(), String> {
    // ✅ Validate container ID format before using it
    if id.is_empty() || id.len() > 64 {
        return Err("Invalid container ID".to_string());
    }
    // Only alphanumeric + hyphens allowed in container IDs
    if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("Container ID contains invalid characters".to_string());
    }

    docker.client
        .start_container(&id, None::<StartContainerOptions<String>>)
        .await
        .map_err(|e| e.to_string())
}
```

---

## 2. Capabilities & Permissions

### Use individual capability files, not one monolithic file
```
src-tauri/capabilities/
├── main.json       ← main window permissions
└── updater.json    ← auto-update specific permissions
```

### Scope filesystem and shell permissions tightly
```json
// ✅ Only allow opening specific URLs — not all URLs
{
  "permissions": [
    {
      "identifier": "shell:allow-open",
      "allow": [
        { "url": "https://**" },
        { "url": "http://localhost:*" }
      ]
    }
  ]
}
```
```json
// ❌ Never allow arbitrary shell execution
{
  "permissions": [
    "shell:allow-execute"  // Too broad — grants shell access
  ]
}
```

### Minimum required permissions for DockerLens

| Feature | Required Permission |
|---|---|
| Docker socket connection | None — handled entirely in Rust |
| Open browser (port links) | `shell:allow-open` scoped to `http://localhost:*` |
| System tray | `tray-icon:default` |
| Desktop notifications | `notification:default` |
| Auto-updater | `updater:default` |
| Deep link (OAuth) | `deep-link:default` |
| Config file read/write | `fs:allow-read` + `fs:allow-write` scoped to `$APPCONFIG` |

---

## 3. Command Design

### Keep commands thin — logic belongs in modules
```rust
// ❌ Fat command — hard to test, hard to maintain
#[tauri::command]
pub async fn list_containers(docker: State<'_, DockerState>) -> Result<Vec<Container>, String> {
    let opts = ListContainersOptions::<String> { all: true, ..Default::default() };
    let containers = docker.client.list_containers(Some(opts)).await.map_err(|e| e.to_string())?;
    let filtered: Vec<_> = containers.into_iter().filter(|c| c.state.is_some()).collect();
    // ... 50 more lines of business logic
}

// ✅ Thin command — delegates to module
#[tauri::command]
pub async fn list_containers(docker: State<'_, DockerState>) -> Result<Vec<Container>, String> {
    containers::list_all(&docker.client)
        .await
        .map_err(|e| e.to_string())
}
```

### Use Tauri's managed state, never globals
```rust
// ✅ Register state with Tauri's state manager
tauri::Builder::default()
    .manage(DockerState::connect(socket_path).expect("Docker connection failed"))
    .invoke_handler(tauri::generate_handler![list_containers])

// ❌ Never use global static state
static DOCKER_CLIENT: Option<Docker> = None; // thread-unsafe, testable by nothing
```

---

## 4. Deep Link Security

DockerLens registers `dockerlens://` as a custom URI scheme for Supabase OAuth callbacks.

### Register the scheme in `tauri.conf.json`
```json
{
  "app": {
    "security": {
      "deepLinkProtocols": ["dockerlens"]
    }
  }
}
```

### Validate the deep link URL before processing
```rust
// src-tauri/src/main.rs
app.listen("deep-link://new-url", |event| {
    if let Some(url) = event.payload() {
        // ✅ Validate it starts with our scheme before emitting to frontend
        if url.starts_with("dockerlens://auth/callback") {
            app.emit("auth-callback", url).ok();
        } else {
            log::warn!("Rejected unexpected deep link: {}", url);
        }
    }
});
```

### Never process deep links from untrusted schemes

The deep link handler must reject anything that doesn't match `dockerlens://auth/callback`. Log and discard all other payloads.

---

## 5. WebView Security

### Content Security Policy

Define a strict CSP in `tauri.conf.json` to prevent XSS and code injection:
```json
{
  "app": {
    "security": {
      "csp": "default-src 'self'; connect-src 'self' https://*.supabase.co; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com"
    }
  }
}
```

### Never use `dangerouslySetInnerHTML` with user data

In the React frontend, log lines and container names come from Docker and must never be injected raw into the DOM.
```tsx
// ❌ XSS risk — never do this with Docker output
<div dangerouslySetInnerHTML={{ __html: logLine }} />

// ✅ React escapes text content automatically
<div>{logLine}</div>

// ✅ For xterm.js — it handles its own sanitisation
terminal.write(logLine);
```

### Disable devtools in production builds
```json
// tauri.conf.json
{
  "app": {
    "windows": [{
      "devtools": false
    }]
  }
}
```

The Tauri CLI automatically disables devtools in `tauri build` — never override this.

---

## 6. Plugin Security

### Pin plugin versions to patch updates only
```toml
# Cargo.toml
[dependencies]
tauri-plugin-shell = "~2.0.0"        # ~ = patch updates only
tauri-plugin-notification = "~2.0.0"
tauri-plugin-updater = "~2.0.0"
tauri-plugin-deep-link = "~2.0.0"
```

### Keep Tauri and all plugins updated

Security advisories are published at `https://github.com/tauri-apps/tauri/security/advisories`.
```bash
# Check for updates
cargo update
cargo audit

# Subscribe to Tauri security advisories on GitHub
# Settings → Watch → Security alerts
```

---

## 7. Build & Release

### Always build with `pnpm tauri build` — never debug binaries for distribution
```bash
# ✅ Production build — optimised, devtools disabled, CSP enforced
pnpm tauri build

# ❌ Never distribute debug builds
pnpm tauri dev  # development only
```

### Sign release artifacts with GPG
```bash
# Generate a signing key (do once)
gpg --gen-key

# Sign each artifact
gpg --detach-sign --armor DockerLens.AppImage
gpg --detach-sign --armor dockerlens_amd64.deb
gpg --detach-sign --armor dockerlens.x86_64.rpm

# Publish the .asc signature files alongside artifacts on GitHub Releases
```

### Generate SHA256 checksums
```bash
sha256sum DockerLens.AppImage dockerlens_amd64.deb dockerlens.x86_64.rpm > checksums.txt
```

### Never commit secrets to the repository
```bash
# ✅ Use environment variables for secrets in CI
# GitHub Actions → Settings → Secrets

# ❌ Never hardcode in tauri.conf.json or Rust source
const SUPABASE_KEY: &str = "eyJhbGci..."; // NEVER
```

---

## 8. Development Security

### Never develop on untrusted networks

The Tauri dev server (`pnpm tauri dev`) does not use mutual authentication or TLS encryption. Never run it on public Wi-Fi or untrusted networks.

### Keep development secrets separate from production
```bash
# .env — development only, never committed
VITE_SUPABASE_URL=https://your-dev-project.supabase.co
VITE_SUPABASE_ANON_KEY=your-dev-anon-key
```
```bash
# CI/CD — production values stored as GitHub Secrets
# Never in code, never in .env.production committed to git
```

### Verify your supply chain
```bash
# cargo-vet — verify crates with audits from trusted organizations
cargo install cargo-vet
cargo vet init
cargo vet
```

---

## 9. Auto-Update Security

### Use the Tauri updater with signature verification
```json
// tauri.conf.json
{
  "plugins": {
    "updater": {
      "active": true,
      "pubkey": "your-public-key-here",
      "endpoints": [
        "https://github.com/yourusername/dockerlens/releases/latest/download/latest.json"
      ]
    }
  }
}
```

Generate an update signing key:
```bash
cargo tauri signer generate -w ~/.tauri/dockerlens.key
# Add the public key to tauri.conf.json
# Store the private key securely — NEVER commit it
```

The updater verifies signatures before applying any update. A compromised GitHub Release cannot deliver malicious updates without the private signing key.

---

## 10. Security Checklist

Run before every release:

| Check | How |
|---|---|
| All capabilities use minimum required permissions | Review `src-tauri/capabilities/*.json` |
| No `"all": true` in any capability | `grep -r '"all": true' src-tauri/capabilities/` |
| All command inputs validated in Rust | Code review |
| CSP is defined and restricts sources | Check `tauri.conf.json` |
| Deep link handler validates URL before emitting | Code review |
| No secrets in source code | `git grep "supabase.co\|eyJhbG"` |
| Devtools disabled in production | Automatic with `tauri build` |
| Update signing key is not committed | `git log --all -- *.key` |
| Artifacts are GPG signed | CI pipeline |
| SHA256 checksums published | CI pipeline |