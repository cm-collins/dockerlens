# GitHub Actions — Best Practices

> **Applies to:** `.github/workflows/` — all CI/CD pipeline files
> **Last reviewed:** March 2026
> **References:** GitHub Security Hardening Guide · OWASP CI/CD Security

### What this repo runs today

Single workflow **`.github/workflows/ci.yml`**, job **`build-and-test`**: checkout → Node 20 + pnpm → `pnpm run lint` → `pnpm run test` → Rust stable → `cargo test` / `fmt` / `clippy` / `cargo audit` on `src-tauri/Cargo.lock`.

Sections below (extra jobs, Vitest, release signing, `pull_request_target` comparisons, etc.) are **patterns and templates** for growth — they are not all present in that file yet.

---

## Table of Contents

1. [Security Fundamentals](#1-security-fundamentals)
2. [Secret Management](#2-secret-management)
3. [Permissions Model](#3-permissions-model)
4. [Runner Security](#4-runner-security)
5. [Dependency Pinning](#5-dependency-pinning)
6. [Build Pipeline Structure](#6-build-pipeline-structure)
7. [Caching](#7-caching)
8. [Artifact Security](#8-artifact-security)
9. [Release Pipeline](#9-release-pipeline)
10. [Checklist](#10-checklist)

---

## 1. Security Fundamentals

### Never trust input from pull requests
```yaml
# ❌ Dangerous — pull_request_target has write access and exposes secrets
on:
  pull_request_target:  # DON'T use this unless you fully understand the implications
    types: [opened]

# ✅ Safe — pull_request has read-only access, no secrets exposed
on:
  pull_request:
    branches: [main]
```

### Restrict which branches trigger workflows
```yaml
on:
  push:
    branches: [main]
    tags: ['v*']
  pull_request:
    branches: [main]
```

### Validate inputs in workflows that accept them
```yaml
jobs:
  deploy:
    runs-on: ubuntu-22.04
    steps:
      - name: Validate version input
        run: |
          if [[ ! "${{ inputs.version }}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            echo "Invalid version format"
            exit 1
          fi
```

---

## 2. Secret Management

### Store all secrets in GitHub Secrets — never in code

**Repository Settings → Secrets and variables → Actions → New repository secret**

Secrets used by DockerLens CI:

| Secret name | Purpose |
|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | Signs auto-update artifacts |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Passphrase for signing key |
| `GPG_SIGNING_KEY` | GPG private key for artifact signing |
| `GPG_SIGNING_KEY_PASSWORD` | GPG passphrase |
| `SUPABASE_SERVICE_ROLE_KEY` | Only for DB migrations (never frontend) |

### Reference secrets correctly
```yaml
steps:
  - name: Build and sign
    env:
      TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
      TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
    run: pnpm tauri build
```

### Never print secrets — GitHub masks known secrets but not all
```yaml
# ❌ Can expose secrets via error messages or indirect output
- run: echo "Key is ${{ secrets.MY_SECRET }}"

# ✅ Never echo secrets
- run: echo "Build signing key is configured"
```

### Rotate secrets periodically

- Set a calendar reminder every 90 days to rotate CI secrets
- Immediately rotate any secret that may have been exposed in a log

---

## 3. Permissions Model

### Principle of least privilege — always
```yaml
# ✅ Explicitly declare minimum required permissions
permissions:
  contents: read       # read source code
  actions: read        # read workflow status

# For release workflow only
permissions:
  contents: write      # needed to create GitHub Releases
  id-token: write      # needed for OIDC signing
```

### Default permissions should be read-only

Set at the organization or repository level:

**Settings → Actions → General → Workflow permissions → Read repository contents permission**

Each workflow then explicitly declares what it needs.

### Never use `permissions: write-all`
```yaml
# ❌ Grants write access to everything — major security risk
permissions: write-all

# ✅ Explicit minimum
permissions:
  contents: write
  packages: write
```

---

## 4. Runner Security

### Use GitHub-hosted runners for security isolation
```yaml
# ✅ Fresh, isolated, GitHub-managed runner
runs-on: ubuntu-22.04

# For Fedora RPM builds — use a container
runs-on: ubuntu-22.04
container:
  image: fedora:40
```

### Pin runner versions
```yaml
# ✅ Pinned to specific Ubuntu version — reproducible
runs-on: ubuntu-22.04

# ❌ Moving target — can change unexpectedly
runs-on: ubuntu-latest
```

### Harden the runner environment
```yaml
steps:
  - name: Harden runner
    uses: step-security/harden-runner@v2
    with:
      egress-policy: audit   # Log unexpected network calls
```

---

## 5. Dependency Pinning

### Pin all GitHub Actions to a specific commit SHA

Using `@v2` trusts the tag — which can be moved. Using a commit SHA is immutable.
```yaml
# ❌ Tag can be moved by attacker
- uses: actions/checkout@v4

# ✅ Commit SHA is immutable
- uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683  # v4.2.2
```

Use Dependabot to keep action versions up to date:
```yaml
# .github/dependabot.yml
updates:
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

### Pin Docker images used in container jobs
```yaml
# ❌ latest tag changes daily
container:
  image: fedora:latest

# ✅ Pinned to digest — immutable
container:
  image: fedora:40@sha256:a1b2c3d4...
```

---

## 6. Build Pipeline Structure

### Sample multi-job pipeline (not in repo today)

**Today:** only `.github/workflows/ci.yml` with job `build-and-test`. The workflow below is a **template** for when you add release builds (Linux bundles, Fedora, tagged releases). Adjust action SHAs and runners to match your security policy.

```yaml
# Example: .github/workflows/build.yml (future)

name: Build & Release

on:
  push:
    branches: [main]
    tags: ['v*.*.*']
  pull_request:
    branches: [main]

permissions:
  contents: read

jobs:
  # ── Job 1: same role as today's ci.yml build-and-test ───────────────────────
  build-and-test:
    name: Build and Test
    runs-on: ubuntu-22.04
    permissions:
      contents: read

    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683  # v4.2.2

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Cache Rust dependencies
        uses: Swatinem/rust-cache@v2
        with:
          workspaces: "src-tauri -> target"

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install pnpm
        uses: pnpm/action-setup@v4
        with:
          version: 10

      - name: Install frontend deps
        run: pnpm install --frozen-lockfile

      - name: Frontend lint (TypeScript)
        run: pnpm run lint

      - name: Frontend test (build gate)
        run: pnpm run test

      # When Vitest is configured, add e.g.:
      # - name: Unit tests
      #   run: pnpm exec vitest run

      - name: Install Tauri system deps
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libwebkit2gtk-4.1-dev libgtk-3-dev \
            libayatana-appindicator3-dev librsvg2-dev

      - name: Rust format check
        run: cargo fmt --manifest-path src-tauri/Cargo.toml -- --check

      - name: Clippy
        run: cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings

      - name: Rust tests
        run: cargo test --manifest-path src-tauri/Cargo.toml

      - name: Security audit
        run: |
          cargo install cargo-audit --locked
          cargo audit --file src-tauri/Cargo.lock --deny warnings

  # ── Job 2: Build Ubuntu (.deb + .AppImage + Flatpak) ──────────────────────
  build-ubuntu:
    name: Build Ubuntu
    runs-on: ubuntu-22.04
    needs: build-and-test
    if: github.event_name == 'push'
    permissions:
      contents: read

    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683

      - name: Install system deps
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libwebkit2gtk-4.1-dev libgtk-3-dev \
            libayatana-appindicator3-dev librsvg2-dev

      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: "src-tauri -> target"

      - uses: pnpm/action-setup@v4
        with:
          version: 10

      - run: pnpm install --frozen-lockfile

      - name: Build Tauri app
        env:
          TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
          TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
        run: pnpm tauri build

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ubuntu-artifacts
          path: |
            src-tauri/target/release/bundle/deb/*.deb
            src-tauri/target/release/bundle/appimage/*.AppImage
          retention-days: 7

  # ── Job 3: Build Fedora (.rpm) ─────────────────────────────────────────────
  build-fedora:
    name: Build Fedora RPM
    runs-on: ubuntu-22.04
    needs: build-and-test
    if: github.event_name == 'push'

    container:
      image: fedora:40

    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683

      - name: Install Fedora deps
        run: |
          dnf install -y \
            webkit2gtk4.0-devel gtk3-devel \
            libappindicator-gtk3-devel librsvg2-devel \
            openssl-devel curl nodejs

      - uses: dtolnay/rust-toolchain@stable
      - uses: pnpm/action-setup@v4
        with:
          version: 10

      - run: pnpm install --frozen-lockfile

      - name: Build RPM
        env:
          TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
          TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
        run: pnpm tauri build

      - uses: actions/upload-artifact@v4
        with:
          name: fedora-artifacts
          path: src-tauri/target/release/bundle/rpm/*.rpm
          retention-days: 7

  # ── Job 4: Release ─────────────────────────────────────────────────────────
  release:
    name: Release
    runs-on: ubuntu-22.04
    needs: [build-ubuntu, build-fedora]
    if: startsWith(github.ref, 'refs/tags/v')
    permissions:
      contents: write

    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683

      - uses: actions/download-artifact@v4
        with:
          path: artifacts/

      - name: Generate checksums
        run: |
          find artifacts/ -type f \( -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" \) \
            -exec sha256sum {} \; > checksums.txt
          cat checksums.txt

      - name: Import GPG key
        run: |
          echo "${{ secrets.GPG_SIGNING_KEY }}" | gpg --import
        env:
          GPG_TTY: $(tty)

      - name: Sign artifacts
        run: |
          find artifacts/ -type f \( -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" \) | \
          while read f; do
            gpg --batch --yes --passphrase "${{ secrets.GPG_SIGNING_KEY_PASSWORD }}" \
              --detach-sign --armor "$f"
          done

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            artifacts/**/*.deb
            artifacts/**/*.rpm
            artifacts/**/*.AppImage
            artifacts/**/*.asc
            checksums.txt
          generate_release_notes: true
```

---

## 7. Caching

### Cache Rust compilation artifacts
```yaml
- name: Cache Rust
  uses: Swatinem/rust-cache@v2
  with:
    workspaces: "src-tauri -> target"
    # Cache key changes when Cargo.lock changes
    shared-key: "rust-${{ hashFiles('src-tauri/Cargo.lock') }}"
```

### Cache pnpm store
```yaml
- name: Cache pnpm store
  uses: actions/cache@v4
  with:
    path: ~/.local/share/pnpm/store
    key: pnpm-${{ hashFiles('pnpm-lock.yaml') }}
    restore-keys: pnpm-
```

### Cache invalidation rules

- Rust cache: invalidated when `Cargo.lock` changes
- pnpm cache: invalidated when `pnpm-lock.yaml` changes
- Never cache build output directories — only dependency caches

---

## 8. Artifact Security

### Set retention periods on uploaded artifacts
```yaml
- uses: actions/upload-artifact@v4
  with:
    name: build-artifacts
    path: src-tauri/target/release/bundle/
    retention-days: 7      # don't accumulate artifacts indefinitely
    if-no-files-found: error  # fail loudly if expected files are missing
```

### Verify artifact integrity before release
```yaml
- name: Verify artifact checksums
  run: |
    sha256sum --check checksums.txt
```

### Never include debug symbols in release artifacts
```toml
# src-tauri/Cargo.toml
[profile.release]
strip = true          # strips debug symbols — reduces binary size, prevents reverse engineering
```

---

## 9. Release Pipeline

### Semantic versioning for tags
```
v1.0.0   → major.minor.patch
v1.0.1   → patch — bug fixes only
v1.1.0   → minor — new features, backwards compatible
v2.0.0   → major — breaking changes
```

### Update `latest.json` for Tauri auto-updater
```yaml
- name: Generate update manifest
  run: |
    VERSION="${GITHUB_REF_NAME}"
    cat > latest.json << EOF
    {
      "version": "${VERSION}",
      "notes": "See release notes",
      "pub_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
      "platforms": {
        "linux-x86_64": {
          "signature": "$(cat artifacts/ubuntu-artifacts/*.AppImage.sig)",
          "url": "https://github.com/${{ github.repository }}/releases/download/${VERSION}/DockerLens.AppImage"
        }
      }
    }
    EOF
```

---

## 10. Checklist

### Before merging any PR

- [ ] Required CI job passes (today: `build-and-test` in `ci.yml`)
- [ ] No new secrets in code (search: `git grep -i "password\|secret\|token\|key"`)
- [ ] No `pull_request_target` triggers added without review
- [ ] New workflow steps use pinned action SHAs

### Before every release

- [ ] All release pipeline jobs pass (when a multi-job workflow like §6 exists — e.g. test + Ubuntu + Fedora + release)
- [ ] Artifacts are GPG signed
- [ ] SHA256 checksums published
- [ ] `latest.json` generated for auto-updater
- [ ] Release notes are accurate
- [ ] `CHANGELOG.md` updated
- [ ] Git tag follows `vX.Y.Z` format