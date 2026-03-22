-- =============================================================================
-- DockerLens · 003_functions.sql
-- Functions, triggers and automation.
-- MUST be run after 001_schema.sql and 002_rls.sql.
-- Safe to re-run — uses CREATE OR REPLACE throughout.
-- Run in: Supabase Dashboard → SQL Editor → New Query
-- =============================================================================

-- -----------------------------------------------------------------------------
-- 1. updated_at trigger function
--    Automatically sets updated_at = now() on every UPDATE.
--    Shared by all tables that have an updated_at column.
-- -----------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
-- Runs with the privileges of the function owner (postgres), not the caller.
-- Required because the trigger fires internally and doesn't need caller auth.
SET search_path = public
AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$;

COMMENT ON FUNCTION public.handle_updated_at()
    IS 'Trigger function — automatically updates updated_at column on row update.';

-- Attach trigger to user_preferences
-- DROP first to make this idempotent
DROP TRIGGER IF EXISTS on_user_preferences_updated
    ON public.user_preferences;

CREATE TRIGGER on_user_preferences_updated
    BEFORE UPDATE ON public.user_preferences
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_updated_at();

-- -----------------------------------------------------------------------------
-- 2. Auto-create user_preferences on signup
--    Fires after a new user is inserted into auth.users.
--    Creates a default preferences row automatically.
--    Uses ON CONFLICT DO NOTHING — safe to fire multiple times.
-- -----------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
-- SECURITY DEFINER required — this function writes to public.user_preferences
-- on behalf of a newly created user who doesn't yet have a session.
SET search_path = public
AS $$
BEGIN
    INSERT INTO public.user_preferences (id)
    VALUES (NEW.id)
    ON CONFLICT (id) DO NOTHING;
    -- ON CONFLICT: safe re-entry if trigger fires more than once for same user
    RETURN NEW;
END;
$$;

COMMENT ON FUNCTION public.handle_new_user()
    IS 'Trigger function — creates a default user_preferences row when a new auth user signs up.';

-- Attach trigger to auth.users
-- Note: triggers on auth schema tables require superuser privileges
-- This runs automatically in Supabase — auth schema is accessible
DROP TRIGGER IF EXISTS on_auth_user_created
    ON auth.users;

CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_new_user();

-- -----------------------------------------------------------------------------
-- 3. get_user_preferences(user_uuid)
--    Convenience RPC function — fetches preferences for the calling user.
--    Callable from supabase-js: supabase.rpc('get_user_preferences')
--    Returns NULL if no row exists (shouldn't happen after trigger is set up).
-- -----------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION public.get_user_preferences()
RETURNS public.user_preferences
LANGUAGE sql
SECURITY DEFINER
STABLE
-- STABLE: no side effects, same input = same output within a transaction
SET search_path = public
AS $$
    SELECT *
    FROM   public.user_preferences
    WHERE  id = auth.uid()
    LIMIT  1;
$$;

COMMENT ON FUNCTION public.get_user_preferences()
    IS 'RPC — returns the preferences row for the currently authenticated user.';

-- Grant execute to authenticated users
REVOKE ALL    ON FUNCTION public.get_user_preferences() FROM PUBLIC;
GRANT  EXECUTE ON FUNCTION public.get_user_preferences() TO  authenticated;

-- -----------------------------------------------------------------------------
-- 4. upsert_user_preferences(theme, socket_path, ...)
--    Upserts the calling user's preferences in one RPC call.
--    Callable from supabase-js: supabase.rpc('upsert_user_preferences', {...})
--    Returns the updated row.
-- -----------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION public.upsert_user_preferences(
    p_theme                   TEXT        DEFAULT NULL,
    p_socket_path             TEXT        DEFAULT NULL,
    p_notifications_enabled   BOOLEAN     DEFAULT NULL,
    p_start_on_login          BOOLEAN     DEFAULT NULL,
    p_dismissed_suggestions   TEXT[]      DEFAULT NULL
)
RETURNS public.user_preferences
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
    v_user_id   UUID := auth.uid();
    v_result    public.user_preferences;
BEGIN
    -- Reject unauthenticated calls
    IF v_user_id IS NULL THEN
        RAISE EXCEPTION 'Not authenticated'
            USING ERRCODE = 'insufficient_privilege';
    END IF;

    -- Validate theme if provided
    IF p_theme IS NOT NULL AND p_theme NOT IN ('dark', 'light', 'system') THEN
        RAISE EXCEPTION 'Invalid theme value: %. Must be dark, light or system.', p_theme
            USING ERRCODE = 'check_violation';
    END IF;

    -- Validate socket_path length if provided
    IF p_socket_path IS NOT NULL AND length(p_socket_path) > 512 THEN
        RAISE EXCEPTION 'socket_path exceeds maximum length of 512 characters'
            USING ERRCODE = 'check_violation';
    END IF;

    -- Upsert — insert on first call, update on subsequent calls
    INSERT INTO public.user_preferences (
        id,
        theme,
        socket_path,
        notifications_enabled,
        start_on_login,
        dismissed_suggestions
    )
    VALUES (
        v_user_id,
        COALESCE(p_theme,                 'dark'),
        COALESCE(p_socket_path,           ''),
        COALESCE(p_notifications_enabled, true),
        COALESCE(p_start_on_login,        true),
        COALESCE(p_dismissed_suggestions, '{}')
    )
    ON CONFLICT (id) DO UPDATE
        SET theme                   = COALESCE(p_theme,                 user_preferences.theme),
            socket_path             = COALESCE(p_socket_path,           user_preferences.socket_path),
            notifications_enabled   = COALESCE(p_notifications_enabled, user_preferences.notifications_enabled),
            start_on_login          = COALESCE(p_start_on_login,        user_preferences.start_on_login),
            dismissed_suggestions   = COALESCE(p_dismissed_suggestions, user_preferences.dismissed_suggestions)
            -- updated_at is handled automatically by the trigger
    RETURNING *
    INTO v_result;

    RETURN v_result;
END;
$$;

COMMENT ON FUNCTION public.upsert_user_preferences(TEXT, TEXT, BOOLEAN, BOOLEAN, TEXT[])
    IS 'RPC — upserts preferences for the calling user. Pass only the fields to update — others are left unchanged. Returns the updated row.';

-- Grant execute to authenticated users
REVOKE ALL     ON FUNCTION public.upsert_user_preferences(TEXT, TEXT, BOOLEAN, BOOLEAN, TEXT[]) FROM PUBLIC;
GRANT  EXECUTE ON FUNCTION public.upsert_user_preferences(TEXT, TEXT, BOOLEAN, BOOLEAN, TEXT[]) TO  authenticated;

-- -----------------------------------------------------------------------------
-- 5. Verification — confirm functions and triggers exist
-- -----------------------------------------------------------------------------

-- Check triggers:
-- SELECT trigger_name, event_manipulation, event_object_table, action_timing
-- FROM information_schema.triggers
-- WHERE trigger_schema IN ('public', 'auth')
-- ORDER BY event_object_table;

-- Check functions:
-- SELECT routine_name, routine_type, security_type
-- FROM information_schema.routines
-- WHERE routine_schema = 'public'
--   AND routine_name IN (
--       'handle_updated_at',
--       'handle_new_user',
--       'get_user_preferences',
--       'upsert_user_preferences'
--   );

-- Test upsert RPC (authenticated context required):
-- SELECT * FROM public.upsert_user_preferences(
--     p_theme  := 'dark',
--     p_notifications_enabled := true
-- );