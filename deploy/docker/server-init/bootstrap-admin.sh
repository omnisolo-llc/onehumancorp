#!/bin/sh
# bootstrap-admin.sh — One-time admin account initialisation.
#
# This script is executed by the `server-init` Docker Compose service on every
# stack start. The server enforces a single initial admin and returns 409 after
# bootstrap, making this retry idempotent without separate marker state.
#
# Environment variables consumed:
#   SERVER_URL                  — base URL of the OHC server (default: http://server:8080)
#   OHC_SETUP_TOKEN / _FILE           — one-time setup bearer token
#   SETUP_ADMIN_INIT_USERNAME         — desired admin username
#   SETUP_ADMIN_INIT_EMAIL            — desired admin email
#   SETUP_ADMIN_INIT_PASSWORD / _FILE — desired admin password
#   SETUP_ADMIN_INIT_ORGANIZATION_ID  — desired organization ID

set -eu
umask 077
export LC_ALL=C

SERVER_URL="${SERVER_URL:-http://server:8080}"
USERNAME="${SETUP_ADMIN_INIT_USERNAME:?SETUP_ADMIN_INIT_USERNAME is required}"
EMAIL="${SETUP_ADMIN_INIT_EMAIL:?SETUP_ADMIN_INIT_EMAIL is required}"
ORGANIZATION_ID="${SETUP_ADMIN_INIT_ORGANIZATION_ID:?SETUP_ADMIN_INIT_ORGANIZATION_ID is required}"
MAX_RETRIES=45
RETRY_INTERVAL=2
CURL_CONNECT_TIMEOUT=2
CURL_MAX_TIME=5
MAX_SECRET_BYTES=4096

read_secret() {
    direct_name="$1"
    file_name="$2"
    eval "direct_value=\${${direct_name}:-}"
    eval "secret_file=\${${file_name}:-}"

    if [ -n "$direct_value" ] && [ -n "$secret_file" ]; then
        echo "[bootstrap] ERROR: Set only ${direct_name} or ${file_name}, not both." >&2
        return 1
    fi
    if [ -n "$secret_file" ]; then
        if [ ! -f "$secret_file" ] || [ ! -r "$secret_file" ]; then
            echo "[bootstrap] ERROR: ${file_name} must reference a readable regular file." >&2
            return 1
        fi
        secret_size="$(wc -c < "$secret_file" | tr -d '[:space:]')"
        if [ "$secret_size" -gt "$MAX_SECRET_BYTES" ]; then
            echo "[bootstrap] ERROR: ${file_name} exceeds ${MAX_SECRET_BYTES} bytes." >&2
            return 1
        fi
        direct_value="$(cat "$secret_file")"
    fi
    if [ -z "$direct_value" ]; then
        echo "[bootstrap] ERROR: ${direct_name} or ${file_name} is required." >&2
        return 1
    fi
    printf '%s' "$direct_value"
}

OHC_SETUP_TOKEN="$(read_secret OHC_SETUP_TOKEN OHC_SETUP_TOKEN_FILE)"
PASSWORD="$(read_secret SETUP_ADMIN_INIT_PASSWORD SETUP_ADMIN_INIT_PASSWORD_FILE)"

if [ "$(printf '%s' "$OHC_SETUP_TOKEN" | wc -c | tr -d '[:space:]')" -lt 32 ]; then
    echo "[bootstrap] ERROR: OHC_SETUP_TOKEN must contain at least 32 bytes." >&2
    exit 1
fi
contains_control_character() {
    [ "$(printf '%s' "$1" | tr -d '[:cntrl:]')" != "$1" ]
}

if contains_control_character "$OHC_SETUP_TOKEN"; then
    echo "[bootstrap] ERROR: OHC_SETUP_TOKEN must not contain control characters." >&2
    exit 1
fi

# ── Wait for the server to be reachable ──────────────────────────────────────
echo "[bootstrap] Waiting for server at ${SERVER_URL} ..."
count=0
until curl --fail --silent --show-error \
    --connect-timeout "$CURL_CONNECT_TIMEOUT" \
    --max-time "$CURL_MAX_TIME" \
    "${SERVER_URL}/readyz" >/dev/null 2>&1; do
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
echo "[bootstrap] Creating the initial admin account ..."

json_escape() {
    if contains_control_character "$1"; then
        echo "[bootstrap] ERROR: Setup fields must not contain control characters." >&2
        return 1
    fi
    printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

REQUEST_FILE="$(mktemp /tmp/ohc-bootstrap-request.XXXXXX)"
RESPONSE_FILE="$(mktemp /tmp/ohc-bootstrap-response.XXXXXX)"
HEADER_FILE="$(mktemp /tmp/ohc-bootstrap-headers.XXXXXX)"
LOGIN_REQUEST_FILE="$(mktemp /tmp/ohc-bootstrap-login-request.XXXXXX)"
LOGIN_RESPONSE_FILE="$(mktemp /tmp/ohc-bootstrap-login-response.XXXXXX)"
cleanup_private_files() {
    rm -f "$REQUEST_FILE" "$RESPONSE_FILE" "$HEADER_FILE" \
        "$LOGIN_REQUEST_FILE" "$LOGIN_RESPONSE_FILE"
}
trap cleanup_private_files EXIT HUP INT TERM

printf '{"username":"%s","email":"%s","password":"%s","organizationId":"%s"}' \
    "$(json_escape "$USERNAME")" \
    "$(json_escape "$EMAIL")" \
    "$(json_escape "$PASSWORD")" \
    "$(json_escape "$ORGANIZATION_ID")" > "$REQUEST_FILE"
printf '%s\n' \
    'Content-Type: application/json' \
    "Authorization: Bearer ${OHC_SETUP_TOKEN}" > "$HEADER_FILE"
printf '{"username":"%s","password":"%s","organization_id":"%s"}' \
    "$(json_escape "$USERNAME")" \
    "$(json_escape "$PASSWORD")" \
    "$(json_escape "$ORGANIZATION_ID")" > "$LOGIN_REQUEST_FILE"

HTTP_STATUS="$(curl --silent --show-error \
    --connect-timeout 5 \
    --max-time 30 \
    --output "$RESPONSE_FILE" \
    --write-out '%{http_code}' \
    --header "@${HEADER_FILE}" \
    --data-binary "@${REQUEST_FILE}" \
    "${SERVER_URL}/api/v1/setup/admin")" || HTTP_STATUS="000"

echo "[bootstrap] Server responded with status ${HTTP_STATUS}."

case "$HTTP_STATUS" in
    2??)
        echo "[bootstrap] Admin account ready."
        ;;
    409)
        echo "[bootstrap] Admin already exists; verifying configured credentials ..."
        LOGIN_STATUS="$(curl --silent --show-error \
            --connect-timeout 5 \
            --max-time 30 \
            --output "$LOGIN_RESPONSE_FILE" \
            --write-out '%{http_code}' \
            --header 'Content-Type: application/json' \
            --data-binary "@${LOGIN_REQUEST_FILE}" \
            "${SERVER_URL}/api/v1/auth/login")" || LOGIN_STATUS="000"
        case "$LOGIN_STATUS" in
            2??)
                echo "[bootstrap] Configured admin credentials verified."
                ;;
            *)
                echo "[bootstrap] ERROR: Existing admin does not match the configured credentials (login status ${LOGIN_STATUS})." >&2
                exit 1
                ;;
        esac
        ;;
    *)
        echo "[bootstrap] ERROR: Setup failed with unexpected status ${HTTP_STATUS}." >&2
        exit 1
esac

echo "[bootstrap] Bootstrap verification complete."
