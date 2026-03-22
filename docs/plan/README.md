# DockerLens — Implementation Plan

> **Branch convention:** Each phase gets its own feature branch.
> **Merge policy:** Phase N must be merged and `pnpm tauri dev` confirmed working before Phase N+1 begins.
> **Scope:** This folder covers backend (Rust) only. Frontend plan lives in `docs/plan/frontend/`.

---

## Table of Contents

- [Backend Phases](#backend-phases)
- [Full Phase Dependency Map](#full-phase-dependency-map)
- [File Ownership Map](#file-ownership-map)
- [Branch Strategy](#branch-strategy)
- [Definition of Done](#definition-of-done)

---

## Backend Phases

| Phase | Name | Branch | Unlocks |
|---|---|---|---|
| **P1** | Foundation — Socket + Client | `feat/docker-client-foundation` | Everything |
| **P2** | Container Management | `feat/backend-containers` | P3, P4, P6 |
| **P3** | Image Management | `feat/backend-images` | None (parallel with P4) |
| **P4** | Volumes & Networks | `feat/backend-volumes-networks` | None (parallel with P3) |
| **P5** | Daemon Control & System Tray | `feat/backend-daemon-tray` | None (parallel with P3/P4) |
| **P6** | Real-Time Streaming | `feat/backend-streaming` | Frontend logs + stats UI |
| **P7** | Config, Notifications & Updater | `feat/backend-config-notifications` | Settings sync, onboarding |

---

## Full Phase Dependency Map
```mermaid
graph LR
    subgraph FOUNDATION["Phase 1 — Foundation"]
        P1A["socket.rs\nAuto-detection"]
        P1B["client.rs\nbollard singleton"]
        P1C["docker/mod.rs\nModule exports"]
        P1D["commands.rs\nTauri command registry"]
        P1E["lib.rs\nManaged state"]
    end

    subgraph CONTAINERS["Phase 2 — Containers"]
        P2A["containers.rs\nlist · start · stop\nrestart · delete · inspect"]
    end

    subgraph IMAGES["Phase 3 — Images"]
        P3A["images.rs\nlist · pull · delete · tag"]
    end

    subgraph VOLUMES["Phase 4 — Volumes & Networks"]
        P4A["volumes.rs\nlist · create · delete"]
        P4B["networks.rs\nlist · create · delete"]
    end

    subgraph DAEMON["Phase 5 — Daemon & Tray"]
        P5A["daemon.rs\nstart · stop · restart\nenable · disable"]
        P5B["tray.rs\nicon · menu · state"]
    end

    subgraph STREAMING["Phase 6 — Streaming"]
        P6A["streams.rs\nlog tail · stats stream\nevent stream"]
        P6B["exec.rs\nWebSocket TTY"]
    end

    subgraph CONFIG["Phase 7 — Config & Notifications"]
        P7A["config.rs\nTOML read/write"]
        P7B["notifications.rs\ndesktop alerts"]
        P7C["updater.rs\nGitHub Releases"]
    end

    P1A --> P1B
    P1B --> P1C
    P1C --> P1D
    P1D --> P1E

    P1E -->|"required by all"| P2A
    P1E -->|"required by all"| P3A
    P1E -->|"required by all"| P4A
    P1E -->|"required by all"| P4B
    P1E -->|"required by all"| P5A
    P1E -->|"required by all"| P5B

    P2A -->|"container IDs for streaming"| P6A
    P2A -->|"container IDs for exec"| P6B

    P5A -->|"daemon state events"| P5B
    P5A -->|"daemon state events"| P6A

    P7A -->|"config feeds"| P5A
    P7A -->|"config feeds"| P1A
    P5B -->|"crash events trigger"| P7B
    P6A -->|"event stream triggers"| P7B
```

---

## File Ownership Map
```mermaid
graph TD
    subgraph SYSTEM["src-tauri/src/system/"]
        S1["socket.rs"]
        S2["daemon.rs"]
        S3["tray.rs"]
        S4["notifications.rs"]
        S5["config.rs"]
        S6["updater.rs"]
    end

    subgraph DOCKER["src-tauri/src/docker/"]
        D1["mod.rs"]
        D2["client.rs"]
        D3["containers.rs"]
        D4["images.rs"]
        D5["volumes.rs"]
        D6["networks.rs"]
        D7["streams.rs"]
        D8["exec.rs"]
    end

    subgraph ENTRY["src-tauri/src/"]
        E1["main.rs"]
        E2["lib.rs"]
        E3["commands.rs"]
    end

    E1 --> E2
    E2 --> E3
    E2 --> D2
    E2 --> S5
    E3 --> D3 & D4 & D5 & D6 & D7 & D8
    E3 --> S2 & S3 & S4 & S6
    D2 --> S1
    D3 & D4 & D5 & D6 & D7 & D8 --> D1
    D1 --> D2
```

---

## Branch Strategy
```
main
├── feat/docker-client-foundation   ← Phase 1
├── feat/backend-containers         ← Phase 2 (branches from P1 merge)
├── feat/backend-images             ← Phase 3 (branches from P1 merge)
├── feat/backend-volumes-networks   ← Phase 4 (branches from P1 merge)
├── feat/backend-daemon-tray        ← Phase 5 (branches from P1 merge)
├── feat/backend-streaming          ← Phase 6 (branches from P2 merge)
└── feat/backend-config-notifications ← Phase 7 (branches from P5 merge)
```

Phases 2, 3, 4 and 5 can all be worked on in parallel after Phase 1 merges.
Phase 6 requires Phase 2 to be merged first — it needs real container IDs.
Phase 7 requires Phase 5 to be merged first — it needs daemon state.

---

## Definition of Done

A phase is **done** when all of the following are true:

- [ ] All files listed in the phase are implemented — no empty stubs
- [ ] `cargo clippy -- -D warnings` passes with zero warnings
- [ ] `cargo test` passes — all unit tests for the phase pass
- [ ] `cargo audit` passes — no new vulnerabilities
- [ ] `pnpm tauri dev` opens with no Rust compile errors
- [ ] A smoke test confirms the feature works against a real Docker socket
- [ ] PR opened with the phase checklist completed
- [ ] Merged to `main` before next phase begins