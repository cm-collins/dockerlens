# DockerLens — Design System

> **Version:** 1.0.0
> **Last Updated:** March 2026
> **Screens Reference:** `docs/design/SCREENS.md`
> **Mockup Reference:** `docs/design/MOCKUP.md`

---

## Table of Contents

1. [Design Philosophy](#1-design-philosophy)
2. [Colour Tokens](#2-colour-tokens)
3. [Typography](#3-typography)
4. [Spacing & Layout](#4-spacing--layout)
5. [Border Radius](#5-border-radius)
6. [Component Library](#6-component-library)
7. [Status System](#7-status-system)
8. [Iconography](#8-iconography)
9. [Motion & Animation](#9-motion--animation)
10. [Keyboard Shortcuts](#10-keyboard-shortcuts)
11. [Accessibility](#11-accessibility)
12. [Microcopy & Tone](#12-microcopy--tone)

---

## 1. Design Philosophy

DockerLens should feel like a tool built by engineers, for engineers. Every design decision is driven by four principles:

| Principle | What it means in practice |
|---|---|
| **Information density** | Show what matters. Details on demand via panels and tabs. Never hide critical state. |
| **Zero latency feel** | Optimistic UI — actions reflect immediately before the server confirms. |
| **Status at a glance** | Colour-coded dots, badges and tray icons are always visible. Users should never need to read text to understand state. |
| **Native Linux feel** | Respect the system theme. Use native file dialogs. Behave like VS Code or Linear — not like a web app wrapped in a frame. |

**Reference aesthetic:** VS Code, Linear, Vercel Dashboard — dark-first, information-dense, no decorative noise.

---

## 2. Colour Tokens

All colours are defined as CSS custom properties and used consistently across every component.

### Background & Surface

| Token | Hex | Usage |
|---|---|---|
| `--bg` | `#080B14` | App background — darkest layer |
| `--surface` | `#0E1220` | Cards, panels, modals, detail panes |
| `--surface-hover` | `#141826` | Hover state on surface elements |
| `--sidebar` | `#090C18` | Sidebar + title bar — slightly darker than surface |
| `--border` | `#1E2535` | All borders, dividers, separators |
| `--border-light` | `#252D40` | Subtle dividers inside panels |

### Brand & Interactive

| Token | Hex | Usage |
|---|---|---|
| `--blue` | `#3B7EF6` | Primary CTA, active nav, links, focus rings |
| `--blue-dim` | `#1E3A6E` | Blue tinted backgrounds |
| `--blue-glow` | `#3B7EF625` | Glow effects, radial gradients |

### Status Colours

| Token | Hex | Usage |
|---|---|---|
| `--green` | `#22D47A` | Running state, success, connected |
| `--green-dim` | `#0D3D28` | Green tinted backgrounds |
| `--red` | `#F05252` | Stopped state, error, delete actions |
| `--red-dim` | `#3D1515` | Red tinted backgrounds |
| `--yellow` | `#F5A623` | Paused state, warning, suggestions |
| `--yellow-dim` | `#3D2A0A` | Yellow tinted backgrounds |

### Extended Palette

| Token | Hex | Usage |
|---|---|---|
| `--purple` | `#A78BFA` | Networks, volumes stats, user avatar gradient |
| `--purple-dim` | `#2D1B69` | Purple tinted backgrounds |
| `--cyan` | `#22D3EE` | Reserved — future use |
| `--cyan-dim` | `#0C3B45` | Reserved — future use |

### Text

| Token | Hex | Usage |
|---|---|---|
| `--text-primary` | `#E8EDF8` | Main text — headings, labels, values |
| `--text-secondary` | `#8B96B0` | Subtitles, descriptions, placeholders |
| `--text-muted` | `#4A5568` | Metadata, timestamps, secondary labels |

---

## 3. Typography

### Font Families

| Role | Font | Fallback | Usage |
|---|---|---|---|
| UI / Body | **DM Sans** | `system-ui, sans-serif` | All interface text |
| Code / Terminal | **JetBrains Mono** | `monospace` | Container IDs, socket paths, log lines, inspect JSON, CLI commands |

Both fonts are loaded from Google Fonts:
```css
@import url('https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap');
```

### Type Scale

| Role | Font | Size | Weight | Line Height |
|---|---|---|---|---|
| Page title | DM Sans | 22px | 800 | 1.2 |
| Section heading | DM Sans | 18px | 700 | 1.3 |
| Screen title (TopBar) | DM Sans | 15px | 700 | 1.4 |
| Container name | DM Sans | 14px | 700 | 1.4 |
| Body text | DM Sans | 13px | 400 | 1.6 |
| Label (uppercase) | DM Sans | 10–11px | 600 | 1.4 |
| Monospace value | JetBrains Mono | 12–13px | 400 | 1.7 |
| Log lines | JetBrains Mono | 11px | 400 | 1.8 |
| Metadata | JetBrains Mono | 10–11px | 400 | 1.6 |

### Label Convention

Section labels inside cards and panels use uppercase + letter-spacing:
```css
font-size: 10px;
font-weight: 600;
letter-spacing: 0.5px;
text-transform: uppercase;
color: var(--text-muted);
```

---

## 4. Spacing & Layout

### Spacing Scale

| Token | Value | Usage |
|---|---|---|
| `--space-xs` | `4px` | Icon-to-label gaps, tight inline spacing |
| `--space-sm` | `8px` | Compact padding inside badges, small gaps |
| `--space-md` | `12px` | Standard component padding |
| `--space-lg` | `16–20px` | Panel padding, list item padding |
| `--space-xl` | `22–24px` | Page-level padding |

### App Shell Layout
```
┌─────────────────────────────────────────────────────┐
│  Title Bar (38px) — macOS traffic lights + title     │
├──────────┬──────────────────────────────────────────┤
│          │  Top Bar (52–54px) — title + actions      │
│ Sidebar  ├──────────────────────────────────────────┤
│ (210px)  │                                          │
│          │  Main Content Area                       │
│          │  padding: 20–24px                        │
│          │                                          │
└──────────┴──────────────────────────────────────────┘
```

### Sidebar

- **Width:** 210px fixed
- **Background:** `--sidebar` (`#090C18`)
- **Border right:** `1px solid --border`

### Title Bar

- **Height:** 38px
- **Background:** `--sidebar`
- **Contains:** Traffic light circles (11px, macOS colours) + centred title

### Top Bar

- **Height:** 52–54px
- **Border bottom:** `1px solid --border`
- **Contains:** Screen title (left) + action buttons (right)

### Containers Split View
```
┌──────────────────────────┬──────────────────────────────┐
│  Container List (300px)  │  Detail Panel (flex: 1)       │
│  search + filter         │  header + tabs + tab content  │
└──────────────────────────┴──────────────────────────────┘
```

---

## 5. Border Radius

| Token | Value | Usage |
|---|---|---|
| `--radius-xs` | `5–6px` | Badges, tags, small pills |
| `--radius-sm` | `7–8px` | Buttons, inputs, small cards |
| `--radius-md` | `9–10px` | Cards, detail panels, overview grid items |
| `--radius-lg` | `12px` | Row items (volumes, networks), modals |
| `--radius-xl` | `14px` | Settings sections |
| `--radius-2xl` | `18–20px` | Onboarding card, large modals |
| `--radius-full` | `50%` | Status dots, user avatars, toggle knobs |

---

## 6. Component Library

### Button

Four variants. All share: `border-radius: 7px`, `font-weight: 600`, `font-family: inherit`, `cursor: pointer`.

| Variant | Background | Text | Border | Use case |
|---|---|---|---|---|
| `primary` | `--blue` | `#fff` | `--blue` | Main CTAs — Pull, Run, Launch |
| `ghost` | `transparent` | `--text-secondary` | `--border` | Secondary actions — Back, Cancel, Filter |
| `success` | `--green-dim` | `--green` | `#22D47A30` | Start, Confirm, Launch |
| `danger` | `--red-dim` | `--red` | `#F0525230` | Delete, Stop, Remove |

**Size variants:**
- Default: `padding: 8px 16px`, `font-size: 13px`
- Small: `padding: 5px 11px`, `font-size: 11px`

---

### Status Dot
```
Running  →  #22D47A  pulse animation + glow
Stopped  →  #F05252  static
Paused   →  #F5A623  static
Connected → #22D47A  pulse animation (networks)
Unknown  →  #4A5568  static
```

Standard sizes: `8px` (list), `10px` (detail header), `12px` (settings).

Pulse animation:
```css
@keyframes pulse {
  0%, 100% { opacity: 1; box-shadow: 0 0 5px currentColor; }
  50%       { opacity: 0.5; box-shadow: none; }
}
```

---

### Status Badge

Inline pill shown next to container names.

| State | Background | Text |
|---|---|---|
| `running` | `#0D3D28` | `#22D47A` |
| `stopped` | `#3D1515` | `#F05252` |
| `paused` | `#3D2A0A` | `#F5A623` |
| `built-in` | `#1E3A6E` | `#3B7EF6` |

Style: `font-size: 10px`, `padding: 2px 7px`, `border-radius: 10px`, `font-weight: 600`.

---

### Card
```css
background: var(--surface);       /* #0E1220 */
border: 1px solid var(--border);  /* #1E2535 */
border-radius: 10px;
padding: 14px 16px;
```

---

### Input
```css
background: var(--bg);
border: 1px solid var(--border);
border-radius: 7px;
padding: 7px 11px;
color: var(--text-primary);
font-size: 12px;
font-family: inherit;
outline: none;

/* Focus state */
border-color: var(--blue);
```

---

### Toggle Switch
```
Width: 40px
Height: 22px
Border radius: 11px
Knob: 16×16px, white, border-radius: 50%

ON:  background: --blue,    knob left: 21px
OFF: background: --border,  knob left: 3px
Transition: background 0.2s, left 0.2s
```

---

### Tab Bar

Tabs sit below the detail panel header. Active tab has a `2px solid --blue` bottom border.
```css
/* Tab button */
padding: 10–11px 14px;
background: none;
border: none;
border-bottom: 2px solid transparent;
font-size: 12px;
font-family: inherit;
cursor: pointer;
margin-bottom: -1px;   /* flush with the divider below */

/* Active */
color: var(--blue);
border-bottom-color: var(--blue);
font-weight: 600;

/* Inactive */
color: var(--text-muted);
```

---

### Expandable Row Item

Used for volumes and networks.
```css
background: var(--surface);
border: 1px solid var(--border);
border-radius: 12px;
padding: 16px 20px;
cursor: pointer;
transition: border-color 0.15s;

/* Selected */
border-color: var(--blue-dim) / var(--purple-dim);
```

---

### Navigation Item (Sidebar)
```css
/* Base */
display: flex;
align-items: center;
gap: 9px;
padding: 8px 12px;
border-radius: 7px;
border: 1px solid transparent;
font-size: 13px;
cursor: pointer;
color: var(--text-secondary);

/* Active */
background: rgba(59, 126, 246, 0.094);   /* --blue at 9.4% */
border-color: rgba(59, 126, 246, 0.188); /* --blue at 18.8% */
color: var(--blue);
font-weight: 600;

/* Suggestions nav item — active */
background: rgba(245, 166, 35, 0.094);
border-color: rgba(245, 166, 35, 0.188);
color: var(--yellow);
```

---

## 7. Status System

### Container Status

| Status | Dot | Badge | Tray icon |
|---|---|---|---|
| Running | 🟢 pulse | green pill | 🟢 green |
| Stopped | 🔴 static | red pill | 🔴 red |
| Paused | 🟡 static | yellow pill | — |

### Daemon Status

| State | Sidebar pill | Tray icon | Dashboard |
|---|---|---|---|
| Running | Green background, green text | 🟢 | Normal |
| Stopped | Red background, red text | 🔴 | Reconnect banner |

### Suggestion Severity

| Type | Icon | Border colour | Background |
|---|---|---|---|
| `warning` | ⚠ | `#F5A62325` | `--yellow-dim` icon bg |
| `info` | 💡 | `#3B7EF625` | `--blue-dim` icon bg |
| `success` | ✓ | `#22D47A25` | `--green-dim` icon bg |

---

## 8. Iconography

**Library:** Lucide React — consistent stroke weight, pixel-aligned.

Key icons used:

| Element | Icon |
|---|---|
| Containers | `◫` (custom Unicode — placeholder until Lucide) |
| Images | `⬡` |
| Volumes | `◉` |
| Networks | `◈` |
| Compose | `≋` |
| Suggestions | `💡` |
| Settings | `⚙` |
| Docker logo | 🐳 |

**Sidebar nav icons:** 13px, `opacity: 0.8` when inactive.

---

## 9. Motion & Animation

Keep animation purposeful and fast. No decorative motion.

| Animation | Duration | Easing | Trigger |
|---|---|---|---|
| Screen transition (`fadeIn`) | 200ms | `ease` | Page change |
| Container row selection | 100ms | linear | Click |
| Status dot pulse | 2s | infinite | Running state |
| Cursor blink (terminal) | 1s | infinite | Always in terminal |
| Toggle knob slide | 200ms | linear | Toggle click |
| Daemon pill background | 200ms | linear | Daemon state change |
| Theme pill border | 150ms | linear | Theme selection |
```css
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(6px); }
  to   { opacity: 1; transform: translateY(0); }
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50%       { opacity: 0; }
}
```

---

## 10. Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+1` | Navigate to Dashboard |
| `Ctrl+2` | Navigate to Containers |
| `Ctrl+3` | Navigate to Images |
| `Ctrl+4` | Navigate to Volumes |
| `Ctrl+5` | Navigate to Networks |
| `Ctrl+6` | Navigate to Suggestions |
| `Ctrl+,` | Navigate to Settings |
| `Ctrl+R` | Refresh current view |
| `Ctrl+F` | Focus search input |
| `Ctrl+T` | Open Terminal tab (container detail) |
| `Ctrl+L` | Open Logs tab (container detail) |
| `Space` | Start / Stop selected container |
| `Del` | Delete selected container or image |
| `Escape` | Close modal / collapse detail panel |

---

## 11. Accessibility

| Requirement | Implementation |
|---|---|
| Keyboard navigable | All interactive elements reachable via Tab |
| Focus rings | `box-shadow: 0 0 0 2px --blue` on focus |
| Screen reader labels | `aria-label` on icon-only buttons |
| Status dot | Always paired with text badge — never colour alone |
| Colour contrast | Text on all backgrounds meets WCAG AA (4.5:1 minimum) |
| Reduced motion | `@media (prefers-reduced-motion)` disables pulse and fadeIn |

---

## 12. Microcopy & Tone

DockerLens speaks like a knowledgeable colleague — direct, honest, no fluff.

### Empty States

| Screen | Empty state copy |
|---|---|
| Containers (no containers) | `No containers yet. Run one from the Images tab.` |
| Containers (none running) | `No containers running. Start one to begin.` |
| Images (no images) | `No images pulled yet. Click Pull Image to get started.` |
| Suggestions (all resolved) | `All caught up. No active suggestions right now.` |

### Error States

| Situation | Copy |
|---|---|
| Socket not found | `Docker socket not found. Make sure Docker Engine is installed.` |
| Not in docker group | `Your user isn't in the docker group yet. Run the command below, then log out and back in.` |
| Daemon stopped | `Docker daemon is not running. Click Start Docker to continue.` |
| Daemon start cancelled | `Start cancelled. Click Start Docker to try again.` |
| Pull failed | `Pull failed. Check the image name and tag and try again.` |

### Action Confirmations

| Action | Confirmation copy |
|---|---|
| Delete container | `Delete container? This action cannot be undone.` |
| Delete image | `Remove image? Any containers using it will need to pull it again.` |
| Remove volume | `Remove volume? All data stored in it will be permanently deleted.` |
| Stop Docker | `Stop Docker? All running containers will be paused.` |
| Prune containers | `Remove all stopped containers? This cannot be undone.` |

### Polkit Dialog

| Field | Copy |
|---|---|
| Title | `DockerLens wants to start Docker` |
| Body | `Enter your password to start the Docker Engine daemon.` |