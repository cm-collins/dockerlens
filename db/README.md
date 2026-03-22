# DockerLens — Database

This folder contains all SQL migrations for the DockerLens Supabase project.
Each file is self-contained and idempotent — safe to run multiple times.

## Execution Order

Run these files **in order** by pasting each into the Supabase SQL Editor:
**Dashboard → Project → SQL Editor → New Query**

| Order | File | Purpose |
|---|---|---|
| 1 | `001_schema.sql` | Create tables with constraints and indexes |
| 2 | `002_rls.sql` | Enable RLS and create all security policies |
| 3 | `003_functions.sql` | Functions, triggers and automation |
| 4 | `004_seed_dev.sql` | Development seed data — **never run in production** |

## Rules

- Never modify a file after it has been run in production
- Never run `004_seed_dev.sql` against a production project
- All files use `IF NOT EXISTS` / `IF EXISTS` — safe to re-run
- Test in a dev Supabase project before running in production
