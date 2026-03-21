# DockerLens — Interactive React Mockup

> **Version:** 2.0.0
> **Last Updated:** March 2026
> **Design System:** `docs/design/DESIGN-SYSTEM.md`
> **Screen Specs:** `docs/design/SCREENS.md`

---

## Overview

`dockerlens-v2.jsx` is a fully interactive React mockup of the DockerLens application. Every screen is implemented — you can click through the entire app, interact with containers, watch logs stream live, sign in with GitHub or Google, dismiss suggestions, and control the Docker daemon — all before a single line of production code is written.

This file is the **visual source of truth** for UI implementation. When building a component, reference this mockup alongside `DESIGN-SYSTEM.md` and `SCREENS.md`.

---

## What's Implemented

| Screen | Implemented | Interactive |
|---|---|---|
| Onboarding — Step 1 Welcome | ✅ | Continue button |
| Onboarding — Step 2 Sign In | ✅ | GitHub + Google buttons with loading state + success |
| Onboarding — Step 3 System Check | ✅ | Run System Check → animated 4-item scan → all green |
| Onboarding — Step 4 Ready | ✅ | Launch DockerLens → enters main app |
| Sidebar | ✅ | All nav items navigate screens |
| Daemon pill | ✅ | Green/red state, click → Settings |
| User pill | ✅ | Appears after auth, shows name + provider |
| Dashboard | ✅ | Stat cards navigate to screen, suggestions preview |
| Containers — List | ✅ | Click any row → detail panel opens |
| Containers — Overview Tab | ✅ | Shows ports, CPU, memory, network, image |
| Containers — Logs Tab | ✅ | Live log lines stream every 2 seconds |
| Containers — Terminal Tab | ✅ | Static exec shell with blinking cursor |
| Containers — Stats Tab | ✅ | CPU + Memory + Network I/O progress bars |
| Containers — Inspect Tab | ✅ | Formatted JSON |
| Images | ✅ | Table with Run + Delete actions |
| Volumes | ✅ | Expandable cards with mount paths and container badges |
| Networks | ✅ | Expandable cards with subnet, gateway, connected containers |
| Settings — Account | ✅ | Sign in/out, shows signed-in user |
| Settings — Daemon Control | ✅ | Start/Stop/Restart with 1.4s simulated delay + feedback |
| Settings — Start on Login | ✅ | Toggle on/off |
| Settings — Appearance | ✅ | Theme pill selection |
| Settings — Connection | ✅ | Socket path input |
| Settings — Danger Zone | ✅ | Buttons rendered |
| Suggestions | ✅ | 5 suggestions, dismiss → resolves, badge updates |

---

## How to Run

### Option 1 — StackBlitz (instant, no install)

1. Go to [stackblitz.com/edit/vitejs-vite-react](https://stackblitz.com/edit/vitejs-vite-react)
2. Replace `src/App.jsx` with the contents of `dockerlens-v2.jsx`
3. The app runs immediately in the browser

### Option 2 — Local development server
```bash
# From the repo root
cp docs/design/mockup/dockerlens-v2.jsx src/App.jsx
pnpm install
pnpm dev
# Opens at http://localhost:5173
```

### Option 3 — Inside the Tauri dev window
```bash
pnpm tauri dev
# The mockup renders inside the native Tauri window
```

---

## Interaction Guide

Walk through the app in this order to see everything:

### 1. Onboarding

- Click `Continue →` on the Welcome screen
- On Sign In: click `Continue with GitHub` — watch the loading state → success screen
- Click `Continue →` to go to System Check
- Click `Run System Check` — watch the 4 items animate green one by one
- Click `Continue →` to see the Ready screen
- Click `Launch DockerLens →` to enter the main app

### 2. Containers

- Click `Containers` in the sidebar
- Click `nginx-proxy` in the list → detail panel opens
- Click through the 5 tabs — **Logs** streams live, **Terminal** shows exec shell
- Click `postgres-db` → notice the Stats tab shows higher memory usage
- Click `node-api` (stopped) → action buttons change to Start only

### 3. Dashboard

- Click `Dashboard` in the sidebar
- Click any stat card — navigates to that screen

### 4. Volumes & Networks

- Click `Volumes` → click any volume card to expand
- Click `Networks` → click any network to see subnet + connected containers

### 5. Suggestions

- Click `Suggestions` in the sidebar
- Click any suggestion's action button → navigates to relevant screen
- Click `Dismiss` on a suggestion → it moves to Resolved, badge count decrements

### 6. Settings — Daemon Control

- Click `Settings`
- Click `⏹ Stop Docker` → watch the simulated `systemctl stop docker` feedback appear
- Watch the sidebar daemon pill turn red
- Click `▶ Start Docker` to restore it

### 7. Auth in Settings

- In Settings → Account section → click `GitHub` or `Google` to simulate sign-in
- User name appears in the sidebar user pill and title bar
- Click `Sign out` to return to unsigned state

---

## File Structure
```
docs/design/
├── DESIGN-SYSTEM.md         ← colour tokens, typography, components
├── SCREENS.md               ← every screen spec and layout
├── MOCKUP.md                ← this file
└── mockup/
    └── dockerlens-v2.jsx    ← 904-line interactive React mockup
```

---

## Design Decisions Captured in the Mockup

| Decision | Implementation |
|---|---|
| Dark theme | `#080B14` background — carefully chosen to avoid eye strain at high contrast |
| DM Sans | Friendly but technical — not the overused Inter or Roboto |
| JetBrains Mono | All code, IDs, socket paths, log lines, inspect JSON |
| Animated status dots | Running containers pulse with green glow — stopped are static red |
| macOS traffic lights | Familiar to developers on any OS — Linux users recognise them too |
| Daemon pill in sidebar | Daemon status always visible — never hidden in a menu |
| Suggestions badge | Yellow to distinguish from the blue container count badge |
| Expandable volume/network rows | Show summary → expand for detail — avoids a split panel for simpler data |
| Optimistic daemon control | Shows `systemctl` command immediately — confirms after 1.4s |

---

## Relationship to Production Code

This mockup is a **design prototype**, not production code. When building the real implementation:

| Mockup | Production |
|---|---|
| Hardcoded `CONTAINERS` array | `invoke('list_containers')` → bollard → Docker Engine |
| Simulated log stream (`setInterval`) | `listen('log_line')` → Rust streams.rs → dockerd |
| Fake daemon control delay | `invoke('start_docker_daemon')` → daemon.rs → pkexec |
| Hardcoded `RECS` suggestions | Rust analysis module checking Docker state |
| Inline CSS styles | Tailwind CSS classes + shadcn/ui components |
| Single `.jsx` file | Split into components per `docs/requirements/TRD.md` structure |

---

## Changelog

| Version | Changes |
|---|---|
| `2.0.0` | Added Volumes, Networks, Suggestions, improved Dashboard, optional Supabase auth in onboarding and settings, user pill in sidebar, Danger Zone in settings |
| `1.0.0` | Initial mockup — Onboarding (3 steps), Containers, Images, Settings (basic) |
```

---

## ✅ `docs/design/` is complete
```
docs/design/
├── DESIGN-SYSTEM.md   ✅ — colour tokens, typography, spacing,
│                           components, status system, keyboard
│                           shortcuts, accessibility, microcopy
├── SCREENS.md         ✅ — every screen spec: onboarding (4 steps),
│                           shell, dashboard, containers (5 tabs),
│                           images, volumes, networks,
│                           suggestions, settings (5 sections)
└── MOCKUP.md          ✅ — what's implemented, 3 ways to run,
                            full interaction guide, design decisions,
                            relationship to production code, changelog
```

Your full `docs/` folder is now complete:
```
docs/
├── requirements/
│   ├── PRD.md          ✅
│   └── TRD.md          ✅
├── architecture/
│   ├── ARCHITECTURE.md ✅
│   └── *.png           ⏳ export from FigJam
└── design/
    ├── DESIGN-SYSTEM.md ✅
    ├── SCREENS.md        ✅
    └── MOCKUP.md         ✅