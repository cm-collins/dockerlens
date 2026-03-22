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
-- Inserts a default preferences row for every existing user in auth.users.
-- Safe to re-run — ON CONFLICT DO NOTHING skips users that already have a row.
-- If auth.users is empty, this is a no-op.
-- -----------------------------------------------------------------------------

INSERT INTO public.user_preferences (id)
SELECT id FROM auth.users
ON CONFLICT (id) DO NOTHING;

-- -----------------------------------------------------------------------------
-- Verify seed data
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