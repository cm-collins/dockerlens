 
## Contributing to dockerlens

Thanks for helping make **dockerlens** better.

The repo currently has a **Tauri 2 + React + Vite** scaffold (`src/`, `src-tauri/`) plus product and engineering docs under `docs/`. The Docker UI described in the README is still to be built on top of that shell. Contributions are welcome for **features, planning, design, documentation, and tooling**. This guide describes the dev workflow as it exists today.

## Ways to contribute (today)

- **Report bugs / request features**: Open an issue with clear reproduction steps or a concrete user story.
- **Design & UX feedback**: Share screenshots/mockups and interaction notes (what you expected vs what happened).
- **Docs improvements**: Clarify wording, add examples, fix typos, expand FAQs.
- **Project setup**: Suggest CI, release automation, linting/formatting standards, or templates.

## Before you start

- **Be kind**: This project follows the code of conduct in `CODE_OF_CONDUCT.md`.
- **Search first**: Check existing issues/PRs to avoid duplicates.
- **Keep scope small**: Prefer focused PRs over “big bang” changes.

## Proposing changes

If you’re not sure a change will be accepted, start with an issue describing:

- **Problem**: what’s broken or missing (include who it affects).
- **Proposal**: your suggested solution (alternatives welcome).
- **Acceptance criteria**: how we’ll know it’s done.

## Development workflow

DockerLens is built with **Tauri + Rust + React** on Linux. The package manager in CI and `tauri.conf.json` is **pnpm**.

### Prerequisites

- **Rust toolchain**: `rustup` with a stable toolchain.
- **Node.js**: 20 LTS (see CI) and **pnpm** 10+ (see `package.json` / lockfile).
- **Tauri system dependencies**: packages required by Tauri / WebKitGTK on your distro (see `README.md` for an Ubuntu example).

### Typical commands

From the repository root:

- **Install dependencies**: `pnpm install`
- **Run app (native window)**: `pnpm tauri dev`
- **Frontend only (Vite)**: `pnpm dev`
- **Production UI bundle**: `pnpm build`
- **Typecheck (frontend)**: `pnpm run lint`
- **Rust** (from `src-tauri/` or with `--manifest-path`): `cargo fmt`, `cargo clippy`, `cargo test`

If you change scripts or toolchain versions, update this file and `README.md` in the same PR.

## Coding standards (intended)

- **Formatting**
  - Rust: `rustfmt`
  - JS/TS: Prettier and ESLint once added to the repo (not wired yet)
- **Quality gates**
  - Keep warnings low; don’t introduce new lints.
  - Add tests where behavior changes (unit tests for logic, integration/e2e where appropriate).
- **Security**
  - Never commit secrets (tokens, credentials, `.env` files).
  - For security issues, see “Security” below.

## Git & pull requests

- **Branching**: create a topic branch from `main` (e.g. `feat/...`, `fix/...`, `docs/...`).
- **Commits**: write clear messages that explain *why* the change is needed.
- **PR description**: include summary + test plan.
- **Screenshots**: for UI changes, attach before/after screenshots or a short recording.

## Documentation changes

Small doc-only PRs are welcome and should be quick to review. Please keep language concise and consistent with `README.md`.

## Security

If you believe you’ve found a security vulnerability, **do not open a public issue**. Instead, open a private report through the repository’s security advisory flow (once enabled) or contact the maintainer privately.
