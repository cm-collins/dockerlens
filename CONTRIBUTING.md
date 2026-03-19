 
## Contributing to dockerlens

Thanks for helping make **dockerlens** better.

At the moment, this repository mostly contains community and project metadata (no app source code checked in yet). Contributions are still very welcome—especially around **planning, design, documentation, and project setup**. As the codebase lands, this guide also describes the intended dev workflow.

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

## Development workflow (once the app code is present)

dockerlens is intended to be built with **Tauri + Rust + React** on Linux.

### Prerequisites

- **Rust toolchain**: `rustup` with a stable toolchain.
- **Node.js**: LTS recommended (plus your preferred package manager).
- **Tauri system dependencies**: platform packages required by Tauri/WebKitGTK on Linux.

### Typical commands

Exact commands will be documented in the repo once the code is added, but the expected shape is:

- **Install dependencies**: `npm install` (or `pnpm install` / `yarn`)
- **Run dev**: `npm run dev` (or `npm run tauri dev`)
- **Build**: `npm run build` (or `npm run tauri build`)
- **Rust checks**: `cargo fmt`, `cargo clippy`, `cargo test`
- **Frontend checks**: `npm run lint`, `npm run format`, `npm test`

If you add the initial codebase and these commands differ, please update this file in the same PR.

## Coding standards (intended)

- **Formatting**
  - Rust: `rustfmt`
  - JS/TS: Prettier (and ESLint)
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
