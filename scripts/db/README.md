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
- Run `validate-migrations.sh` before `apply-migrations.sh`
- Test in a dev Supabase project before running in production

## Scripts

| Script | Purpose |
|---|---|
| `validate-migrations.sh` | Dry-run — validates all migrations in a rolled-back transaction |
| `apply-migrations.sh` | Applies migrations permanently to dev or prod |
| `rollback-migrations.sh` | Drops all tables and functions — destructive, use with caution |

### Prerequisites

```bash
# Install psql client
sudo apt install postgresql-client

# Add to .env.local
DOCKERLENS_SUPABASE_DB_URL=postgresql://postgres.<ref>:<password>@aws-0-<region>.pooler.supabase.com:5432/postgres
```

Get the connection string from: **Supabase Dashboard → Project → Settings → Database → Connection string (URI mode)**

### Usage

```bash
# 1. Validate first (safe — no changes applied)
./db/validate-migrations.sh

# 2. Apply to dev
./db/apply-migrations.sh

# 3. Apply to prod
./db/apply-migrations.sh --target prod

# Rollback dev (destructive!)
./db/rollback-migrations.sh
```
