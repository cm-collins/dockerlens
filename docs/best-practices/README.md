# DockerLens — Best Practices

> This folder contains best practices for every technology used in DockerLens.
> Read the relevant file before touching any part of the codebase.

---

## Files

| File | Covers | Read before touching |
|---|---|---|
| `RUST.md` | Memory safety, error handling, unsafe code, concurrency, testing, security | `src-tauri/src/` |
| `TAURI.md` | IPC security, capabilities, command design, CSP, deep links, signing | `src-tauri/tauri.conf.json` · `src-tauri/capabilities/` |
| `REACT-TYPESCRIPT.md` | Type safety, component design, state management, performance, security | `src/` |
| `SUPABASE.md` | RLS, key management, PKCE OAuth, session management, schema security | Supabase dashboard · future `src/lib/supabase.ts` |
| `GITHUB-ACTIONS.md` | Secret management, permissions, dependency pinning, CI pipeline structure | `.github/workflows/` |
| `GENERAL.md` | Git workflow, commits, code review, versioning, performance targets | Everything |

---

## Quick Reference

### Run before every PR (matches CI today)

Commands mirror `.github/workflows/ci.yml` and `package.json` scripts:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo audit --file src-tauri/Cargo.lock --deny warnings
pnpm run lint          # tsc --noEmit (see package.json)
pnpm run test          # currently: production UI build gate
```

**Not wired in this repo yet:** Vitest/ESLint/Prettier, `cargo geiger`, `cargo deny` — see `REACT-TYPESCRIPT.md` / `RUST.md` when you add them.

### Run before every release

```bash
pnpm tauri build
cargo audit --file src-tauri/Cargo.lock
# Optional hardening (install tools first): cargo geiger --all-features ; cargo deny check
```

---

## ✅ `docs/best-practices/` index

These files are **guidance** (and some **forward-looking** examples). They are not all enforced by the current minimal scaffold.

```
docs/best-practices/
├── README.md                  ← this file
├── RUST.md                    ← Rust backend standards
├── TAURI.md                   ← Tauri 2 / IPC / capabilities
├── REACT-TYPESCRIPT.md        ← frontend standards (target stack)
├── SUPABASE.md                ← when optional auth/sync ships
├── GITHUB-ACTIONS.md          ← CI/CD patterns (see also `.github/workflows/`)
└── GENERAL.md                 ← repo-wide process
```

Related spec folders (actual paths on disk — mind `Archtecture` spelling):

```
docs/
├── Requirements/
│   ├── PRD.md
│   └── TRD.md
├── Archtecture/
│   ├── ARCHITECTURE.md
│   └── *.png / *.pdf
├── Design/
│   ├── DESIGN-SYSTEM.md
│   ├── SCREENS.md
│   └── MOCKUP.md
└── best-practices/
    └── …
```