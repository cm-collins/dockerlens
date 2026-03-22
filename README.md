# DockerLens

**A native Linux Docker Desktop GUI — manage containers, images, volumes and networks visually.**  
No VM. No subscription. No terminal required.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri%202.0-24C8D8?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)
[![Platform](https://img.shields.io/badge/Platform-Linux-FCC624?logo=linux&logoColor=black)](https://kernel.org)

[Download](#-installation) · [Screenshots](#-screenshots) · [Documentation](#-documentation) · [Contributing](#-contributing) · [Roadmap](#-roadmap)

---

## 🐳 What is DockerLens?

DockerLens is a **free, open-source, native Linux desktop application** that gives you a beautiful, visual interface to manage your Docker environment — the same polished experience Windows and Mac users get with Docker Desktop, but built specifically for Linux.

It connects directly to your Docker Engine via the Unix socket (`/var/run/docker.sock`). No virtual machine. No background service. No paid subscription. Just install Docker Engine via your package manager, open DockerLens, and start managing your containers visually.

---

## ✨ Why DockerLens?

| Problem with Docker Desktop on Linux | DockerLens Solution |
|---|---|
| Runs a VM even though Linux doesn't need one | Connects directly to Docker Engine — zero overhead |
| Requires $21/month for team use | Free forever — MIT licensed |
| Broken system tray on GNOME and other DEs | Properly implemented for all desktop environments |
| Conflicts with an existing Docker Engine install | No conflict — uses your existing installation |
| Doesn't feel native (Wayland / X11 issues) | Tauri 2.0 — native support for both |
| `host.docker.internal` doesn't work on Linux | Auto-injected on every container create |

---

## 📸 Screenshots

These images are **UI design mockups** from [`docs/Design/samples/`](docs/Design/samples/). They show the target v1.0 experience; the shipping app will converge on this look as features land.

| Onboarding — welcome | Dashboard — system overview |
|:---:|:---:|
| ![Welcome — first-run onboarding](docs/Design/samples/Screenshot%20From%202026-03-21%2023-19-47.png) | ![Dashboard with resource summary and suggestions](docs/Design/samples/Screenshot%20From%202026-03-21%2023-20-50.png) |

| Containers — overview & stats | Containers — inspect (JSON) |
|:---:|:---:|
| ![Containers list with detail pane — Overview tab](docs/Design/samples/Screenshot%20From%202026-03-21%2023-21-18.png) | ![Containers — Inspect tab with container JSON](docs/Design/samples/Screenshot%20From%202026-03-21%2023-22-03.png) |

| Volumes | Suggestions |
|:---:|:---:|
| ![Volumes list with usage and unused volume](docs/Design/samples/Screenshot%20From%202026-03-21%2023-22-45.png) | ![Suggestions — active recommendations](docs/Design/samples/Screenshot%20From%202026-03-21%2023-23-42.png) |

More samples live in [`docs/Design/samples/`](docs/Design/samples/).

---

## 🚀 Features

### ✅ MVP (v1.0)

**Containers**

- List all containers with live status badges (running 🟢 / stopped 🔴 / paused 🟡)
- Start, stop, restart, pause and delete containers from the UI
- Real-time CPU, memory and network I/O graphs
- Live log streaming with colour-coded `INFO` / `WARN` / `ERROR` lines
- Exec into any container — full terminal session inside the app (xterm.js)
- Container detail view: ports, environment variables, mounts, labels, image

**Images**

- List all local images with size, tags and creation date
- Pull any image with real-time layer-by-layer progress
- Delete unused images
- Run a container from any image with a 5-step wizard (name → ports → env → volumes → restart policy)

**Volumes**

- List all volumes with size, driver and mount path
- See which containers are using each volume
- Create and delete volumes

**Networks**

- List all networks with driver, subnet and gateway info
- See all containers connected to each network
- Create and remove custom networks

**Settings & Daemon Control**

- Start, stop and restart the Docker daemon from the UI — same as Windows/Mac Docker Desktop
- Toggle "Start Docker on Login" (`systemctl enable docker`)
- Native OS privilege escalation via Polkit — no sudo in a terminal
- Rootless Docker supported — no password prompt needed
- Auto-reconnect if the daemon restarts
- Socket path override (supports rootless Docker and custom paths)

**Onboarding**

- First-run wizard with automatic Docker Engine detection
- Distro-specific install instructions (Ubuntu, Fedora, Arch, openSUSE)
- Docker group permission check with guided fix
- Optional sign-in with GitHub or Google via Supabase for settings sync

**System Tray**

- Minimize to tray — app keeps running in the background
- Tray icon reflects daemon state (green = running, red = stopped)
- Container crash notifications
- Quick container count in tray menu

**Suggestions Engine**

- Smart recommendations: stopped containers, unused images, unused volumes, CPU spikes, available updates
- Dismiss individually — resolved suggestions are tracked

### 🔜 Coming Soon

| Feature | Version |
|---|---|
| Docker Compose UI — file picker, service graph, up/down/logs | v1.1 |
| Docker Hub & GHCR registry browser | v1.2 |
| Multi-host Docker context support | v1.2 |
| Image vulnerability scanning (Trivy) | v1.3 |
| Kubernetes cluster management | v2.0 |
| Extensions marketplace | v2.0 |

---

## 🐧 Supported Distributions

| Distribution | Package Format | Docker Install |
|---|---|---|
| Ubuntu 22.04 / 24.04 | `.deb` · AppImage | `apt install docker.io` |
| Debian 12 | `.deb` · AppImage | `apt install docker.io` |
| Fedora 39 / 40 | `.rpm` · AppImage | `dnf install docker-ce` |
| Arch Linux / Manjaro | AUR · AppImage | `pacman -S docker` |
| Linux Mint 21 | `.deb` · AppImage | `apt install docker.io` |
| Pop!_OS | `.deb` · AppImage | `apt install docker.io` |
| openSUSE Tumbleweed | `.rpm` · AppImage | `zypper install docker` |

> **Universal fallback:** The `.AppImage` runs on any Linux distribution with glibc 2.31+ (kernel 5.x+) — no installation required.

---

## 📦 Installation

### AppImage (Universal — any Linux distro)
```bash
chmod +x DockerLens.AppImage
./DockerLens.AppImage
```

### Ubuntu / Debian / Mint / Pop!_OS
```bash
sudo apt install ./dockerlens_amd64.deb
```

### Fedora / RHEL / CentOS / openSUSE
```bash
sudo dnf install ./dockerlens.x86_64.rpm
# or
sudo zypper install ./dockerlens.x86_64.rpm
```

### Arch Linux / Manjaro (AUR)
```bash
yay -S dockerlens
# or
paru -S dockerlens
```

### Flatpak (Flathub)
```bash
flatpak install flathub io.dockerlens.DockerLens
```

---

## 🛠️ Prerequisites

DockerLens is a GUI frontend — it does **not** install or bundle Docker Engine. You need to have Docker Engine installed on your system first.
```bash
# Ubuntu / Debian
sudo apt install docker.io
sudo usermod -aG docker $USER   # add yourself to the docker group
newgrp docker                   # apply group change without re-login

# Fedora
sudo dnf install docker-ce
sudo systemctl start docker
sudo usermod -aG docker $USER

# Arch Linux
sudo pacman -S docker
sudo systemctl enable --now docker
sudo usermod -aG docker $USER
```

> After adding yourself to the `docker` group, log out and back in for the change to take effect. DockerLens will guide you through this automatically on first launch.

---

## 🏗️ Tech Stack

| Layer | Technology | Purpose |
|---|---|---|
| **App framework** | Tauri 2.0 | Native window, OS APIs, IPC bridge |
| **Backend** | Rust + Tokio | Async Docker API client, system integration |
| **Docker client** | `bollard` crate | Pure Rust Docker Engine HTTP client |
| **Frontend** | React 18 + TypeScript | All visual components |
| **Styling** | Tailwind CSS + shadcn/ui | Design system |
| **State** | Zustand | Lightweight global store |
| **Terminal** | xterm.js | Exec into containers |
| **Charts** | Recharts | CPU / RAM / network graphs |
| **Auth** | Supabase | Optional GitHub / Google sign-in + settings sync |
| **Packaging** | Tauri bundler | AppImage · .deb · .rpm |

---

## 🧑‍💻 Development

### Prerequisites

- [Rust](https://rustup.rs) 1.75+
- Node.js 20 LTS (via [fnm](https://github.com/Schniz/fnm) recommended)
- [pnpm](https://pnpm.io) 10+
- Docker Engine installed and running on your Ubuntu host

### Install system dependencies (Ubuntu)
```bash
sudo apt install \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libssl-dev \
  curl \
  wget \
  file
```

### Clone and run
```bash
git clone https://github.com/cm-collins/dockerlens.git
cd dockerlens

pnpm install          # install frontend dependencies
pnpm tauri dev        # starts Vite + compiles Rust + opens native window
```

Hot reload is enabled — React changes reflect instantly. Rust changes trigger a recompile.

### Build for production
```bash
pnpm tauri build
# Output: src-tauri/target/release/bundle/
# → .deb, .rpm, .AppImage
```

### Project structure
Target layout as the app grows (some paths are not created yet):
```
dockerlens/
├── src/                        # React + TypeScript frontend
│   ├── components/             # UI components (add as needed)
│   ├── pages/                  # Screen-level components
│   ├── store/                  # Zustand state slices
│   ├── hooks/                  # Custom React hooks
│   └── lib/                    # Tauri invoke() helpers
│
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs             # Tauri app bootstrap
│   │   ├── commands.rs         # All #[tauri::command] exports
│   │   ├── docker/             # Docker API module
│   │   │   ├── containers.rs
│   │   │   ├── images.rs
│   │   │   ├── volumes.rs
│   │   │   ├── networks.rs
│   │   │   ├── exec.rs         # WebSocket TTY exec
│   │   │   └── streams.rs      # Log tail + stats stream
│   │   └── system/             # OS integration
│   │       ├── socket.rs       # Socket auto-detection
│   │       ├── tray.rs         # System tray
│   │       ├── daemon.rs       # Daemon start/stop/restart
│   │       ├── notifications.rs
│   │       └── config.rs       # TOML config
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── database/                   # Supabase SQL migrations
│   ├── 001_schema.sql          # Tables, constraints, indexes
│   ├── 002_rls.sql             # Row-level security policies
│   ├── 003_functions.sql       # Functions, triggers, automation
│   └── 004_seed_dev.sql        # Dev seed data — never run in production
│
├── docs/                       # Project documentation (see Documentation below)
│   ├── Requirements/           # PRD + TRD
│   ├── Archtecture/            # ARCHITECTURE.md + diagrams (PNG/PDF)
│   ├── Design/                 # Design system, screens, mockups, samples
│   └── best-practices/         # Per-tech contributor guides
│
├── package.json
└── vite.config.ts
```

Add `tailwind.config.ts` and related tooling when you wire up Tailwind (see [`docs/Requirements/TRD.md`](docs/Requirements/TRD.md)).

---

## 📚 Documentation

In-repo docs live under [`docs/`](docs/). Highlights:

| Area | Documents |
|------|-----------|
| **Requirements** | [Product requirements (PRD)](docs/Requirements/PRD.md) · [Technical requirements (TRD)](docs/Requirements/TRD.md) |
| **Architecture** | [Architecture overview](docs/Archtecture/ARCHITECTURE.md) · diagrams: [system overview](docs/Archtecture/DockerLens%20%E2%80%94%20System%20Overview.png), [screen map](docs/Archtecture/DockerLens%20%E2%80%94%20Screen%20Map.png), [user flows](docs/Archtecture/DockerLens%20%E2%80%94%20All%20User%20Flows.png) (PDFs in the same folder) |
| **Design** | [Design system](docs/Design/DESIGN-SYSTEM.md) · [Screen specs](docs/Design/SCREENS.md) · [Mockup notes](docs/Design/MOCKUP.md) · [UI samples](docs/Design/samples/) |
| **Best practices** | [Index](docs/best-practices/README.md) — Tauri, Rust, React/TypeScript, Supabase, GitHub Actions, and general guidelines |
| **Database** | [Migrations guide](database/README.md) — Supabase SQL migrations, execution order and rules |

---

## 🤝 Contributing

Contributions are welcome! DockerLens is open-source and community-driven.

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Commit with conventional commits: `git commit -m "feat: add volume pruning"`
4. Push and open a pull request

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR.

### Good first issues

Look for issues tagged [`good first issue`](https://github.com/cm-collins/dockerlens/issues?q=label%3A%22good+first+issue%22) — these are well-scoped tasks suitable for new contributors.

---

## 📋 Roadmap

Roadmap and execution are tracked in **GitHub**, not Linear or an external PM tool:

| Where | Link / use |
|-------|------------|
| **Project board** | [**dockerlens** (Kanban)](https://github.com/users/cm-collins/projects/1) — primary roadmap view. [All projects](https://github.com/cm-collins?tab=projects) if you use more than one board. |
| **Issues** | [cm-collins/dockerlens/issues](https://github.com/cm-collins/dockerlens/issues) |
| **Pull requests** | [cm-collins/dockerlens/pulls](https://github.com/cm-collins/dockerlens/pulls) |

**GitHub CLI** (optional — needs **`read:project`** once: `gh auth refresh -s read:project`):

```bash
# Print this board’s URL (JSON is {"projects":[...]} — filter with .projects[])
gh project list --owner cm-collins --format json -q '.projects[] | select(.title=="dockerlens") | .url'

# Same via jq: … | jq -r '.projects[] | select(.title=="dockerlens") | .url'

# Open in browser from the shell:
BOARD_URL="$(gh project list --owner cm-collins --format json -q '.projects[] | select(.title=="dockerlens") | .url')"
(command -v xdg-open >/dev/null && xdg-open "$BOARD_URL") || (command -v open >/dev/null && open "$BOARD_URL") || true

gh project list --owner cm-collins          # table listing
gh project list --owner cm-collins --web    # projects UI in browser
```

If `gh project list` errors with missing `read:project`, complete `gh auth refresh -s read:project` (browser/device flow), then retry.

---

## 🔒 Security

Found a security vulnerability? Please **do not** open a public issue. Email `security@dockerlens.io` instead. We will respond within 48 hours.

---

## 📄 License

DockerLens is licensed under the [MIT License](LICENSE).

---

## 🙏 Acknowledgements

- [Tauri](https://tauri.app) — for making native Linux apps with web tech actually possible
- [bollard](https://github.com/fussybeaver/bollard) — the Rust Docker client powering the backend
- [Docker](https://www.docker.com) — for the API that makes all of this possible

---

Built with ❤️ for the Linux community.

**[⭐ Star this repo if DockerLens saves you time](https://github.com/cm-collins/dockerlens)**