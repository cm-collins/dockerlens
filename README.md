# DockerLens

**A native Linux Docker Desktop GUI — manage containers, images, volumes and networks visually.**  
No VM. No subscription. No terminal required.

[License: MIT](https://opensource.org/licenses/MIT)
[Built with Tauri](https://tauri.app)
[Rust](https://www.rust-lang.org)
[React](https://react.dev)
[Platform](https://kernel.org)

[Download](#-installation) · [Screenshots](#-screenshots) · [Contributing](#-contributing) · [Roadmap](#-roadmap)



---

## 🐳 What is DockerLens?

DockerLens is a **free, open-source, native Linux desktop application** that gives you a beautiful, visual interface to manage your Docker environment — the same polished experience Windows and Mac users get with Docker Desktop, but built specifically for Linux.

It connects directly to your Docker Engine via the Unix socket (`/var/run/docker.sock`). No virtual machine. No background service. No paid subscription. Just install Docker Engine via your package manager, open DockerLens, and start managing your containers visually.

---

## ✨ Why DockerLens?


| Problem with Docker Desktop on Linux             | DockerLens Solution                                |
| ------------------------------------------------ | -------------------------------------------------- |
| Runs a VM even though Linux doesn't need one     | Connects directly to Docker Engine — zero overhead |
| Requires $21/month for team use                  | Free forever — MIT licensed                        |
| Broken system tray on GNOME and other DEs        | Properly implemented for all desktop environments  |
| Conflicts with an existing Docker Engine install | No conflict — uses your existing installation      |
| Doesn't feel native (Wayland / X11 issues)       | Tauri 2.0 — native support for both                |
| `host.docker.internal` doesn't work on Linux     | Auto-injected on every container create            |


---

## 📸 Screenshots

> Screenshots coming with v1.0 beta release.

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
- Optional sign-in with GitHub or Google for settings sync

**System Tray**

- Minimize to tray — app keeps running in the background
- Tray icon reflects daemon state (green = running, red = stopped)
- Container crash notifications
- Quick container count in tray menu

**Suggestions Engine**

- Smart recommendations: stopped containers, unused images, unused volumes, CPU spikes, available updates
- Dismiss individually — resolved suggestions are tracked

### 🔜 Coming Soon


| Feature                                                      | Version |
| ------------------------------------------------------------ | ------- |
| Docker Compose UI — file picker, service graph, up/down/logs | v1.1    |
| Docker Hub & GHCR registry browser                           | v1.2    |
| Multi-host Docker context support                            | v1.2    |
| Image vulnerability scanning (Trivy)                         | v1.3    |
| Kubernetes cluster management                                | v2.0    |
| Extensions marketplace                                       | v2.0    |


---

## 🐧 Supported Distributions


| Distribution         | Package Format    | Docker Install          |
| -------------------- | ----------------- | ----------------------- |
| Ubuntu 22.04 / 24.04 | `.deb` · AppImage | `apt install docker.io` |
| Debian 12            | `.deb` · AppImage | `apt install docker.io` |
| Fedora 39 / 40       | `.rpm` · AppImage | `dnf install docker-ce` |
| Arch Linux / Manjaro | AUR · AppImage    | `pacman -S docker`      |
| Linux Mint 21        | `.deb` · AppImage | `apt install docker.io` |
| Pop!_OS              | `.deb` · AppImage | `apt install docker.io` |
| openSUSE Tumbleweed  | `.rpm` · AppImage | `zypper install docker` |


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


| Layer             | Technology               | Purpose                                     |
| ----------------- | ------------------------ | ------------------------------------------- |
| **App framework** | Tauri 2.0                | Native window, OS APIs, IPC bridge          |
| **Backend**       | Rust + Tokio             | Async Docker API client, system integration |
| **Docker client** | `bollard` crate          | Pure Rust Docker Engine HTTP client         |
| **Frontend**      | React 18 + TypeScript    | All visual components                       |
| **Styling**       | Tailwind CSS + shadcn/ui | Design system                               |
| **State**         | Zustand                  | Lightweight global store                    |
| **Terminal**      | xterm.js                 | Exec into containers                        |
| **Charts**        | Recharts                 | CPU / RAM / network graphs                  |
| **Compose**       | serde_yaml               | YAML parsing                                |
| **Packaging**     | Tauri bundler            | AppImage · .deb · .rpm                      |


---

## 🧑‍💻 Development

### Prerequisites

- [Rust](https://rustup.rs) 1.75+
- Node.js 20 LTS (via [fnm](https://github.com/Schniz/fnm) recommended)
- [pnpm](https://pnpm.io) 8+
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
git clone https://github.com/yourusername/dockerlens.git
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

```
dockerlens/
├── src/                        # React + TypeScript frontend
│   ├── components/             # UI components
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
├── package.json
├── vite.config.ts
└── tailwind.config.ts
```

---

## 🤝 Contributing

Contributions are welcome! DockerLens is open-source and community-driven.

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Commit with conventional commits: `git commit -m "feat: add volume pruning"`
4. Push and open a pull request

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR.

### Good first issues

Look for issues tagged `[good first issue](https://github.com/yourusername/dockerlens/issues?q=label%3A%22good+first+issue%22)` — these are well-scoped tasks suitable for new contributors.

---

## 📋 Roadmap

See the full phased roadmap in the [project Notion workspace](https://www.notion.so/327412df037681d7a661ed8d478ad6e2) or the [GitHub Projects board](https://github.com/yourusername/dockerlens/projects).

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

