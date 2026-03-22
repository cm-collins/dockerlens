# AGENTS.md — Engineering Standards (Router)

## Task routing

- If the task is **not** changing code (e.g., questions, analysis, “how does X work?”, repo statistics like “count lines of code”, running read-only commands): keep it light — do the task without invoking heavy “change” gates.
- If the task **is** writing/modifying code, tests, configuration, or docs: apply the opt-in skill:
  - `.cursor/skills/senior-software-engineer/SKILL.md`

## Communication standard (always-on)

All agents must present work and explanations in **plain English** with a **layered, skimmable structure**: easy to scan in seconds, and still detailed enough to re-read later and fully understand.

Use the lightest structure that still makes the answer easy to skim. Not every conversation needs a full hierarchy.

When helpful (non-trivial tasks, investigations, or multi-step work), you can use sections like:

- **Answer / outcome**: the direct answer first (or a 1–2 line summary).
- **What I did**: 1–3 bullets (if work was performed).
- **Why**: the reason(s) in plain terms; define acronyms/jargon once if needed.
- **Impact**: who/what is affected (behavior, risk, compatibility).
- **Evidence**: concrete support (commands run + results, diffs, file/line refs, logs, screenshots, etc.).
- **Details (optional)**: deeper reasoning, edge cases, trade-offs, alternatives, and follow-ups.

Style requirements:

- Use **short headings + bullets**; avoid long unbroken paragraphs.
- Prefer **specific, testable statements** over vague claims (“should work”, “seems fine”).
- If an issue needs deeper explanation: include a **1–2 line summary first**, then the deeper detail below it (progressive disclosure).
- Emojis are allowed **when useful** (e.g., to improve scanability or tone), but use them **sparingly** and never as a substitute for clear text.

Document loading reliability:

- AGENTS or IDE's may sometimes fail to load nested/referenced documents.
- If you are instructed or you determine to rely on a doc (e.g. PRD/plan/architecture/best-practices etc) but cannot access it, **stop** and:
  - state which doc(s) are missing, and
  - ask the user to attach/share them (or provide the relevant excerpts) before proceeding.

---

## Minimal always-on safety rules

### 1) When making changes: smallest set, no unintended behavior change

Only when you are writing/modifying code, tests, config, or docs:

- Prioritise codebase stability; while you are free to perform wider changes if that is the best approach to accomplishing a user's request, be cautious about changes that could introduce unwanted / unrequested changes and regressions.
- Make the smallest set of changes that **correctly accomplishes what the user asked**.
- Broader sets of changes are allowed when, in your assessment, that broader scope is the best and most recommended way to accomplish the objective. If you do this, explicitly state:
  - what expanded (in/out of scope),
  - why the narrower change was worse, and
  - how you’ll control risk (verification steps, rollback plan if applicable).
- Preserve all other **externally observable behavior** unless the user explicitly asked to change it.
- If you go beyond a surgical patch, follow the “broader changes” rule above (state what expanded, why, and how risk is controlled).

### 2) Don’t leak secrets / sensitive data

- Never commit secrets.
- Don’t log tokens/keys/connection strings or PII unless explicitly required and approved.

### 3) Database schema verification lives in the skill

If the task is DB-related, apply `.cursor/skills/senior-software-engineer/SKILL.md` (it contains the “never assume schema” hard-stop and links).

---

## Operator notes (for humans & AI Agents modifying and maintaining this file)

- This file is intentionally **short** to keep default agent context small.
- It routes work into `.cursor/skills/...` so “change process” doesn’t apply to non-code tasks.
- `AGENTS.md` is also used as a stable **repo root marker** in tests/tooling — do not remove it.
- **Work tracking**: Use **GitHub Issues**, **PRs**, and the **GitHub Project “dockerlens”** — not Linear. Reference GitHub issue numbers (`#NNN`) or URLs. For `gh project` commands you may need `gh auth refresh -s read:project`. See `.cursor/README.md` for a short overview.

 - Please avoid doing the following whenever I ask for an explanation or documentation "in plain english":
 - Example: "What this script is for (in plain English):" 
 - I really hate it when you do this.