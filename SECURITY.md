# Security Policy

## Supported Versions

dockerlens is currently **pre-release** (no stable releases in this repository yet).

Until releases are published, security fixes will be made on:

| Version / branch | Supported |
| --- | --- |
| `main` | Yes |
| Any fork / older commits | Best effort |

Once releases exist, we will support **the latest stable release** (and may support older versions depending on severity and feasibility).

## Reporting a Vulnerability

Please **do not open a public GitHub issue** for suspected security vulnerabilities.

Instead, report privately using GitHub’s vulnerability reporting:

- Go to the repository’s **Security** tab
- Click **Report a vulnerability**

### What to include

- **Description** of the issue and why it’s a security concern
- **Impact** (what an attacker can do)
- **Affected area** (UI, Rust backend, Tauri bridge, build/release, dependencies)
- **Reproduction steps** or a proof-of-concept (safe and minimal)
- **Environment** (Linux distro/version, Docker Engine version, how dockerlens is installed/run)
- **Logs/screenshots** if helpful (please redact secrets)

### What to expect from us

- **Acknowledgement**: within 7 days
- **Status updates**: at least every 14 days while investigating
- **Fix & disclosure**: for validated issues, we’ll aim to ship a fix as soon as practical and coordinate disclosure timing with you

### Scope

In scope:

- Vulnerabilities in dockerlens itself (UI, backend, IPC/command handling)
- Dependency vulnerabilities that are exploitable in dockerlens
- Supply-chain / build issues that could compromise distributed artifacts (once releases exist)

Out of scope (generally):

- Issues requiring physical access to an unlocked machine
- Vulnerabilities in Docker Engine/daemon itself (please report upstream to Docker)
- Denial-of-service from obviously untrusted local user actions (case-by-case)

### Safe harbor

We welcome good-faith security research. Please avoid privacy violations, destructive testing, or disruption of others’ systems.
