-- =============================================================================
-- DockerLens · 004_seed_dev.sql
-- Development seed data ONLY.
--
-- ⚠️  WARNING — NEVER RUN THIS IN PRODUCTION ⚠️
--
-- This file inserts test data for local development and testing.
-- It requires test users to already exist in auth.users.
-- Run ONLY against a local or development Supabase project.
-- Run in: Supabase Dashboard → SQL Editor → New Query
-- =============================================================================

-- Guard: refuse to run if this looks like production
-- Update this check to match your production project ref
DO $$
BEGIN
    IF current_setting('app.environment', TRUE) = 'production' THEN
        RAISE EXCEPTION
            'Refusing to run seed file in production environment. Aborting.'
            USING ERRCODE = 'insufficient_privilege';
    END IF;
END;
$$;

-- -----------------------------------------------------------------------------
-- Test preferences rows
-- Replace these UUIDs with real user IDs from your dev auth.users table.
-- Find them: SELECT id, email FROM auth.users;
-- -----------------------------------------------------------------------------

-- Insert test preferences — ON CONFLICT allows re-running safely
INSERT INTO public.user_preferences (
    id,
    theme,
    socket_path,
    notifications_enabled,
    start_on_login,
    dismissed_suggestions
)
VALUES
    -- Developer 1: dark theme, default socket, all defaults
    (
        '00000000-0000-0000-0000-000000000001',  -- replace with real UUID
        'dark',
        '',
        true,
        true,
        '{}'
    ),
    -- Developer 2: light theme, rootless Docker socket
    (
        '00000000-0000-0000-0000-000000000002',  -- replace with real UUID
        'light',
        '/home/devuser/.docker/run/docker.sock',
        false,
        false,
        ARRAY['stopped-container', 'unused-images']
    ),
    -- Developer 3: system theme, notifications off, some dismissed suggestions
    (
        '00000000-0000-0000-0000-000000000003',  -- replace with real UUID
        'system',
        '',
        false,
        true,
        ARRAY['cpu-spike']
    )
ON CONFLICT (id) DO UPDATE
    SET theme                 = EXCLUDED.theme,
        socket_path           = EXCLUDED.socket_path,
        notifications_enabled = EXCLUDED.notifications_enabled,
        start_on_login        = EXCLUDED.start_on_login,
        dismissed_suggestions = EXCLUDED.dismissed_suggestions;

-- -----------------------------------------------------------------------------
-- Verify seed data was inserted
-- -----------------------------------------------------------------------------

SELECT
    id,
    theme,
    socket_path,
    notifications_enabled,
    start_on_login,
    dismissed_suggestions,
    created_at
FROM public.user_preferences
ORDER BY created_at;
```

---

## Folder structure in the repo
```
dockerlens/
└── db/
    ├── README.md            ← execution order + rules
    ├── 001_schema.sql       ← tables, constraints, indexes
    ├── 002_rls.sql          ← RLS enable + all policies
    ├── 003_functions.sql    ← triggers + RPC functions
    └── 004_seed_dev.sql     ← dev seed data only