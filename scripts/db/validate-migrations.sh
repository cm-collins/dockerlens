#!/usr/bin/env bash
# =============================================================================
# DockerLens — validate-migrations.sh
# Dry-run validation: runs all migrations inside a transaction that is
# rolled back at the end. No changes are persisted.
#
# How it works:
#   1. Reads DOCKERLENS_SUPABASE_DB_URL from .env.local (or --connection-uri)
#   2. Starts a transaction
#   3. Runs all migration files in order
#   4. Rolls back — nothing is committed
#   5. Reports success / failure
#
# Usage:
#   ./db/validate-migrations.sh
#   ./db/validate-migrations.sh --connection-uri "postgresql://..."
#   ./db/validate-migrations.sh --verbose
#
# Prerequisites:
#   - psql installed (sudo apt install postgresql-client)
#   - .env.local with DOCKERLENS_SUPABASE_DB_URL set
#     (Supabase Dashboard → Project → Settings → Database → Connection string)
# =============================================================================

set -euo pipefail

# ── Paths ────────────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
MIGRATIONS_DIR="$SCRIPT_DIR"
ENV_FILE="$REPO_ROOT/.env.local"

# ── Config ───────────────────────────────────────────────────────────────────
CONN_URI_OVERRIDE=""
VERBOSE=false

# ── Helpers ──────────────────────────────────────────────────────────────────
log_info()  { echo "[INFO]  $*"; }
log_warn()  { echo "[WARN]  $*" >&2; }
log_error() { echo "[ERROR] $*" >&2; }
fatal()     { log_error "$*"; exit 1; }

mask_uri() {
    # Replace password in URI with ****
    echo "$1" | sed 's|://[^:]*:[^@]*@|://****:****@|'
}

prompt_yes_no() {
    local answer
    read -rp "$1 [y/N] " answer
    [[ "${answer,,}" == "y" || "${answer,,}" == "yes" ]]
}

print_usage() {
    cat <<'USAGE'
Usage: validate-migrations.sh [OPTIONS]

Validates migration scripts by running them in a transaction that is rolled back.
No changes are applied to the database.

Options:
  --connection-uri URI  Override .env.local with a specific PostgreSQL URI
                        Format: postgresql://postgres.<ref>:<password>@<host>:5432/postgres
  --verbose             Show full SQL output during validation
  -h, --help            Show this help message

Examples:
  ./db/validate-migrations.sh
  ./db/validate-migrations.sh --verbose
  ./db/validate-migrations.sh --connection-uri "postgresql://postgres.abc:secret@aws-0-eu-west-1.pooler.supabase.com:5432/postgres"
USAGE
}

# ── Arg parsing ───────────────────────────────────────────────────────────────
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --connection-uri)
                [[ $# -ge 2 ]] || fatal "--connection-uri requires a value"
                CONN_URI_OVERRIDE="$2"; shift 2 ;;
            --connection-uri=*)
                CONN_URI_OVERRIDE="${1#*=}"; shift ;;
            --verbose)
                VERBOSE=true; shift ;;
            -h|--help)
                print_usage; exit 0 ;;
            *)
                fatal "Unknown argument: $1. Use --help for usage." ;;
        esac
    done
}

# ── Load connection URI ───────────────────────────────────────────────────────
load_conn_uri() {
    if [[ -n "$CONN_URI_OVERRIDE" ]]; then
        echo "$CONN_URI_OVERRIDE"
        return
    fi

    [[ -f "$ENV_FILE" ]] || fatal ".env.local not found at $ENV_FILE. Copy .env.local.example and fill in DOCKERLENS_SUPABASE_DB_URL."

    local uri
    uri=$(grep -E '^DOCKERLENS_SUPABASE_DB_URL=' "$ENV_FILE" | cut -d'=' -f2- | tr -d '"' | tr -d "'")

    [[ -n "$uri" ]] || fatal "DOCKERLENS_SUPABASE_DB_URL is not set in $ENV_FILE.
Get it from: Supabase Dashboard → Project → Settings → Database → Connection string (URI mode)"

    echo "$uri"
}

# ── Get migration files ───────────────────────────────────────────────────────
get_migration_files() {
    find "$MIGRATIONS_DIR" -maxdepth 1 -name "*.sql" -type f | sort -V
}

# ── Validate ──────────────────────────────────────────────────────────────────
validate_migrations() {
    local conn_uri="$1"

    local -a migrations
    mapfile -t migrations < <(get_migration_files)

    [[ ${#migrations[@]} -gt 0 ]] || fatal "No .sql files found in $MIGRATIONS_DIR"

    log_info "Found ${#migrations[@]} migration file(s):"
    for m in "${migrations[@]}"; do echo "  - $(basename "$m")"; done
    echo ""

    # Build combined script: BEGIN + all files + ROLLBACK
    local temp_script
    temp_script=$(mktemp /tmp/dockerlens-validate-XXXXXX.sql)
    trap "rm -f '$temp_script'" EXIT

    {
        echo "BEGIN;"
        echo ""
        for migration in "${migrations[@]}"; do
            local filename
            filename=$(basename "$migration")
            echo "-- >>> $filename"
            echo "\\i ${migration}"
            echo ""
        done
        echo "-- Dry-run complete — rolling back"
        echo "ROLLBACK;"
    } > "$temp_script"

    if [[ "$VERBOSE" == true ]]; then
        log_info "Combined script:"
        echo "---"
        cat "$temp_script"
        echo "---"
        echo ""
    fi

    log_info "Connecting to: $(mask_uri "$conn_uri")"
    echo ""

    local psql_opts=("-v" "ON_ERROR_STOP=1" "-f" "$temp_script")
    [[ "$VERBOSE" != true ]] && psql_opts+=("--quiet")

    if psql "$conn_uri" "${psql_opts[@]}"; then
        echo ""
        log_info "✓ All migrations validated successfully — no changes applied."
        return 0
    else
        echo ""
        log_error "✗ Validation failed. Review the error above — no changes were applied."
        return 1
    fi
}

# ── Main ──────────────────────────────────────────────────────────────────────
main() {
    parse_args "$@"

    echo ""
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║         DockerLens — Migration Dry-Run Validation             ║"
    echo "║   Runs migrations in a rolled-back transaction (safe)         ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo ""

    command -v psql >/dev/null 2>&1 || fatal "psql is not installed. Run: sudo apt install postgresql-client"
    log_info "psql: $(psql --version)"
    echo ""

    local conn_uri
    conn_uri=$(load_conn_uri)

    echo "  Migrations dir : $MIGRATIONS_DIR"
    echo "  Database       : $(mask_uri "$conn_uri")"
    echo ""

    validate_migrations "$conn_uri"
}

main "$@"
