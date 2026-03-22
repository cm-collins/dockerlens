# Cursor config for DockerLens

This folder holds **commands**, **prompts**, and a **skill** used in Cursor for this repo. Paths are relative to the repository root.

---

## Project tracking (source of truth)

- **Not used here**: Linear, `SHI-###` tickets, or `/implement-shi`-style Linear workflows.
- **Used here**: **GitHub Issues**, **Pull Requests**, and the **dockerlens** project board: [github.com/users/cm-collins/projects/1](https://github.com/users/cm-collins/projects/1). Repo: [`cm-collins/dockerlens`](https://github.com/cm-collins/dockerlens).

**GitHub CLI**

- Issues & PRs: `gh issue`, `gh pr`
- Projects: `gh auth refresh -s read:project` (once), then e.g. `gh project list --owner cm-collins` or  
  `gh project list --owner cm-collins --format json -q '.projects[] | select(.title=="dockerlens") | .url'`  
  (JSON root is `{"projects":[...]}`; see `README.md` → Roadmap.)

Link work in PR descriptions with `Closes #123` or explicit Project item URLs as you prefer.

---

## Commands (`.cursor/commands/`)

These are the command specs checked into **this** repo:

| Command | Purpose |
|--------|---------|
| `/commit` | Commit **already-staged** changes via `COMMIT_MESSAGE.txt` workflow. |
| `/pr` | Create or update a PR with `gh` and `PR_MESSAGE.txt`. |
| `/delete-feature-branch` | After merge to **`main`** (default branch), safely delete the feature branch with verification + explicit user confirmation. |
| `/oversee` | PO/architect-style **APPROVE / BLOCK / NEEDS-INFO** pass against `docs/` + your stated GitHub issue/Project context. |

If your Cursor UI lists other slash commands (e.g. `/review`, `/implement-shi`), they are **not** defined under `.cursor/commands/` here unless you add matching files—do not assume they exist.

---

## Prompts (`.cursor/prompts/`)

Helper text for deeper audits or fix workflows (invoke manually or from custom commands you define):

- `documentation-drift-audit-prompt.md`
- `general-code-audit-review-prompt.md`
- `code-review-validation-and-fix.md`

These are aligned to **DockerLens** paths under `docs/` (see `general-code-audit-review-prompt.md` Step 0).

---

## Skills (`.cursor/skills/`)

- `senior-software-engineer/SKILL.md` — pre-flight checklist for code/test/config/doc changes; **Tauri + Rust + React** verification commands (`pnpm`, `cargo`).

**Repo-wide agent router**: see `AGENTS.md` at the repository root.
