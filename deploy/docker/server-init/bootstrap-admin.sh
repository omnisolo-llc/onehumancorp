#!/bin/sh
# bootstrap-admin.sh — One-time admin account initialisation.
#
# This script is executed by the `server-init` Docker Compose service on every
# container start-up.  A marker file written to a named volume (/data) ensures
# the actual account-creation logic runs only ONCE, during the very first
# installation.  Subsequent restarts exit immediately after detecting the marker.
#
# Environment variables consumed:
#   SERVER_URL                  — base URL of the OHC server (default: http://server:8080)
#   SETUP_ADMIN_INIT_USERNAME   — desired admin username          (default: admin)
#   SETUP_ADMIN_INIT_PASSWORD   — desired admin password          (default: admin)

set -eu

MARKER="/data/.admin_initialized"
SERVER_URL="${SERVER_URL:-http://server:8080}"
USERNAME="${SETUP_ADMIN_INIT_USERNAME:-admin}"
PASSWORD="${SETUP_ADMIN_INIT_PASSWORD:-admin}"
MAX_RETRIES=60
RETRY_INTERVAL=5

# ── Guard: only run once ──────────────────────────────────────────────────────
if [ -f "$MARKER" ]; then
    echo "[bootstrap] Admin already initialised (marker found). Skipping."
    exit 0
fi

# ── Wait for the server to be reachable ──────────────────────────────────────
echo "[bootstrap] Waiting for server at ${SERVER_URL} ..."
count=0
until wget -q --spider --tries=1 --timeout=4 "${SERVER_URL}/readyz" >/dev/null 2>&1; do
    count=$((count + 1))
    if [ "$count" -ge "$MAX_RETRIES" ]; then
        echo "[bootstrap] ERROR: Server did not become ready after $((MAX_RETRIES * RETRY_INTERVAL))s. Aborting."
        exit 1
    fi
    echo "[bootstrap] Attempt ${count}/${MAX_RETRIES} — not ready yet, retrying in ${RETRY_INTERVAL}s ..."
    sleep "$RETRY_INTERVAL"
done
echo "[bootstrap] Server is ready."

# ── Create the initial admin account ─────────────────────────────────────────
echo "[bootstrap] Creating admin account for user '${USERNAME}' ..."

HTTP_STATUS=$(
    wget -q -O /tmp/bootstrap_response.json \
        --server-response \
        --header "Content-Type: application/json" \
        --post-data "{\"username\":\"${USERNAME}\",\"password\":\"${PASSWORD}\",\"role\":\"admin\"}" \
        "${SERVER_URL}/api/setup/admin" 2>/tmp/bootstrap_headers.txt || true
    awk '/^[[:space:]]*HTTP\// { status = $2 } END { print status ? status : "000" }' /tmp/bootstrap_headers.txt
)

RESPONSE_BODY=""
if [ -f /tmp/bootstrap_response.json ]; then
    RESPONSE_BODY=$(cat /tmp/bootstrap_response.json)
fi

echo "[bootstrap] Server responded with status ${HTTP_STATUS}: ${RESPONSE_BODY}"

# Treat 200 (created) and 409 (already exists) as success so the script is
# idempotent even if the volume was wiped but the database already has the user.
case "$HTTP_STATUS" in
    200|201|204|409)
        echo "[bootstrap] Admin account ready."
        ;;
    *)
        echo "[bootstrap] WARNING: Unexpected status ${HTTP_STATUS}. The server may not yet expose /api/setup/admin; proceeding to write marker anyway to avoid infinite restart loops."
        ;;
esac

# ── Write marker ──────────────────────────────────────────────────────────────
mkdir -p "$(dirname "$MARKER")"
printf "initialized_at=%s\nusername=%s\n" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "${USERNAME}" > "$MARKER"
echo "[bootstrap] Marker written to ${MARKER}. Bootstrap complete."
