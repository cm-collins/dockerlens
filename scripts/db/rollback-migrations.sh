#!/usr/bin/env bash
# =============================================================================
# DockerLens — rollback-migrations.sh
# Rolls back all DockerLens migrations — drops tables and functions.
#
# ⚠️  DANGER: This PERMANENTLY DELETES all DockerLens database objects and data.
#     This action CANNOT be undone. Use only in dev or as a last resort.
#     "both" target is NOT supported — roll back one environment at a time.
#
# Usage:
#   ./db/rollback-migrations.sh                       # uses .env.local (dev)
#   ./db/rollback-migrations.sh --target prod         # uses DOCKERLENS_SUPABASE_DB_URL_PROD
#   ./db/rollback-migrations.sh --connection-uri "postgresql://..."
#
# Prerequisites:
#   - psql installed (sudo apt install postgresql-client)
#   - .env.local with DOCKERLENS_SUPABASE_DB_URL (dev) and/or
#     DOCKERLENS_SUPABASE_DB_URL_PROD (prod) set
# =============================================================================

set -euo pipefail

# ── Paths ────────────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ENV_FILE="$REPO_ROOT/.env.local"

# ── Config ───────────────────────────────────────────────────────────────────
CONN_URI_OVERRIDE=""
CLI_TARGET="dev"
AUTO_YES=false

# ── Helpers ──────────────────────────────────────────────────────────────────
log_info()  { echo "[INFO]  $*"; }
log_warn()  { echo "[WARN]  $*" >&2; }
log_error() { echo "[ERROR] $*" >&2; }
fatal()     { log_error "$*"; exit 1; }

mask_uri() {
    echo "$1" | sed 's|://[^:]*:[^@]*@|://****:****@|'
}

prompt_yes_no() {
    local answer
    read -rp "$1 [y/N] " answer
    [[ "${answer,,}" == "y" || "${answer,,}" == "yes" ]]
}

print_usage() {
    cat <<'USAGE'
Usage: rollback-migrations.sh [OPTIONS]

Rolls back all DockerLens migrations — drops all tables, triggers and functions.

⚠️  DANGER: PERMANENTLY DELETES all data. Cannot be undone.
    "both" is NOT supported — roll back one environment at a time.

Options:
  --target TARGET       Target environment: dev (default) or prod
  --connection-uri URI  Override .env.local with a specific PostgreSQL URI
  --yes, -y             Skip confirmation prompts (DANGEROUS)
  -h, --help            Show this help message

Examples:
  ./db/rollback-migrations.sh
  ./db/rollback-migrations.sh --target prod
USAGE
}

# ── Arg parsing ───────────────────────────────────────────────────────────────
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --target)
                [[ $# -ge 2 ]] || fatal "--target requires a value (dev|prod)"
                CLI_TARGET="$2"; shift 2 ;;
            --target=*)
                CLI_TARGET="${1#*=}"; shift ;;
            --connection-uri)
                [[ $# -ge 2 ]] || fatal "--connection-uri requires a value"
                CONN_URI_OVERRIDE="$2"; shift 2 ;;
            --connection-uri=*)
                CONN_URI_OVERRIDE="${1#*=}"; shift ;;
            --yes|-y)
                AUTO_YES=true; shift ;;
            -h|--help)
                print_usage; exit 0 ;;
            *)
                fatal "Unknown argument: $1. Use --help for usage." ;;
        esac
    done

    case "${CLI_TARGET,,}" in
        dev|prod) ;;
        both|all) fatal "'both' is not supported for rollback. Roll back one environment at a time." ;;
        *) fatal "Invalid --target '$CLI_TARGET'. Must be dev or prod." ;;
    esac
}

# ── Load connection URI ───────────────────────────────────────────────────────
load_conn_uri() {
    if [[ -n "$CONN_URI_OVERRIDE" ]]; then
        echo "$CONN_URI_OVERRIDE"
        return
    fi

    [[ -f "$ENV_FILE" ]] || fatal ".env.local not found at $ENV_FILE."

    local var_name uri
    if [[ "$CLI_TARGET" == "prod" ]]; then
        var_name="DOCKERLENS_SUPABASE_DB_URL_PROD"
    else
        var_name="DOCKERLENS_SUPABASE_DB_URL"
    fi

    uri=$(grep -E "^${var_name}=" "$ENV_FILE" | cut -d'=' -f2- | tr -d '"' | tr -d "'")

    [[ -n "$uri" ]] || fatal "$var_name is not set in $ENV_FILE."

    echo "$uri"
}

# ── Rollback SQL ──────────────────────────────────────────────────────────────
# Inline — no separate rollback files needed for a simple schema like ours.
# Order matters: drop triggers before functions, drop tables last.
ROLLBACK_SQL='
-- DockerLens rollback — removes all objects created by the migrations

-- 1. Triggers
DROP TRIGGER IF EXISTS on_user_preferences_updated ON public.user_preferences;
DROP TRIGGER IF EXISTS on_auth_user_created        ON auth.users;

-- 2. Functions
DROP FUNCTION IF EXISTS public.handle_updated_at()                                          CASCADE;
DROP FUNCTION IF EXISTS public.handle_new_user()                                            CASCADE;
DROP FUNCTION IF EXISTS public.get_user_preferences()                                       CASCADE;
DROP FUNCTION IF EXISTS public.upsert_user_preferences(TEXT, TEXT, BOOLEAN, BOOLEAN, TEXT[]) CASCADE;

-- 3. Tables (CASCADE drops dependent objects automatically)
DROP TABLE IF EXISTS public.user_preferences CASCADE;
'

# ── Execute rollback ──────────────────────────────────────────────────────────
execute_rollback() {
    local conn_uri="$1"

    log_info "Connecting to: $(mask_uri "$conn_uri")"
    echo ""

    if echo "$ROLLBACK_SQL" | psql "$conn_uri" -v ON_ERROR_STOP=1; then
        echo ""
        log_info "✓ Rollback completed — all DockerLens database objects removed."
        return 0
    else
        echo ""
        log_error "✗ Rollback failed. Database may be in a partial state."
        log_error "Review the error above and fix manually."
        return 1
    fi
}

# ── Main ──────────────────────────────────────────────────────────────────────
main() {
    parse_args "$@"

    echo ""
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║       ⚠️  DockerLens — ROLLBACK MIGRATIONS — DANGER  ⚠️        ║"
    echo "║   This will PERMANENTLY DELETE all tables and data!           ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo ""

    command -v psql >/dev/null 2>&1 || fatal "psql is not installed. Run: sudo apt install postgresql-client"

    local conn_uri
    conn_uri=$(load_conn_uri)

    echo "  Target   : $CLI_TARGET"
    echo "  Database : $(mask_uri "$conn_uri")"
    echo ""

    log_warn "This will PERMANENTLY DELETE:"
    log_warn "  - public.user_preferences (table + all data)"
    log_warn "  - handle_updated_at, handle_new_user (triggers + functions)"
    log_warn "  - get_user_preferences, upsert_user_preferences (RPC functions)"
    echo ""
    log_warn "This action CANNOT be undone!"
    echo ""

    if [[ "$AUTO_YES" != true ]]; then
        prompt_yes_no "Are you ABSOLUTELY SURE you want to rollback ${CLI_TARGET^^}?" \
            || { log_info "Rollback cancelled."; exit 0; }
        echo ""
        prompt_yes_no "Final confirmation — type 'yes' to confirm PERMANENT DELETION from ${CLI_TARGET^^}" \
            || { log_info "Rollback cancelled at final confirmation."; exit 0; }
        echo ""
    fi

    execute_rollback "$conn_uri"
}

main "$@"
