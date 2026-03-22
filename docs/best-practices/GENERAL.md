# General — Project-Wide Best Practices

> **Applies to:** Entire DockerLens repository
> **Last reviewed:** March 2026

---

## Table of Contents

1. [Git Workflow](#1-git-workflow)
2. [Commit Conventions](#2-commit-conventions)
3. [Code Review](#3-code-review)
4. [Dependency Updates](#4-dependency-updates)
5. [Documentation](#5-documentation)
6. [Environment Variables](#6-environment-variables)
7. [Versioning](#7-versioning)
8. [Open Source Etiquette](#8-open-source-etiquette)
9. [Performance Targets](#9-performance-targets)
10. [Master Checklist](#10-master-checklist)

---

## 1. Git Workflow

### Branch naming
```
feat/container-exec-terminal     ← new feature
fix/daemon-reconnect-loop        ← bug fix
docs/update-architecture-md      ← documentation only
chore/update-bollard-0.18        ← dependency or tooling update
refactor/volumes-module          ← code change with no user-facing change
test/container-list-unit-tests   ← tests only
```

### Branch protection rules (set in GitHub → Settings → Branches)

| Rule | Setting |
|---|---|
| Require PR before merging to `main` | ✅ |
| Require status checks to pass | ✅ required CI job (see `.github/workflows/ci.yml` — today: `build-and-test`) |
| Require branches to be up to date | ✅ |
| No direct push to `main` | ✅ |
| Require linear history | ✅ (use rebase, not merge commits) |

### Keep branches short-lived

- Open a PR within 2 days of starting a branch
- No branch should live longer than 1 week without review
- Delete branches after merging

---

## 2. Commit Conventions

DockerLens uses **Conventional Commits** (`conventionalcommits.org`).

### Format
```
<type>(<optional scope>): <description>

<optional body>

<optional footer>
```

### Types

| Type | When to use |
|---|---|
| `feat` | New user-facing feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `chore` | Build, CI, tooling, dependencies |
| `test` | Adding or updating tests |
| `refactor` | Code change with no feature or fix |
| `perf` | Performance improvement |
| `style` | Formatting, whitespace (no logic change) |

### Examples
```
feat(containers): add exec terminal to detail panel

Implements a full WebSocket TTY exec session using xterm.js.
The terminal opens in a new tab in the container detail panel.

Closes #42

---

fix(daemon): prevent reconnect loop when daemon intentionally stopped

Previously, clicking Stop Docker would trigger the auto-reconnect
loop immediately. Now the loop only fires if the daemon stops
unexpectedly.

---

chore(deps): update bollard to 0.18.0

Adds support for Docker API v1.44 and fixes streaming stability
on high-throughput log outputs.
```

### Breaking changes
```
feat(api)!: rename invoke command list_all_containers to list_containers

BREAKING CHANGE: The Tauri command name has changed. Update all
invoke('list_all_containers') calls to invoke('list_containers').
```

---

## 3. Code Review

### Before opening a PR

- [ ] Self-review your diff — read every line before requesting review
- [ ] All CI checks pass locally (`pnpm tauri dev` runs, `cargo test` passes)
- [ ] PR description explains *why*, not just *what*
- [ ] Reference the relevant issue number (`Closes #XX`)
- [ ] Screenshots or a short screen recording for UI changes

### PR description template
```markdown
## What

One-paragraph summary of what this PR does.

## Why

Why is this change needed? Which issue does it address?

## How

Brief explanation of the implementation approach.

## Testing

How did you test this? What edge cases did you consider?

## Checklist

- [ ] `cargo clippy` — zero warnings
- [ ] `cargo test` — all pass
- [ ] `pnpm run lint` — TypeScript clean (`tsc --noEmit`)
- [ ] `pnpm run test` — passes (today: production build gate; add Vitest when configured)
- [ ] No secrets in code
- [ ] Documentation updated if needed
```

### Review standards

- Approve only when you have genuinely read and understood the code
- Leave specific, actionable comments — not vague "this looks weird"
- Distinguish between blocking issues (`must fix:`) and suggestions (`nit:`)
- Approve + merge — don't leave PRs open after approving
- Any PR touching `src-tauri/src/system/daemon.rs` requires two approvals

---

## 4. Dependency Updates

### Dependabot handles routine updates automatically

Configured in `.github/dependabot.yml` to run **daily** for:
- Cargo (Rust) — Mondays 09:00 Africa/Nairobi
- npm (frontend) — Mondays 09:00 Africa/Nairobi
- GitHub Actions — Mondays 09:00 Africa/Nairobi

### Manual update procedure for major versions

1. Open an issue discussing the breaking changes
2. Create a feature branch: `chore/update-tauri-2.1`
3. Test thoroughly — run the full app, not just tests
4. Update the TRD dependency reference table
5. Merge only after full CI passes

### Never ignore security advisories

When `cargo audit` or Dependabot reports a vulnerability:
- **Critical/High:** Fix within 24 hours — patch release if needed
- **Medium:** Fix within 1 week
- **Low:** Fix in next scheduled update cycle

---

## 5. Documentation

### What must be documented

| Change | Required documentation |
|---|---|
| New Tauri command | Rust doc comment on the function |
| New React component | Props interface with JSDoc comments |
| New architecture decision | Update `docs/Archtecture/ARCHITECTURE.md` |
| New feature | Update `docs/Requirements/PRD.md` |
| New dependency | Update `docs/Requirements/TRD.md` dependency table |
| Breaking API change | Update TRD + `CHANGELOG.md` |

### Keep `CHANGELOG.md` up to date

Every PR that changes user-facing behaviour must add an entry to `CHANGELOG.md` under `[Unreleased]`:
```markdown
## [Unreleased]

### Added
- Container exec terminal — full WebSocket TTY session inside the app

### Fixed
- Daemon reconnect loop no longer fires after intentional Stop Docker action

### Changed
- Container list now shows network name alongside CPU and memory

### Security
- Updated bollard to 0.18.0 — fixes potential stream buffer overflow
```

---

## 6. Environment Variables

### Three environments — three `.env` files
```
.env                  ← development defaults (committed — no secrets)
.env.local            ← local overrides (never committed)
.env.production       ← production values (never committed — use CI secrets)
```

### What goes where
```bash
# .env — safe to commit — defaults only
VITE_APP_NAME=DockerLens
VITE_APP_VERSION=1.0.0

# .env.local — never commit — developer-specific
VITE_SUPABASE_URL=https://your-dev-project.supabase.co
VITE_SUPABASE_ANON_KEY=eyJhbGci...

# .env.production — never commit — set as GitHub Secrets
VITE_SUPABASE_URL=https://your-prod-project.supabase.co
VITE_SUPABASE_ANON_KEY=eyJhbGci...
TAURI_SIGNING_PRIVATE_KEY=...
```

### Validate required variables at startup
```typescript
// src/lib/env.ts
const required = ['VITE_SUPABASE_URL', 'VITE_SUPABASE_ANON_KEY'] as const;

for (const key of required) {
    if (!import.meta.env[key]) {
        throw new Error(`Missing required environment variable: ${key}`);
    }
}

export const env = {
    supabaseUrl: import.meta.env.VITE_SUPABASE_URL as string,
    supabaseAnonKey: import.meta.env.VITE_SUPABASE_ANON_KEY as string,
    appVersion: import.meta.env.VITE_APP_VERSION as string,
} as const;
```

---

## 7. Versioning

### Semantic versioning (`semver.org`)
```
MAJOR.MINOR.PATCH

1.0.0 → 1.0.1  patch: bug fix
1.0.1 → 1.1.0  minor: new feature, backwards compatible
1.1.0 → 2.0.0  major: breaking change
```

### Version lives in one place — `package.json`
```json
{
  "version": "1.0.0"
}
```

The Tauri build system reads this automatically. Never manually update the version in `tauri.conf.json` — it should always reference the `package.json` version.

### Release checklist
```bash
# 1. Update version
npm version patch  # or minor, or major

# 2. Update CHANGELOG.md — move [Unreleased] to [1.0.1]

# 3. Commit
git add package.json CHANGELOG.md
git commit -m "chore: release v1.0.1"

# 4. Tag
git tag -a v1.0.1 -m "Release v1.0.1"

# 5. Push — triggers CI release pipeline
git push origin main --tags
```

---

## 8. Open Source Etiquette

### For maintainers

- Respond to issues within 48 hours — even a brief acknowledgement
- Use issue labels consistently: `bug`, `enhancement`, `good first issue`, `help wanted`, `wontfix`, `duplicate`
- Close stale PRs (no activity for 30 days) with a friendly message
- Celebrate contributors in release notes

### For contributors

- Check for an existing issue before opening a new one
- For large changes — open an issue first to discuss before writing code
- One feature or fix per PR — keep PRs focused
- Don't resolve review comments yourself — let the reviewer resolve
- Be responsive — PRs with no activity for 14 days may be closed

### Issue templates

Create in `.github/ISSUE_TEMPLATE/`:
```
.github/ISSUE_TEMPLATE/
├── bug_report.md         ← steps to reproduce, expected vs actual behaviour
├── feature_request.md    ← user story, motivation, proposed solution
└── config.yml            ← template chooser config
```

---

## 9. Performance Targets

These are hard targets, not aspirational goals. Any PR that causes these to regress requires an explicit discussion before merging.

| Metric | Target | How to measure |
|---|---|---|
| App launch time | < 2 seconds | Time from launch to dashboard visible |
| Log stream latency | < 50ms | Time from Docker output to xterm.js render |
| Memory at idle | < 120 MB | `cat /proc/$(pgrep dockerlens)/status \| grep VmRSS` |
| Binary size (.AppImage) | < 15 MB | `ls -lh *.AppImage` |
| Container list render | < 100ms | React DevTools Profiler |
| Daemon reconnect time | < 6 seconds | Time from daemon start to dashboard reload |

---

## 10. Master Checklist

Run this before every PR and release:

### Every PR
```bash
# Rust (same shape as CI)
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo audit --file src-tauri/Cargo.lock --deny warnings

# Frontend
pnpm run lint
pnpm run test

# Security
git grep -i "password\|secret\|private_key\|service_role" -- '*.ts' '*.tsx' '*.rs'
```

### Every release
```bash
# All PR checks above, plus:
pnpm tauri build                              # production build succeeds
sha256sum artifacts/*.AppImage *.deb *.rpm   # checksums generated
gpg --verify artifact.AppImage.asc           # GPG signatures valid
cargo audit                                   # zero vulnerabilities
cat CHANGELOG.md | head -20                   # changelog updated
git tag -v v1.x.x                            # tag is signed
```