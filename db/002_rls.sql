-- =============================================================================
-- DockerLens · 002_rls.sql
-- Row Level Security policies for all public tables.
-- MUST be run after 001_schema.sql.
-- Safe to re-run — uses DROP POLICY IF EXISTS before CREATE.
-- Run in: Supabase Dashboard → SQL Editor → New Query
-- =============================================================================

-- =============================================================================
-- IMPORTANT SECURITY RULES
-- =============================================================================
-- 1. RLS is enabled on ALL public tables — no exceptions
-- 2. All policies use TO authenticated — anon role has NO access
-- 3. auth.uid() is wrapped in a subquery for performance
--    (prevents per-row evaluation — evaluated once per query instead)
-- 4. INSERT policies use WITH CHECK — UPDATE policies use USING + WITH CHECK
-- 5. service_role bypasses RLS by design — NEVER expose it in the frontend
-- =============================================================================

-- -----------------------------------------------------------------------------
-- 1. Enable RLS on all tables
-- -----------------------------------------------------------------------------

ALTER TABLE public.user_preferences ENABLE ROW LEVEL SECURITY;

-- Force RLS even for table owners (extra safety)
ALTER TABLE public.user_preferences FORCE ROW LEVEL SECURITY;

-- -----------------------------------------------------------------------------
-- 2. user_preferences policies
-- -----------------------------------------------------------------------------

-- Drop existing policies before recreating — makes this file idempotent
DROP POLICY IF EXISTS "users_select_own_preferences"  ON public.user_preferences;
DROP POLICY IF EXISTS "users_insert_own_preferences"  ON public.user_preferences;
DROP POLICY IF EXISTS "users_update_own_preferences"  ON public.user_preferences;
DROP POLICY IF EXISTS "users_delete_own_preferences"  ON public.user_preferences;

-- SELECT — users can only read their own row
CREATE POLICY "users_select_own_preferences"
    ON  public.user_preferences
    FOR SELECT
    TO  authenticated
    USING (
        id = (SELECT auth.uid())
    );

-- INSERT — users can only insert a row for themselves
-- WITH CHECK prevents inserting a row with a different user's ID
CREATE POLICY "users_insert_own_preferences"
    ON  public.user_preferences
    FOR INSERT
    TO  authenticated
    WITH CHECK (
        id = (SELECT auth.uid())
    );

-- UPDATE — users can only update their own row
-- USING: which rows can be targeted
-- WITH CHECK: what the row can be updated to
CREATE POLICY "users_update_own_preferences"
    ON  public.user_preferences
    FOR UPDATE
    TO  authenticated
    USING (
        id = (SELECT auth.uid())
    )
    WITH CHECK (
        id = (SELECT auth.uid())
    );

-- DELETE — users can only delete their own row
-- Note: deletion is normally handled by CASCADE from auth.users
-- This policy exists as a safety net for explicit deletes
CREATE POLICY "users_delete_own_preferences"
    ON  public.user_preferences
    FOR DELETE
    TO  authenticated
    USING (
        id = (SELECT auth.uid())
    );

-- -----------------------------------------------------------------------------
-- 3. Verification — confirm RLS is active and policies exist
-- -----------------------------------------------------------------------------

-- Run this after applying to verify everything is set up correctly:

-- Check RLS is enabled:
-- SELECT tablename, rowsecurity, forcerowsecurity
-- FROM pg_tables
-- WHERE schemaname = 'public';

-- Check all policies:
-- SELECT
--     schemaname,
--     tablename,
--     policyname,
--     permissive,
--     roles,
--     cmd,
--     qual,
--     with_check
-- FROM pg_policies
-- WHERE schemaname = 'public'
-- ORDER BY tablename, policyname;

-- Test as authenticated user (replace with real user UUID):
-- SET LOCAL role = authenticated;
-- SET LOCAL "request.jwt.claims" = '{"sub": "your-user-uuid-here"}';
-- SELECT * FROM public.user_preferences;
-- RESET role;