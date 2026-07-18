#!/bin/bash
# OHC Hybrid Agentic OS - Day One Data Seeder

set -euo pipefail

# Premium aesthetics colors
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}         OHC: Day One Database Seeder                 ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

# This invokes the real database-backed seed endpoint. Authentication is
# required; secrets may be supplied directly or through a private file.

for tool in curl jq; do
    command -v "$tool" >/dev/null 2>&1 || {
        echo "Required tool not found: $tool" >&2
        exit 1
    }
done

if [[ -n "${OHC_ACCESS_TOKEN:-}" && -n "${OHC_ACCESS_TOKEN_FILE:-}" ]]; then
    echo "Set only OHC_ACCESS_TOKEN or OHC_ACCESS_TOKEN_FILE, not both." >&2
    exit 1
fi
if [[ -n "${OHC_ACCESS_TOKEN_FILE:-}" ]]; then
    [[ -f "$OHC_ACCESS_TOKEN_FILE" && -r "$OHC_ACCESS_TOKEN_FILE" ]] || {
        echo "OHC_ACCESS_TOKEN_FILE must be a readable regular file." >&2
        exit 1
    }
    ACCESS_TOKEN="$(<"$OHC_ACCESS_TOKEN_FILE")"
else
    ACCESS_TOKEN="${OHC_ACCESS_TOKEN:-}"
fi
[[ -n "$ACCESS_TOKEN" ]] || {
    echo "OHC_ACCESS_TOKEN or OHC_ACCESS_TOKEN_FILE is required." >&2
    exit 1
}
if [[ ${#ACCESS_TOKEN} -gt 4096 || "$ACCESS_TOKEN" == *$'\n'* || "$ACCESS_TOKEN" == *$'\r'* ]]; then
    echo "Access token has an invalid format." >&2
    exit 1
fi

PORT=${PORT:-8080}
API_URL="http://127.0.0.1:${PORT}/api/v1/dev/seed"

echo -e "${DIM}[Calling API to seed data: ${API_URL}]${RESET}"

umask 077
REQUEST_FILE="$(mktemp)"
HEADER_FILE="$(mktemp)"
trap 'rm -f "$REQUEST_FILE" "$HEADER_FILE"' EXIT
jq -n --arg scenario 'launch-readiness' '{scenario: $scenario}' > "$REQUEST_FILE"
printf '%s\n' 'Content-Type: application/json' "Authorization: Bearer ${ACCESS_TOKEN}" > "$HEADER_FILE"
RESPONSE=$(curl --silent --show-error --connect-timeout 5 --max-time 30 \
    --output /dev/null --write-out '%{http_code}' --request POST \
    --header "@${HEADER_FILE}" --data-binary "@${REQUEST_FILE}" "$API_URL" || echo "failed")

if [ "$RESPONSE" == "200" ]; then
    echo -e "${GREEN}✓ Database seed completed successfully!${RESET}"
    echo -e "${DIM}Your dashboard is now populated with 'Launch Readiness' demo data.${RESET}"
elif [ "$RESPONSE" == "failed" ]; then
    echo -e "${PURPLE}✗ Failed to connect to OHC Backend on port ${PORT}.${RESET}"
    echo -e "${DIM}Please ensure the server is running (e.g., using 'Launch Standalone Desktop Mode').${RESET}"
else
    echo -e "${PURPLE}✗ Failed to seed data. Server returned HTTP ${RESPONSE}.${RESET}"
fi
echo ""
