# DockerLens — Screen Specifications

> **Version:** 1.0.0
> **Last Updated:** March 2026
> **Design System:** `docs/design/DESIGN-SYSTEM.md`
> **Mockup:** `docs/design/MOCKUP.md`

---

## Table of Contents

1. [Onboarding Wizard](#1-onboarding-wizard)
2. [App Shell](#2-app-shell)
3. [Dashboard](#3-dashboard)
4. [Containers](#4-containers)
5. [Images](#5-images)
6. [Volumes](#6-volumes)
7. [Networks](#7-networks)
8. [Suggestions](#8-suggestions)
9. [Settings](#9-settings)

---

## 1. Onboarding Wizard

Shown on first launch only. A centred modal card on a dark background with a blue radial gradient at the top.

**Card dimensions:** 520px wide, auto height
**Background:** `--bg` with `radial-gradient(ellipse 60% 40% at 50% 0%, --blue-glow, transparent)`

### Step 1 — Welcome

| Element | Spec |
|---|---|
| Progress dots | 4 dots, active = 26px wide, inactive = 12px, `--blue` / `--border` |
| Logo | 88×88px, `border-radius: 22px`, blue gradient, 🐳 emoji at 44px |
| Headline | `Welcome to DockerLens` — 21px / 800 weight |
| Sub | `The Docker Desktop Linux deserves.` — muted |
| Feature pills | 4 pills: No VM overhead · Free forever · All distros · Native feel |
| CTA | `Continue →` — primary button, right-aligned in footer |

---

### Step 2 — Sign In (Optional)

| Element | Spec |
|---|---|
| GitHub button | Black (`#161B22`) background, `#30363D` border, GitHub SVG icon, white text |
| Google button | White background, `#ddd` border, Google SVG icon, black text |
| Divider | `─── or ───` in muted text |
| Skip link | `Skip for now` — plain text button in muted colour, left of Continue |
| Continue button | Primary — right-aligned |
| Success state | Green checkmark circle, `Signed in with {provider}` heading, muted confirmation |
| Back button | Ghost — shown from step 2 onwards |

---

### Step 3 — System Check

| Element | Spec |
|---|---|
| Check items | 4 rows: Docker installed · User in docker group · Socket accessible · Daemon running |
| Item row | `background: --surface`, `border-radius: 9px`, circle icon + label + monospace detail |
| Circle icon states | `○` grey (unchecked) → `⟳` blue (checking) → `✓` green (passed) |
| `Run System Check` button | Primary, centred below items |
| Checking state | Blue spinner text: `Scanning system…` |
| All passed state | Green banner: `✓ All checks passed — ready to launch!` |
| Transition | `border: 1px solid --green + 25` on each row with `transition-delay` stagger |

---

### Step 4 — Ready

| Element | Spec |
|---|---|
| Checkmark circle | 70×70px, `--green-dim` background, green border, green glow shadow |
| Summary grid | 2×2 grid of cards: containers · images · volumes · networks |
| Each summary card | Icon (18px) + bold count + muted subtitle |
| CTA | `Launch DockerLens →` — success (green) button |

---

## 2. App Shell

The persistent wrapper rendered after onboarding completes.

### Title Bar (38px)

| Element | Spec |
|---|---|
| Traffic lights | 3 circles, 11px, `#FF5F57` / `#FFBD2E` / `#28CA42`, gap: 5px |
| Title | Centred, `font-size: 11px`, `color: --text-muted` — `DockerLens — {screen}` |
| User label | Right-aligned monospace, 10px, muted — signed-in user's name |

---

### Sidebar (210px)

| Element | Spec |
|---|---|
| Logo section | 🐳 30×30px gradient icon + `DockerLens` 14px/700 + `v1.0.0 · Linux` monospace 10px muted |
| Daemon pill | Clickable, green/red background, status dot + label + ⚙ icon. Click → Settings |
| Nav items | 8px 12px padding, 7px radius, blue active state, yellow for Suggestions |
| Container count badge | `--blue` background, white text, 10px/700, `border-radius: 10px` |
| Suggestions badge | `--yellow` background, black text, same spec |
| User pill | Shown when signed in — avatar gradient circle + name + provider |
| Settings link | Bottom section, above border |

---

### Top Bar (52–54px)

| Element | Spec |
|---|---|
| Screen title | 15px / 700 |
| Screen subtitle | 11px, `--text-muted` — e.g. `4 running · 6 total` |
| Action buttons | Right-aligned, primary variant |

---

## 3. Dashboard

Default landing screen after onboarding.

### Stat Cards Row (4 columns)

Each card is clickable and navigates to the relevant screen.

| Card | Icon colour | Value | Subtitle |
|---|---|---|---|
| Running containers | `--green` ◫ | Count | `N stopped` |
| Images | `--blue` ⬡ | Count | Total size |
| Volumes | `--yellow` ◉ | Count | Total used |
| Networks | `--purple` ◈ | Count | `N custom` |

Card spec: `--surface` background, `border-radius: 12px`, `padding: 18px`, arrow icon top-right.

---

### Memory Usage Card

- Section label: `MEMORY USAGE` — uppercase muted
- Per running container: name (left) + size (right monospace) + progress bar
- Progress bar: 5px height, `--blue` fill gradient, `border-radius: 3px`
- Footer row: `Total used` + `N MB / 8192 MB` in blue monospace

---

### Containers Card

- Section label: `CONTAINERS`
- 5 most recent containers: dot + name + badge
- `View all →` link in blue at bottom

---

### Suggestions Preview Card

- Section label: `SUGGESTIONS` + `N active →` link right-aligned
- Top 2 active suggestions: icon + title + body
- Coloured left border per suggestion type

---

## 4. Containers

Split-panel layout. No Top Bar padding — the panel fills the full height.

### Left Panel — Container List (300px)

| Element | Spec |
|---|---|
| Search input | Full width, `--bg` background, `border-radius: 7px` |
| Container row | `padding: 11px 14px`, `border-bottom: 1px solid --border` |
| Active row | `background: --blue at 6.25%`, `border-left: 3px solid --blue` |
| Row content | Status dot + name (bold) + badge + image (muted monospace) + CPU/MEM for running |

---

### Right Panel — Container Detail

**Header:**

| Element | Spec |
|---|---|
| Status dot | 10px |
| Container name | 15px / 700 |
| Container meta | ID + image — `10px monospace --text-muted` |
| Action buttons | Right-aligned: Pause / Stop / Restart (ghost) + Delete (danger) OR Start (success) + Delete |

**Tab Bar:** Overview · Logs · Terminal · Stats · Inspect

---

### Overview Tab

2-column grid of info cards. Each card:
- Label: `10px uppercase --text-muted letter-spacing: 0.5px`
- Value: `13px --text-primary font-weight: 600 monospace`
- Status value uses status colour

Fields: Status · Ports · CPU Usage · Memory · Network · Image

---

### Logs Tab

| Element | Spec |
|---|---|
| Background | `#060810` — darker than app background |
| Font | JetBrains Mono 11px, line-height 1.8 |
| `INFO` lines | `--text-secondary` |
| `WARN` lines | `--yellow` |
| `ERROR` lines | `--red` |
| Cursor | `█` blue, blink animation |
| Auto-scroll | Scrolls to bottom as new lines arrive |

---

### Terminal Tab

| Element | Spec |
|---|---|
| Background | `#060810` |
| Prompt | Green — `root@{id}:/# ` |
| Command text | `--text-primary` |
| Output | `--text-secondary` |
| Cursor | `█` blue, blink animation |
| Connection header | Green text: `Connected to {name} — /bin/sh` |

---

### Stats Tab

Three stats with progress bars:

| Stat | Colour |
|---|---|
| CPU Usage | `--blue` |
| Memory | `--green` |
| Network I/O | `--yellow` |

Bar spec: `height: 7px`, `--border` background, `border-radius: 4px`, value label right-aligned.

---

### Inspect Tab

Raw Docker inspect JSON. JetBrains Mono 11px, `--text-secondary`, `line-height: 1.8`, scrollable.

---

## 5. Images

Full-width table view.

### Table

Columns: Name · Tag · Size · Created · ID · In use · Actions

| Column | Spec |
|---|---|
| Name | `--text-primary` 13px / 600 |
| Tag | `--blue` monospace, `background: --blue at 8%`, `border-radius: 5px`, `padding: 2px 7px` |
| Size | `--text-secondary` monospace 12px |
| Created | `--text-muted` 12px |
| ID | `--text-muted` monospace 11px — first 12 chars only |
| In use | `--green` `✓ Yes` or `--text-muted` `—` |
| Actions | Run (ghost) + Delete (danger) — small variant |

Row style: `background: --surface`, borders on top/bottom + first/last column, `border-radius: 9px` on ends.

---

### Pull Modal

| Element | Spec |
|---|---|
| Input | `name:tag` — full width, `--bg` background |
| Progress bar | `height: 6px`, blue gradient fill, `transition: width 0.1s` |
| Progress label | Percentage right-aligned, `--blue` monospace |
| Status text | Monospace muted: `Pulling from...` → `Downloading...` → `Extracting...` → `Pull complete ✓` |

---

### Run Wizard

5-step form inside a modal or side panel. Step indicator at top. Each step is a form with labelled inputs.

| Step | Fields |
|---|---|
| 1 | Container name (text input) |
| 2 | Port mappings (HOST:CONTAINER — add/remove rows) |
| 3 | Environment variables (KEY=VALUE — add/remove rows) |
| 4 | Volume mounts (/host:/container — add/remove rows) |
| 5 | Restart policy (select: always / on-failure / no / unless-stopped) |

---

## 6. Volumes

### Stats Row (3 columns)

| Card | Colour | Value |
|---|---|---|
| Total Volumes | `--purple` | Count |
| Total Used | `--blue` | Size string |
| Unused | `--yellow` | Count |

---

### Volume Cards

One card per volume. Expandable on click.

**Collapsed state:**

| Element | Spec |
|---|---|
| Icon | 34×34px, `border-radius: 9px` — blue if attached, red if unattached |
| Name | 14px / 700 |
| Mount path | Monospace 10px muted |
| Size | Monospace 13px / 600 right-aligned |
| Driver + created | Muted 10px right-aligned |
| Container badges | Blue pills with `◫` prefix + container name |
| No containers | Muted text: `No containers attached` |
| Remove button | Danger small — only shown if no containers attached |

**Expanded state (click to toggle):**

Additional row with 3 columns: Driver · Size · Created — each with muted label + monospace value.

---

## 7. Networks

### Stats Row (3 columns)

| Card | Colour | Value |
|---|---|---|
| Total Networks | `--purple` | Count |
| Custom | `--blue` | Count |
| Connected Containers | `--green` | Total count |

---

### Network Cards

One card per network. Expandable on click.

**Collapsed state:**

| Element | Spec |
|---|---|
| Icon | 34×34px — purple if built-in, blue if custom |
| Name | 14px / 700 |
| `built-in` badge | Blue pill — only on bridge, host, none |
| Subnet | Monospace 10px muted |
| Driver | 12px / 600 right-aligned |
| Scope | Muted 10px right-aligned |
| Container pills | Green with animated pulse dot + container name |
| No containers | Muted: `No containers connected` |
| Remove button | Danger small — only on non-built-in networks |

**Expanded state:**

3-column grid: ID (first 12 chars) · Gateway · Driver

---

## 8. Suggestions

### Stats Row (3 columns)

| Card | Colour | Value |
|---|---|---|
| Active | `--yellow` | Count |
| Resolved | `--green` | Count |
| Total | `--blue` | Count |

---

### Active Suggestions

Section label: `ACTIVE SUGGESTIONS` — uppercase muted

Each suggestion card:

| Element | Spec |
|---|---|
| Icon container | 38×38px, `border-radius: 10px`, coloured background per type |
| Title | 14px / 700 |
| Body | `--text-secondary` 12px, `line-height: 1.6` |
| Action button | Primary small — navigates to relevant screen |
| Dismiss button | Ghost small |
| Border | `1px solid` — coloured per type at 14% opacity |

---

### Empty Active State

Centred: large `✓` at 40% opacity + `All caught up!` heading + muted subtitle.

---

### Resolved Section

Section label: `RESOLVED` — uppercase muted

Each resolved item: `opacity: 0.5`, `✓` icon + title only. No actions.

---

## 9. Settings

Full-width scrollable, `max-width: 640px`.

### Account Section

**Signed out state:** GitHub + Google sign-in buttons side by side.

**Signed in state:** Avatar circle (gradient) + name + provider + Sign out danger button.

---

### Docker Daemon Section

| Element | Spec |
|---|---|
| Status row | Status dot (12px) + `Docker is Running/Stopped` (13px/600) + socket path monospace muted |
| Action buttons | Start (success) · Stop (danger) · Restart (ghost) — flex row |
| Command feedback | `background: --blue at 7%`, `border: 1px solid --blue at 19%`, monospace blue text |
| Start on Login toggle | Toggle switch + label + `systemctl enable docker` monospace muted |

---

### Appearance Section

Three theme pills: `🌙 Dark` · `☀️ Light` · `🖥 System`

Active pill: `2px solid --blue`, `background: --blue at 7%`, blue text, `font-weight: 600`.

Notifications toggle below theme pills.

---

### Connection Section

Socket path text input (monospace) + Test Connection ghost button.

---

### Danger Zone Section

Red-tinted section header. Two danger buttons side by side:
- `Prune Stopped Containers`
- `Remove Unused Images`