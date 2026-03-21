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
| `SUPABASE.md` | RLS, key management, PKCE OAuth, session management, schema security | `src/lib/supabase.ts` · Supabase dashboard |
| `GITHUB-ACTIONS.md` | Secret management, permissions, dependency pinning, CI pipeline structure | `.github/workflows/` |
| `GENERAL.md` | Git workflow, commits, code review, versioning, performance targets | Everything |

---

## Quick Reference

### Run before every PR
```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo audit --manifest-path src-tauri/Cargo.toml
pnpm tsc --noEmit
pnpm lint
pnpm vitest run
```

### Run before every release
```bash
pnpm tauri build
cargo audit
cargo geiger --all-features
cargo deny check
```
```

---

## ✅ `docs/best-practices/` is complete
```
docs/best-practices/
├── README.md                  ✅ index + quick reference
├── RUST.md                    ✅ memory safety · error handling · unsafe · concurrency · testing
├── TAURI.md                   ✅ IPC security · capabilities · CSP · deep links · signing · updates
├── REACT-TYPESCRIPT.md        ✅ types · components · state · performance · security · testing
├── SUPABASE.md                ✅ RLS · key management · PKCE · sessions · schema · production checklist
├── GITHUB-ACTIONS.md          ✅ secrets · permissions · pinning · full CI pipeline · release pipeline
└── GENERAL.md                 ✅ git · commits · code review · deps · versioning · open source · perf targets
```

Your complete `docs/` folder is now fully done:
```
docs/
├── requirements/
│   ├── PRD.md              ✅
│   └── TRD.md              ✅
├── architecture/
│   ├── ARCHITECTURE.md     ✅
│   └── *.png               ✅ (exported from FigJam)
├── design/
│   ├── DESIGN-SYSTEM.md    ✅
│   ├── SCREENS.md          ✅
│   └── MOCKUP.md           ✅
└── best-practices/
    ├── README.md            ✅
    ├── RUST.md              ✅
    ├── TAURI.md             ✅
    ├── REACT-TYPESCRIPT.md  ✅
    ├── SUPABASE.md          ✅
    ├── GITHUB-ACTIONS.md    ✅
    └── GENERAL.md           ✅