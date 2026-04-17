#!/bin/bash
# OHC Hybrid Agentic OS - Day One Data Seeder

set -e

# Premium aesthetics colors
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}         OHC: Day One Mock Data Seeder                ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

# We will send a POST request to /api/dev/seed
# We assume the server is running on the default port or from .env

if [ -f ".env" ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

PORT=${PORT:-8080}
API_URL="http://127.0.0.1:${PORT}/api/dev/seed"

echo -e "${DIM}[Calling API to seed data: ${API_URL}]${RESET}"

# Use curl to trigger the seeder
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X POST -H "Content-Type: application/json" -d '{"scenario": "launch-readiness"}' "$API_URL" || echo "failed")

if [ "$RESPONSE" == "200" ]; then
    echo -e "${GREEN}✓ Mock Data seeded successfully!${RESET}"
    echo -e "${DIM}Your dashboard is now populated with 'Launch Readiness' demo data.${RESET}"
elif [ "$RESPONSE" == "failed" ]; then
    echo -e "${PURPLE}✗ Failed to connect to OHC Backend on port ${PORT}.${RESET}"
    echo -e "${DIM}Please ensure the server is running (e.g., using 'Launch Standalone Desktop Mode').${RESET}"
else
    echo -e "${PURPLE}✗ Failed to seed data. Server returned HTTP ${RESPONSE}.${RESET}"
fi
echo ""
