#!/usr/bin/env bash
# =============================================================================
# DockerLens — apply-migrations.sh
# Applies migration scripts permanently to the target Supabase database.
#
# ⚠️  WARNING: This makes PERMANENT changes to the database.
#     Run validate-migrations.sh first to dry-run test before applying.
#
# Usage:
#   ./db/apply-migrations.sh                          # uses .env.local
#   ./db/apply-migrations.sh --target prod            # uses DOCKERLENS_SUPABASE_DB_URL_PROD
#   ./db/apply-migrations.sh --target dev             # uses DOCKERLENS_SUPABASE_DB_URL (default)
#   ./db/apply-migrations.sh --connection-uri "postgresql://..."
#   ./db/apply-migrations.sh --yes                    # skip confirmation prompt
#
# Prerequisites:
#   - psql installed (sudo apt install postgresql-client)
#   - .env.local with DOCKERLENS_SUPABASE_DB_URL (dev) and/or
#     DOCKERLENS_SUPABASE_DB_URL_PROD (prod) set
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
Usage: apply-migrations.sh [OPTIONS]

Applies migration scripts permanently to the target Supabase database.

⚠️  WARNING: This makes PERMANENT changes. Run validate-migrations.sh first.

Options:
  --target TARGET       Target environment: dev (default) or prod
  --connection-uri URI  Override .env.local with a specific PostgreSQL URI
  --yes, -y             Skip confirmation prompt
  -h, --help            Show this help message

Environment variables (in .env.local):
  DOCKERLENS_SUPABASE_DB_URL       Dev database connection URI
  DOCKERLENS_SUPABASE_DB_URL_PROD  Prod database connection URI

Examples:
  ./db/apply-migrations.sh
  ./db/apply-migrations.sh --target prod
  ./db/apply-migrations.sh --connection-uri "postgresql://postgres.abc:secret@host:5432/postgres"
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

    [[ -n "$uri" ]] || fatal "$var_name is not set in $ENV_FILE.
Get it from: Supabase Dashboard → Project → Settings → Database → Connection string (URI mode)"

    echo "$uri"
}

# ── Get migration files ───────────────────────────────────────────────────────
get_migration_files() {
    find "$MIGRATIONS_DIR" -maxdepth 1 -name "*.sql" -type f | sort -V
}

# ── Apply ─────────────────────────────────────────────────────────────────────
apply_migrations() {
    local conn_uri="$1"

    local -a migrations
    mapfile -t migrations < <(get_migration_files)

    [[ ${#migrations[@]} -gt 0 ]] || fatal "No .sql files found in $MIGRATIONS_DIR"

    log_info "Applying ${#migrations[@]} migration file(s):"
    for m in "${migrations[@]}"; do echo "  - $(basename "$m")"; done
    echo ""

    local failed=false
    for migration in "${migrations[@]}"; do
        local filename
        filename=$(basename "$migration")
        log_info "Applying: $filename"

        if psql "$conn_uri" -v ON_ERROR_STOP=1 -f "$migration"; then
            log_info "  ✓ $filename"
        else
            log_error "  ✗ $filename failed"
            failed=true
            break
        fi
    done

    echo ""
    if [[ "$failed" == true ]]; then
        log_error "Migration failed. Database may be in a partial state."
        log_error "Review the error above and fix before retrying."
        return 1
    else
        log_info "✓ All migrations applied successfully."
        return 0
    fi
}

# ── Main ──────────────────────────────────────────────────────────────────────
main() {
    parse_args "$@"

    echo ""
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║            DockerLens — Apply Migrations                      ║"
    echo "║   ⚠️  WARNING: PERMANENT changes to the database              ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo ""

    command -v psql >/dev/null 2>&1 || fatal "psql is not installed. Run: sudo apt install postgresql-client"
    log_info "psql: $(psql --version)"
    echo ""

    local conn_uri
    conn_uri=$(load_conn_uri)

    echo "  Target         : $CLI_TARGET"
    echo "  Migrations dir : $MIGRATIONS_DIR"
    echo "  Database       : $(mask_uri "$conn_uri")"
    echo ""
    echo "  TIP: Run validate-migrations.sh first to dry-run test."
    echo ""

    if [[ "$AUTO_YES" != true ]]; then
        log_warn "This will PERMANENTLY apply migrations to the $CLI_TARGET database."
        prompt_yes_no "Are you sure you want to proceed?" || { log_info "Cancelled."; exit 0; }
        echo ""
    fi

    apply_migrations "$conn_uri"
}

main "$@"
