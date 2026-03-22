-- =============================================================================
-- DockerLens · 001_schema.sql
-- Creates all tables with constraints, indexes and comments.
-- Safe to re-run — uses IF NOT EXISTS throughout.
-- Run in: Supabase Dashboard → SQL Editor → New Query
-- =============================================================================

-- -----------------------------------------------------------------------------
-- 0. Extensions
-- -----------------------------------------------------------------------------

-- uuid_generate_v4() — used as default PK generator
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- pgcrypto — used for secure token generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- -----------------------------------------------------------------------------
-- 1. user_preferences
--    One row per authenticated user.
--    Stores all syncable app preferences.
--    Primary key references auth.users — automatically cleaned up on user delete.
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS public.user_preferences (

    -- Identity
    -- Linked directly to auth.users — no orphaned rows possible
    id                      UUID        PRIMARY KEY
                            REFERENCES  auth.users(id)
                            ON DELETE   CASCADE,

    -- Appearance
    -- Constrained to valid values at DB level — not just application level
    theme                   TEXT        NOT NULL DEFAULT 'dark'
                            CHECK       (theme IN ('dark', 'light', 'system')),

    -- Connection
    -- Empty string means "use auto-detected default"
    socket_path             TEXT        NOT NULL DEFAULT ''
                            CHECK       (length(socket_path) <= 512),

    -- Notifications
    notifications_enabled   BOOLEAN     NOT NULL DEFAULT true,

    -- Startup
    start_on_login          BOOLEAN     NOT NULL DEFAULT true,

    -- Suggestions
    -- Array of suggestion IDs the user has dismissed
    -- e.g. ARRAY['stopped-container', 'unused-image']
    dismissed_suggestions   TEXT[]      NOT NULL DEFAULT '{}',

    -- Timestamps
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- Constraints
    CONSTRAINT dismissed_suggestions_max_length
        CHECK (array_length(dismissed_suggestions, 1) IS NULL
            OR array_length(dismissed_suggestions, 1) <= 100)

);

-- Comments — visible in Supabase Table Editor
COMMENT ON TABLE  public.user_preferences                      IS 'Syncable app preferences per authenticated user. One row per user.';
COMMENT ON COLUMN public.user_preferences.id                   IS 'References auth.users.id — row deleted automatically when user is deleted.';
COMMENT ON COLUMN public.user_preferences.theme                IS 'UI theme preference. One of: dark, light, system.';
COMMENT ON COLUMN public.user_preferences.socket_path          IS 'Custom Docker socket path override. Empty string = use auto-detected default.';
COMMENT ON COLUMN public.user_preferences.notifications_enabled IS 'Whether desktop notifications are enabled.';
COMMENT ON COLUMN public.user_preferences.start_on_login       IS 'Whether Docker daemon should start on OS login.';
COMMENT ON COLUMN public.user_preferences.dismissed_suggestions IS 'Array of suggestion IDs the user has permanently dismissed.';
COMMENT ON COLUMN public.user_preferences.created_at           IS 'Row creation timestamp — set once, never updated.';
COMMENT ON COLUMN public.user_preferences.updated_at           IS 'Last update timestamp — maintained by trigger.';

-- -----------------------------------------------------------------------------
-- 2. Indexes
--    user_preferences.id is a PK — already indexed.
--    No additional indexes needed for v1.0 — single-row-per-user access pattern.
-- -----------------------------------------------------------------------------

-- Index for updated_at — useful for sync queries ("give me rows updated after X")
CREATE INDEX IF NOT EXISTS idx_user_preferences_updated_at
    ON public.user_preferences (updated_at DESC);

-- -----------------------------------------------------------------------------
-- Verification query — run after creation to confirm structure
-- -----------------------------------------------------------------------------
-- SELECT
--     column_name,
--     data_type,
--     column_default,
--     is_nullable,
--     character_maximum_length
-- FROM information_schema.columns
-- WHERE table_schema = 'public'
--   AND table_name   = 'user_preferences'
-- ORDER BY ordinal_position;