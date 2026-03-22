# oversee

Use an **in-session** Product Owner / architect gatekeeper pass (no separate subagent file required in this repo).

Use `/oversee` when you want a second opinion that:

- Validates work against **docs** (especially `docs/Requirements/PRD.md` + `docs/Requirements/TRD.md`) unless you name another source of truth
- Checks alignment with **GitHub tracking** (issues, Project items) when you reference them
- Returns a crisp **APPROVE / BLOCK / NEEDS-INFO** verdict with concrete next steps

This project does **not** use Linear. Track work in the **GitHub Project** named **dockerlens** (and GitHub Issues/PRs). Reference **`#123`**-style issues or paste Project/issue links instead of external ticket keys.

---

## How to run (MANDATORY)

1. Identify the decision. Prefer one of:
   - “Is this ready to merge?”
   - “Does this match the PRD / TRD?”
   - “Did we stay within the intended scope?”
   - “What are the hidden risks / doc drift?”

2. Provide minimal context:
   - Path(s) or sections in `docs/` that define expected behavior (required when the change is product-facing)
   - **GitHub**: issue URL or `#NNN`, or the Project item title/link if relevant
   - PR URL or `BASE..HEAD` / branch name if reviewing a branch
   - Paths under `Output/CodeReview/` only if you use the parallel audit workflow and have artifacts there

3. The agent performing `/oversee` must output:

   - **Verdict**: APPROVE / BLOCK / NEEDS-INFO
   - **Why** (short)
   - **Required changes** or **minimum questions** (if not APPROVE)

Communication: simple, clear English; do not use headings that say “plain English” (per `AGENTS.md`).

---

## Output

- **Verdict**: APPROVE / BLOCK / NEEDS-INFO
- A short “why”
- Required changes or minimum questions (if needed)
