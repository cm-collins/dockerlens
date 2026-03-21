# DockerLens — Architecture

> **Version:** 1.0.0
> **Last Updated:** March 2026
> **TRD Reference:** `docs/requirements/TRD.md`
> **PRD Reference:** `docs/requirements/PRD.md`

---

## Table of Contents

1. [Overview](#1-overview)
2. [High-Level System Architecture](#2-high-level-system-architecture)
3. [Application Layers](#3-application-layers)
4. [IPC Communication Model](#4-ipc-communication-model)
5. [Rust Backend — Module Map](#5-rust-backend--module-map)
6. [Frontend — Component Tree](#6-frontend--component-tree)
7. [Docker API Integration](#7-docker-api-integration)
8. [Authentication — Supabase Flow](#8-authentication--supabase-flow)
9. [Real-Time Streaming Pipeline](#9-real-time-streaming-pipeline)
10. [Daemon Control Flow](#10-daemon-control-flow)
11. [Socket Auto-Detection](#11-socket-auto-detection)
12. [Cross-Distro Packaging Pipeline](#12-cross-distro-packaging-pipeline)
13. [User Flows](#13-user-flows)
14. [Screen Map](#14-screen-map)
15. [Visual Exports](#15-visual-exports)

---

## 1. Overview

DockerLens is a **native Linux desktop application** built with Tauri 2.0. It wraps a React + TypeScript frontend inside a native OS window, backed by a Rust core that communicates with Docker Engine via the Unix socket.

There are three distinct layers:
```
┌──────────────────────────────────────────────────┐
│  React + TypeScript  (WebView — what users see)  │
├──────────────────────────────────────────────────┤
│  Tauri IPC Bridge  (invoke / emit)               │
├──────────────────────────────────────────────────┤
│  Rust Core  (Docker client, daemon, tray, auth)  │
└──────────────────────────────────────────────────┘
                        ↕
              /var/run/docker.sock
                        ↕
              Docker Engine (dockerd)
```

The Rust backend **never** communicates with the internet directly. All network communication (Supabase auth, image pulls) is initiated by the React frontend via `supabase-js` and Docker API calls proxied through Rust respectively.

---

## 2. High-Level System Architecture
```mermaid
graph TB
    subgraph DESKTOP["🖥️ Linux Desktop — User Machine"]
        subgraph APP["DockerLens — Tauri 2.0 Binary (~8MB)"]
            subgraph FRONTEND["⚛️ React + TypeScript — WebView"]
                UI["UI Components\nTailwind CSS · shadcn/ui"]
                STATE["Zustand\nGlobal State"]
                QUERY["TanStack Query\nDocker Data Fetching"]
                SUPA_JS["supabase-js\nAuth + Settings Sync"]
            end

            IPC["🔗 Tauri IPC Bridge\ninvoke() · emit()\nType-safe · Zero network exposure"]

            subgraph RUST["🦀 Rust Backend — Tokio Async"]
                BOLLARD["bollard\nDocker API Client"]
                DAEMON["daemon.rs\nStart · Stop · Restart · Enable"]
                SOCKET["socket.rs\nAuto-Detection"]
                TRAY["tray.rs\nSystem Tray"]
                NOTIF["notifications.rs\nDesktop Notifications"]
                CONFIG["config.rs\nTOML Preferences"]
                STREAMS["streams.rs\nLog + Stats Streaming"]
                EXEC["exec.rs\nWebSocket TTY"]
                UPDATER["updater.rs\nAuto-Update"]
            end
        end

        SYSTRAY["🖱️ OS System Tray"]
        AUTOSTART["🔁 systemd User Service"]
    end

    subgraph DOCKER["🐳 Docker Engine — User Installed"]
        SOCK["🔌 Unix Socket\n/var/run/docker.sock"]
        DOCKERD["dockerd\nDaemon Process"]
        CONTAINERS["Containers"]
        IMAGES["Images"]
        VOLUMES["Volumes"]
        NETWORKS["Networks"]
    end

    subgraph CLOUD["☁️ External — Optional"]
        SUPABASE["Supabase\nAuth + Postgres\nSettings Sync"]
        GH_OAUTH["GitHub OAuth"]
        GO_OAUTH["Google OAuth"]
        GH_RELEASES["GitHub Releases\nAuto-Updater Feed"]
    end

    FRONTEND <-->|"invoke() / emit()"| IPC
    IPC <-->|"Rust command handlers"| RUST
    SOCKET -->|"detect path"| SOCK
    BOLLARD <-->|"HTTP over Unix Socket\nDocker REST API v1.41+"| SOCK
    SOCK --- DOCKERD
    DOCKERD --- CONTAINERS & IMAGES & VOLUMES & NETWORKS
    STREAMS -->|"Tauri emit() events"| FRONTEND
    TRAY --- SYSTRAY
    AUTOSTART -.->|"launches on login"| APP
    SUPA_JS <-->|"HTTPS — optional\nauth + preferences only"| SUPABASE
    SUPABASE <--> GH_OAUTH & GO_OAUTH
    UPDATER <-->|"version check"| GH_RELEASES
```

---

## 3. Application Layers
```mermaid
graph LR
    subgraph L1["Layer 1 — UI (WebView)"]
        direction TB
        COMPONENTS["React Components\nContainers · Images · Volumes\nNetworks · Settings · Onboarding"]
        STYLING["Tailwind CSS\nshadcn/ui components"]
        STATE2["Zustand Stores\napp · containers · auth"]
        QUERY2["TanStack Query\nDocker data cache"]
    end

    subgraph L2["Layer 2 — IPC Bridge"]
        direction TB
        INVOKE["invoke()\nReact → Rust calls"]
        EMIT["emit() / listen()\nRust → React events"]
    end

    subgraph L3["Layer 3 — Rust Core"]
        direction TB
        DOCKER_MOD["docker/\ncontainers · images\nvolumes · networks\nexec · streams"]
        SYSTEM_MOD["system/\nsocket · daemon · tray\nnotifications · config"]
    end

    subgraph L4["Layer 4 — OS / External"]
        direction TB
        UNIX["Unix Socket\ndockerd"]
        SYSTEMD["systemd\ndaemon control"]
        POLKIT["Polkit\nprivilege escalation"]
        SUPA2["Supabase\noptional"]
    end

    L1 <-->|"type-safe IPC"| L2
    L2 <-->|"#[tauri::command]"| L3
    L3 <-->|"system calls"| L4
```

---

## 4. IPC Communication Model

All communication between React and Rust flows through Tauri's IPC bridge. There is no HTTP server, no WebSocket exposed externally, and no shared memory between layers.
```mermaid
sequenceDiagram
    participant REACT as ⚛️ React Component
    participant STORE as 🗃️ Zustand Store
    participant INVOKE as 🔗 invoke()
    participant RUST as 🦀 Rust Handler
    participant BOLLARD as bollard
    participant DOCKER as 🐳 dockerd

    Note over REACT,DOCKER: Request / Response — e.g. list containers

    REACT->>INVOKE: invoke("list_containers")
    INVOKE->>RUST: #[tauri::command] fires
    RUST->>BOLLARD: list_containers(opts).await
    BOLLARD->>DOCKER: GET /containers/json
    DOCKER-->>BOLLARD: JSON array
    BOLLARD-->>RUST: Vec<ContainerSummary>
    RUST-->>INVOKE: serialized JSON
    INVOKE-->>REACT: ContainerSummary[]
    REACT->>STORE: dispatch updateContainers()

    Note over REACT,DOCKER: Real-time push — e.g. log streaming

    RUST->>INVOKE: app.emit("log_line", payload)
    INVOKE-->>REACT: listen("log_line") fires
    REACT->>REACT: xterm.js writes line
```

---

## 5. Rust Backend — Module Map
```mermaid
graph LR
    subgraph ENTRY["Entry Point"]
        MAIN["main.rs\nTauri bootstrap +\nplugin registration"]
        COMMANDS["commands.rs\nAll #[tauri::command]\nexports in one place"]
    end

    subgraph DOCKER_MOD["docker/ module"]
        CLIENT["client.rs\nbollard singleton\nmanaged Tauri state"]
        CONT["containers.rs\nlist · start · stop\nrestart · delete"]
        IMG["images.rs\nlist · pull · delete"]
        VOL["volumes.rs\nlist · create · delete"]
        NET["networks.rs\nlist · create · delete"]
        EXEC2["exec.rs\nWebSocket TTY\nexec session"]
        STR["streams.rs\nlog tail\nstats stream\nevent stream"]
    end

    subgraph SYS_MOD["system/ module"]
        SOCK2["socket.rs\nauto-detect path\ndistro detection"]
        DAEM["daemon.rs\nstart · stop · restart\nenable · disable"]
        TRAY2["tray.rs\nicon state\nright-click menu"]
        NOT2["notifications.rs\ncrash alerts\ndaemon state"]
        CONF["config.rs\nTOML read/write\n~/.config/dockerlens/"]
        UPD["updater.rs\nGitHub Releases\nversion check"]
    end

    MAIN --> COMMANDS
    COMMANDS --> CONT & IMG & VOL & NET & EXEC2 & DAEM & CONF & UPD
    CONT & IMG & VOL & NET & EXEC2 --> CLIENT
    STR --> CLIENT
    CLIENT --> SOCK2
    STR --> COMMANDS
    TRAY2 & NOT2 --> COMMANDS
    DAEM --> SOCK2
```

---

## 6. Frontend — Component Tree
```mermaid
graph TD
    APP["App.tsx\nSupabase provider\nZustand provider\nOnboarding gate"]

    APP -->|"first run"| OB["OnboardingWizard\n4 steps"]
    APP -->|"after onboarding"| SHELL["AppShell"]

    OB --> OB1["WelcomeStep"]
    OB --> OB2["AuthStep\nGitHub · Google\nvia Supabase"]
    OB --> OB3["SystemCheckStep\nDocker detection"]
    OB --> OB4["ReadyStep\nResource summary"]

    SHELL --> TITLE["TitleBar\nTraffic lights · title"]
    SHELL --> BODY["Body — flex row"]

    BODY --> SIDEBAR["Sidebar\nNav · Daemon pill\nUser pill · Settings"]
    BODY --> MAIN2["Main Area"]

    MAIN2 --> TOPBAR["TopBar\nTitle · subtitle\nAction buttons"]
    MAIN2 --> PAGES["Pages Outlet"]

    PAGES --> DASH["DashboardPage\nStat cards · memory bars\nContainer list · Suggestions"]
    PAGES --> CONT2["ContainersPage"]
    PAGES --> IMG2["ImagesPage\nTable · PullModal\nRunWizard"]
    PAGES --> VOL2["VolumesPage\nStat cards · VolumeCards"]
    PAGES --> NET2["NetworksPage\nStat cards · NetworkCards"]
    PAGES --> SET["SettingsPage\nAccount · Daemon\nAppearance · Connection\nDanger Zone"]
    PAGES --> REC["RecommendationsPage\nActive · Resolved"]

    CONT2 --> CLIST["ContainerList\nSearch · filter\nStatus badges"]
    CONT2 --> CDETAIL["ContainerDetail\nSide panel"]

    CDETAIL --> T1["Overview Tab\nPorts · Env · Mounts · Labels"]
    CDETAIL --> T2["Logs Tab\nxterm.js · live stream"]
    CDETAIL --> T3["Terminal Tab\nWebSocket exec shell"]
    CDETAIL --> T4["Stats Tab\nRecharts graphs"]
    CDETAIL --> T5["Inspect Tab\nRaw JSON"]
```

---

## 7. Docker API Integration
```mermaid
graph LR
    subgraph UI_ACTIONS["User Actions"]
        A1["View containers"]
        A2["Start / Stop / Delete"]
        A3["Tail logs"]
        A4["Exec into shell"]
        A5["View stats"]
        A6["Pull image"]
        A7["Run container"]
        A8["Manage volumes"]
        A9["Manage networks"]
    end

    subgraph API_CALLS["Docker REST API — via bollard"]
        B1["GET /containers/json?all=true"]
        B2["POST /containers/id/start\nPOST /containers/id/stop\nDELETE /containers/id"]
        B3["GET /containers/id/logs\n?follow=true&stdout=true"]
        B4["POST /containers/id/exec\nPOST /exec/id/start (WS)"]
        B5["GET /containers/id/stats\n?stream=true"]
        B6["POST /images/create\n?fromImage=name&tag=tag"]
        B7["POST /containers/create\nPOST /containers/id/start"]
        B8["GET /volumes\nPOST /volumes/create\nDELETE /volumes/name"]
        B9["GET /networks\nPOST /networks/create\nDELETE /networks/id"]
    end

    A1 --> B1
    A2 --> B2
    A3 --> B3
    A4 --> B4
    A5 --> B5
    A6 --> B6
    A7 --> B7
    A8 --> B8
    A9 --> B9
```

---

## 8. Authentication — Supabase Flow
```mermaid
flowchart TD
    A["User clicks\nContinue with GitHub / Google"] --> B["supabase-js\nsignInWithOAuth()\nPKCE flow"]
    B --> C["Tauri shell opens\nOAuth URL in system browser"]
    C --> D["User grants permission\non GitHub or Google"]
    D --> E["Provider redirects\nto Supabase callback"]
    E --> F["Supabase exchanges code\nfor JWT tokens"]
    F --> G["Supabase redirects to\ndockerlens://auth/callback\nTauri deep link"]
    G --> H["Tauri onOpenUrl handler\nfires in main.rs"]
    H --> I["URL emitted to React\nvia Tauri event"]
    I --> J["supabase-js\nexchangeCodeForSession()"]
    J --> K["JWT stored in localStorage\nSession active"]
    K --> L["auth.store.ts updated\nUser visible in sidebar"]
    L --> M["Preferences loaded\nfrom Supabase Postgres"]

    style A fill:#1E3A6E,color:#E8EDF8
    style K fill:#0D3D28,color:#22D47A
    style M fill:#0D3D28,color:#22D47A
```

---

## 9. Real-Time Streaming Pipeline
```mermaid
flowchart LR
    subgraph SOURCES["Stream Sources"]
        S1["dockerd stdout/stderr\nContainer logs"]
        S2["GET /stats?stream=true\nCPU · RAM · Net I/O"]
        S3["GET /events\nGlobal Docker events"]
    end

    subgraph RUST_PIPELINE["Rust — streams.rs"]
        TOKIO_TASK["tokio::spawn\nasync task per stream"]
        DECODE["Decode chunk\nto UTF-8 string"]
        EMIT2["app.emit()\nTauri event emitter"]
    end

    subgraph REACT_PIPELINE["React — UI Layer"]
        LISTEN["listen()\nTauri event listener"]
        XTERM2["xterm.js\nterminal.write()"]
        RECHARTS2["Recharts\nsetState → re-render"]
        TRAY3["Tray icon update\ndaemon state change"]
    end

    S1 -->|"bollard\nStream<LogOutput>"| TOKIO_TASK
    S2 -->|"bollard\nStream<Stats>"| TOKIO_TASK
    S3 -->|"bollard\nStream<EventMessage>"| TOKIO_TASK

    TOKIO_TASK --> DECODE --> EMIT2

    EMIT2 -->|"log_line event"| LISTEN
    EMIT2 -->|"stats_update event"| LISTEN
    EMIT2 -->|"daemon_state event"| LISTEN

    LISTEN -->|"log lines"| XTERM2
    LISTEN -->|"stats data"| RECHARTS2
    LISTEN -->|"daemon events"| TRAY3

    note1["⚡ End-to-end latency < 50ms"]
```

---

## 10. Daemon Control Flow
```mermaid
flowchart TD
    CLICK["User clicks\nStart / Stop / Restart\nin Settings or Tray"]

    CLICK --> INVOKE2["invoke('start_docker_daemon')\nor stop / restart"]
    INVOKE2 --> RUST_DAEMON["daemon.rs\ndetect_docker_mode()"]

    RUST_DAEMON --> CHECK{"Socket path\ncheck"}
    CHECK -->|"/var/run/docker.sock\nRoot Docker"| POLKIT2["pkexec systemctl\nstart docker\nNative OS password dialog"]
    CHECK -->|"~/.docker/run/docker.sock\nRootless Docker"| ROOTLESS["systemctl --user\nstart docker\nNo password needed"]

    POLKIT2 -->|"authenticated"| SUCCESS["Daemon starts\napp.emit('daemon_state',\n{ running: true })"]
    POLKIT2 -->|"cancelled"| FAIL["app.emit('daemon_state_error',\n{ reason: 'cancelled' })"]
    ROOTLESS --> SUCCESS

    SUCCESS --> RECONNECT["bollard auto-reconnect\nSocket ping every 5s"]
    RECONNECT --> RELOAD["Dashboard reloads\nTray icon → 🟢\nAll streams resume"]

    FAIL --> BANNER["UI shows error banner\nUser can retry"]
```

---

## 11. Socket Auto-Detection
```mermaid
flowchart TD
    START["App starts\nsocket.rs::detect()"]

    START --> C1{"Check\n/var/run/docker.sock"}
    C1 -->|"exists + readable"| CONNECT
    C1 -->|"not found"| C2{"Check\n~/.docker/run/docker.sock\nRootless Docker"}
    C2 -->|"exists"| CONNECT
    C2 -->|"not found"| C3{"Check\n/run/user/UID/docker.sock"}
    C3 -->|"exists"| CONNECT
    C3 -->|"not found"| C4{"DOCKER_HOST\nenv var set?"}
    C4 -->|"yes"| CONNECT
    C4 -->|"no"| WIZARD["First-run wizard\nDistro detection\n/etc/os-release"]

    WIZARD --> DISTRO{"Distro?"}
    DISTRO -->|"Ubuntu / Debian"| I1["apt install docker.io"]
    DISTRO -->|"Fedora / RHEL"| I2["dnf install docker-ce"]
    DISTRO -->|"Arch / Manjaro"| I3["pacman -S docker"]
    DISTRO -->|"openSUSE"| I4["zypper install docker"]

    CONNECT["Socket found\nbollard::connect_with_unix()"] --> PERM{"User has\nr/w permission?"}
    PERM -->|"no"| FIXPERM["Show group fix:\nsudo usermod -aG docker $USER"]
    PERM -->|"yes"| DAEMON_CHECK{"Daemon\nrunning?"}
    DAEMON_CHECK -->|"no"| START_BTN["Show Start Docker button\ndaemon.rs control"]
    DAEMON_CHECK -->|"yes"| DASH["✅ Load Dashboard\nStream all resources"]
```

---

## 12. Cross-Distro Packaging Pipeline
```mermaid
flowchart LR
    DEV["git push\ntag: v1.x.x"] --> GHA["GitHub Actions\nCI triggered"]

    GHA --> LINT2["cargo clippy\npnpm lint\npnpm vitest\nAll must pass"]

    LINT2 --> MATRIX["Build Matrix\n3 parallel jobs"]

    MATRIX --> U22["ubuntu-22.04 runner"]
    MATRIX --> FCON["Fedora 40\nDocker container"]
    MATRIX --> ACON["Arch Linux\nDocker container"]

    U22 --> APPIMG2["📦 .AppImage\nUniversal glibc 2.31+"]
    U22 --> DEB2["📦 .deb\nUbuntu · Debian · Mint"]
    U22 --> FLAT2["📦 Flatpak\nAll distros sandboxed"]
    FCON --> RPM2["📦 .rpm\nFedora · RHEL · openSUSE"]
    ACON --> AUR2["📋 PKGBUILD\nArch · Manjaro · EndeavourOS"]

    APPIMG2 & DEB2 & RPM2 --> SIGN2["GPG sign\nSHA256 checksums"]
    SIGN2 --> GHREL["🚀 GitHub Release\nupdate.json\n→ Tauri auto-updater"]
    FLAT2 --> FLATHUB2["🏪 Flathub\nio.dockerlens.DockerLens"]
    AUR2 --> AUR_REPO["📋 AUR\ndockerlens"]

    GHREL -->|"in-app update check"| USER["✅ User's machine\nDockerLens installed"]
    FLATHUB2 --> USER
    AUR_REPO --> USER
```

---

## 13. User Flows

### Flow 1 — First Launch & Onboarding
```mermaid
flowchart TD
    INSTALL["User installs DockerLens\n.AppImage / .deb / .rpm"] --> LAUNCH["App launches"]
    LAUNCH --> S1["Step 1: Welcome\nFeature highlights"]
    S1 --> S2["Step 2: Sign in\nOptional — Supabase Auth"]

    S2 --> AUTH_CHOICE{"User choice"}
    AUTH_CHOICE -->|"GitHub"| GH["Supabase GitHub OAuth\nSystem browser opens"]
    AUTH_CHOICE -->|"Google"| GO["Supabase Google OAuth\nSystem browser opens"]
    AUTH_CHOICE -->|"Skip"| S3

    GH & GO --> DEEP["OAuth completes\nDeep link → app\nJWT stored"]
    DEEP --> S3

    S3["Step 3: System Check\nDocker detection"] --> DOCKER_CHECK{"Docker\ninstalled?"}
    DOCKER_CHECK -->|"No"| INSTALL_GUIDE["Show distro install guide"]
    INSTALL_GUIDE --> DOCKER_CHECK
    DOCKER_CHECK -->|"Yes"| GROUP_CHECK{"In docker\ngroup?"}
    GROUP_CHECK -->|"No"| FIX["Show usermod fix"]
    FIX --> GROUP_CHECK
    GROUP_CHECK -->|"Yes"| DAEMON_CHECK2{"Daemon\nrunning?"}
    DAEMON_CHECK2 -->|"No"| START_DAEMON["Start Docker button"]
    START_DAEMON --> DAEMON_CHECK2
    DAEMON_CHECK2 -->|"Yes"| S4["Step 4: All set!\nResource summary"]
    S4 --> DASHBOARD["🏠 Dashboard\nAll resources load"]
```

---

### Flow 2 — Container Lifecycle
```mermaid
flowchart TD
    CONT_VIEW["Containers View\nGET /containers/json?all=true"] --> LIST["List renders\nStatus badges · search · filter"]
    LIST --> SELECT["User selects container\nDetail panel opens"]

    SELECT --> TABS{"Tab selected"}
    TABS -->|"Overview"| OV["Ports · Env vars\nMounts · Labels · Network"]
    TABS -->|"Logs"| LOGS["GET /logs?follow=true\nxterm.js live stream\nINFO · WARN · ERROR coloured"]
    TABS -->|"Terminal"| TERM["POST /exec\nWebSocket TTY\nInteractive shell"]
    TABS -->|"Stats"| STATS["GET /stats?stream=true\nRecharts CPU · RAM · Net"]
    TABS -->|"Inspect"| INSPECT["GET /json\nFormatted JSON viewer"]

    LIST --> ACTIONS{"Action button"}
    ACTIONS -->|"Start"| START2["POST /containers/id/start"]
    ACTIONS -->|"Stop"| STOP["POST /containers/id/stop"]
    ACTIONS -->|"Restart"| RESTART["POST /containers/id/restart"]
    ACTIONS -->|"Delete"| CONFIRM["Confirm modal"]
    CONFIRM -->|"Confirmed"| DELETE["DELETE /containers/id?force=true"]
    ACTIONS -->|"Open in browser"| BROWSER["xdg-open\nlocalhost:PORT"]
```

---

### Flow 3 — Image Pull & Run
```mermaid
flowchart TD
    IMG_VIEW["Images View\nGET /images/json"] --> TABLE["Table renders\nName · tag · size · created · in-use"]
    TABLE --> PULL_BTN["User clicks Pull Image"]
    PULL_BTN --> MODAL["Pull modal\nInput: name:tag"]
    MODAL --> PULL_API["POST /images/create\n?fromImage=name&tag=tag"]
    PULL_API --> PROGRESS["Layer progress stream\nPulling → Extracting → Complete"]
    PROGRESS --> DONE["✅ Image in list"]

    DONE --> RUN_BTN["User clicks Run"]
    RUN_BTN --> WIZARD["Run Wizard — 5 steps"]
    WIZARD --> W1["Step 1: Container name"]
    W1 --> W2["Step 2: Port mappings\nHOST:CONTAINER"]
    W2 --> W3["Step 3: Env variables\nKEY=VALUE pairs"]
    W3 --> W4["Step 4: Volume mounts\n/host:/container"]
    W4 --> W5["Step 5: Restart policy\nalways · on-failure · no"]
    W5 --> CREATE["POST /containers/create"]
    CREATE --> START3["POST /containers/id/start"]
    START3 --> RUNNING["Container running\nNavigate to Containers view"]
```

---

### Flow 4 — Daemon Control
```mermaid
flowchart TD
    TRAY_CLICK["Tray click\nor Settings screen"] --> STATUS{"Daemon\nstatus"}

    STATUS -->|"Stopped 🔴"| START_CLICK["User clicks\nStart Docker"]
    START_CLICK --> MODE{"Root or\nRootless?"}
    MODE -->|"Root\n/var/run/docker.sock"| POLKIT3["pkexec systemctl start docker\nNative OS password dialog"]
    MODE -->|"Rootless\n~/.docker/run/docker.sock"| USER_SYSTEMCTL["systemctl --user start docker\nNo password needed"]
    POLKIT3 -->|"authenticated"| STARTED["Daemon starts\nemit('daemon_state', running)"]
    POLKIT3 -->|"cancelled"| CANCEL["Error shown\nUser can retry"]
    USER_SYSTEMCTL --> STARTED
    STARTED --> GREEN["Tray icon → 🟢\nDashboard reloads\nAll streams resume"]

    STATUS -->|"Running 🟢"| STOP_CLICK["User clicks\nStop Docker"]
    STOP_CLICK --> CONFIRM2["Polkit confirmation"]
    CONFIRM2 --> STOPPED["systemctl stop docker\nTray icon → 🔴\nReconnect banner shown\nRetry every 5 seconds"]

    STATUS -->|"Running 🟢"| BOOT_TOGGLE["Toggle Start on Login"]
    BOOT_TOGGLE -->|"Enable"| ENABLE["systemctl enable docker"]
    BOOT_TOGGLE -->|"Disable"| DISABLE["systemctl disable docker"]
```

---

### Flow 5 — Suggestions Engine
```mermaid
flowchart TD
    BACKGROUND["Background analysis\non app startup + every 30 min"]

    BACKGROUND --> CHECK1{"Containers stopped\n> 24 hours?"}
    BACKGROUND --> CHECK2{"Dangling image\nlayers detected?"}
    BACKGROUND --> CHECK3{"Volumes with no\ncontainers attached?"}
    BACKGROUND --> CHECK4{"Container CPU\n> threshold?"}
    BACKGROUND --> CHECK5{"Newer image\ntag available?"}

    CHECK1 -->|"yes"| REC1["⚠ node-api is stopped\nStart or remove"]
    CHECK2 -->|"yes"| REC2["💡 Unused image layers\ndocker image prune"]
    CHECK3 -->|"yes"| REC3["✓ Orphaned volume\nRemove to free storage"]
    CHECK4 -->|"yes"| REC4["⚠ CPU spike detected\nView stats"]
    CHECK5 -->|"yes"| REC5["🔄 Updates available\nPull new version"]

    REC1 & REC2 & REC3 & REC4 & REC5 --> PANEL["Suggestions panel\nActive count in sidebar badge"]

    PANEL --> USER_ACTION{"User action"}
    USER_ACTION -->|"Action button"| NAVIGATE["Navigate to relevant screen"]
    USER_ACTION -->|"Dismiss"| RESOLVED["Moved to Resolved section\nBadge count decrements"]
```

---

### Flow 6 — System Tray Background Mode
```mermaid
flowchart TD
    CLOSE["User closes main window"] --> TRAY_MIN["App does NOT quit\nMinimises to system tray"]

    TRAY_MIN --> ICON_STATE["Tray icon state"]
    ICON_STATE --> GREEN2["🟢 Green\nDaemon running"]
    ICON_STATE --> RED["🔴 Red\nDaemon stopped"]

    TRAY_MIN --> POLL["Background polling\nevery 10 seconds\nGET /events stream"]

    POLL --> CHANGE{"State\nchanged?"}
    CHANGE -->|"Container died"| NOTIF2["🔔 Desktop notification\n'nginx crashed unexpectedly'"]
    CHANGE -->|"Daemon stopped"| ICON_RED["Tray icon → 🔴\nBanner on re-open"]
    CHANGE -->|"No change"| POLL

    TRAY_MIN --> MENU["Right-click tray menu"]
    MENU --> M1["Open DockerLens"]
    MENU --> M2["Running: N containers"]
    MENU --> M3["Start All / Stop All"]
    MENU --> M4["Quit"]
```

---

## 14. Screen Map
```mermaid
flowchart TD
    OB_WIZARD["Onboarding Wizard\n4 steps — first run only"]

    OB_WIZARD -->|"complete"| DASHBOARD2["🏠 Dashboard\nStat cards · Memory bars\nContainer list · Suggestions preview"]

    DASHBOARD2 --> CONT_SCREEN["📦 Containers\nList + Detail panel\n5 tabs per container"]
    DASHBOARD2 --> IMG_SCREEN["🖼️ Images\nTable view\nPull · Run wizard"]
    DASHBOARD2 --> VOL_SCREEN["💾 Volumes\nStat cards\nExpandable cards"]
    DASHBOARD2 --> NET_SCREEN["🌐 Networks\nStat cards\nExpandable cards"]
    DASHBOARD2 --> COMP_SCREEN["🎼 Compose\nFile picker · Service preview\nUp · Down · Logs — v1.1"]
    DASHBOARD2 --> REC_SCREEN["💡 Suggestions\nActive · Resolved\nDismiss actions"]
    DASHBOARD2 --> SET_SCREEN["⚙️ Settings\nAccount · Daemon · Appearance\nConnection · Danger Zone"]

    CONT_SCREEN --> OVERVIEW_TAB["Overview tab\nPorts · Env · Mounts · Labels"]
    CONT_SCREEN --> LOGS_TAB["Logs tab\nLive log stream"]
    CONT_SCREEN --> TERM_TAB["Terminal tab\nExec shell"]
    CONT_SCREEN --> STATS_TAB["Stats tab\nGraphs"]
    CONT_SCREEN --> INSPECT_TAB["Inspect tab\nRaw JSON"]

    IMG_SCREEN --> PULL_MODAL["Pull modal\nLayer progress"]
    IMG_SCREEN --> RUN_WIZARD["Run wizard\n5 steps"]

    SET_SCREEN --> DAEMON_CTRL["Daemon control\nStart · Stop · Restart · Boot toggle"]
    SET_SCREEN --> AUTH_SECTION["Account section\nSupabase sign in/out"]
```

---

## 15. Visual Exports

The following PNG files in this folder are exported from FigJam and provide a visual complement to the Mermaid diagrams above. They are intended for contributors who prefer a visual overview before reading code.

| File | Contents | Source |
|---|---|---|
| `system-overview.png` | High-level system architecture — all layers, connections and external services | FigJam — DockerLens Full User Flow board |
| `user-flows.png` | All 6 user flows — onboarding, containers, images, daemon, suggestions, tray | FigJam — DockerLens Full User Flow board |
| `screen-map.png` | Complete screen navigation map — every page and how they connect | FigJam — DockerLens Screen Map board |

> **Note for contributors:** The Mermaid diagrams in this file are the authoritative source of truth. The PNG exports are supplementary and may lag behind by one version during active development. If there is a discrepancy, the Mermaid diagrams are correct.

---

## How This File Relates to the Rest of `docs/`
```
docs/
├── requirements/
│   ├── PRD.md        ← What to build and why
│   └── TRD.md        ← How to build it — full technical spec
└── architecture/
    ├── ARCHITECTURE.md   ← This file — visual overview of the system
    ├── system-overview.png
    ├── user-flows.png
    └── screen-map.png
```

The TRD contains the authoritative written specification. This file focuses on **visual communication** — every major system concept is represented as a Mermaid diagram so a new contributor can understand the full architecture in one read.