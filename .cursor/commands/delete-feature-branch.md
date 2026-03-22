# delete-feature-branch

Delete a feature branch locally and remotely **only after** verifying its PR was successfully merged into the repository’s **default branch** (this repo: **`main`**) and checks look good.

This command is intended to be run from the feature branch you want to delete.

---

## Hard rules (MANDATORY)

- **Do not delete without verification**: You MUST prove the branch’s PR was merged into the **integration branch** (the repo default branch, e.g. `main`) and that the merge commit is contained in `origin/<that branch>`.
- **Protected branches**: Never delete the default branch, `main`, `master`, `dev`, or any branch you cannot afford to lose.
- **Clean working tree**: If there are uncommitted changes, STOP and report (do not delete).
- **GitHub CLI auth required**: If `gh` is not logged in, STOP and instruct the user to log in (`gh auth login`), then re-run the command.
- **Use GitHub CLI**: Use `gh` to locate the PR and validate merge + checks.
- **Final confirmation required**: Before running any delete commands, you MUST clearly describe the delete actions you’re about to take and ask the user to confirm. If the user does not explicitly confirm in this conversation, DO NOT delete anything.
- **Be explicit**: If you do not delete (or only partially delete), explain exactly why and what to do next.

---

## Step 0: Identify branch to delete

```bash
BRANCH="$(git rev-parse --abbrev-ref HEAD)"
```

If `BRANCH` is `main`, `master`, `dev`, or `HEAD`, STOP.

Resolve the repo **integration branch** (branch PRs should merge into). Prefer GitHub’s default branch:

```bash
git fetch origin --prune
INTEGRATION_BRANCH="$(gh repo view --json defaultBranchRef --jq .defaultBranchRef.name)"
```

If that fails, fall back:

```bash
INTEGRATION_BRANCH="$(git symbolic-ref -q --short refs/remotes/origin/HEAD | sed 's#^origin/##')"
```

If `BRANCH` equals `INTEGRATION_BRANCH`, STOP.

---

## Step 1: Verify working tree is clean (MANDATORY)

```bash
git status --porcelain
```

If output is non-empty, STOP and report the exact reason (dirty working tree).

---

## Step 2: Verify GitHub CLI is logged in (MANDATORY)

```bash
gh auth status
```

If this fails (not logged in / not authenticated), STOP and tell the user to run `gh auth login`, then re-run `/delete-feature-branch`.

---

## Step 3: Verify merge into integration branch via GitHub (MANDATORY)

Locate the PR for this branch and load merge details:

```bash
PR_JSON="$(gh pr view --json url,state,merged,baseRefName,mergeCommit,statusCheckRollup 2>/dev/null || true)"
if [ -z "$PR_JSON" ] || [ "$PR_JSON" = "null" ]; then
  PR_JSON="$(gh pr list --head "$BRANCH" --state all --limit 1 --json url,state,merged,baseRefName,mergeCommit,statusCheckRollup --jq '.[0]' 2>/dev/null || true)"
fi
```

Validation criteria:

- PR exists
- `merged == true`
- `baseRefName` equals **`INTEGRATION_BRANCH`** (must match the default branch you resolved, unless the user explicitly named a different approved integration branch in chat)
- If `statusCheckRollup` is present: all required checks are successful (no failures/cancelled)

If any criterion fails, STOP and report:

- PR URL (if found)
- Current PR state/base
- Expected integration branch: `INTEGRATION_BRANCH`
- Any failing checks (name + conclusion)

---

## Step 4: Verify merge is actually in `origin/<INTEGRATION_BRANCH>` (MANDATORY)

Extract merge commit SHA from the PR JSON and verify it is an ancestor of the remote integration branch:

```bash
MERGE_SHA="$(echo "$PR_JSON" | jq -r '.mergeCommit.oid // empty')"
git fetch origin --prune
git merge-base --is-ancestor "$MERGE_SHA" "origin/$INTEGRATION_BRANCH"
```

If this fails, STOP and report:

- Merge commit SHA you checked
- `origin/$INTEGRATION_BRANCH` tip SHA
- Why deletion is unsafe (merge not present yet)

---

## Step 5: Final confirmation gate (MANDATORY)

Before deleting anything, you MUST print a short plan like this and then STOP to wait for explicit confirmation:

- Branch to delete (remote + local): `<BRANCH>`
- PR: `<url>`
- Integration branch: `<INTEGRATION_BRANCH>`
- Merge commit verified in `origin/<INTEGRATION_BRANCH>`: ✅
- Actions you are about to run:
  - `git checkout <INTEGRATION_BRANCH>` and `git pull --ff-only`
  - `git push origin --delete "<BRANCH>"`
  - `git branch -D "<BRANCH>"`

Then ask the user to reply with the exact phrase:

`CONFIRM DELETE <BRANCH>`

If the user does not reply with that exact phrase, DO NOT delete anything.

---

## Step 6: Delete branch (ONLY AFTER EXPLICIT USER CONFIRMATION)

1. Switch away from the branch (so local deletion succeeds):

```bash
git checkout "$INTEGRATION_BRANCH"
git pull --ff-only
```

2. Delete remote branch (idempotent):

```bash
git push origin --delete "$BRANCH" || true
```

3. Delete local branch:

```bash
git branch -D "$BRANCH"
```

4. Verify:

```bash
git branch --show-current
git branch --list "$BRANCH" || true
git ls-remote --heads origin "$BRANCH" || true
```

---

## Output (MANDATORY)

Return a short, plain-English summary:

- Branch deleted: `<branch>`
- PR: `<url>`
- Merge commit: `<sha>`
- Verified in `origin/<INTEGRATION_BRANCH>`: ✅/❌
- Remote deletion: ✅/❌
- Local deletion: ✅/❌
