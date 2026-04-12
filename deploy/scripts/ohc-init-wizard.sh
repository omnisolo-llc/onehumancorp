#!/bin/bash
set -e
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC: Interactive Initial Setup Wizard           ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""
echo -e "${DIM}[Checking .env Configuration]${RESET}"
if [ -f .env ]; then
    echo -e "  ${GREEN}✓ .env file found.${RESET}"
else
    echo -e "  ${PURPLE}✗ .env file missing. Run 'Initial Setup' in CLI.${RESET}"
fi
echo -e "${DIM}[Checking Standalone Database]${RESET}"
DB_FILE="$HOME/.ohc-local-data/standalone.db"
if [ -f "$DB_FILE" ]; then
    echo -e "  ${GREEN}✓ Standalone DB ready (${DB_FILE}).${RESET}"
else
    echo -e "  ${PURPLE}✗ Standalone DB missing. Will be created on launch.${RESET}"
fi
echo -e "${DIM}[Checking Cloud API Reachability]${RESET}"
PORT=${PORT:-8080}
if curl -s "http://127.0.0.1:${PORT}/healthz" > /dev/null; then
    echo -e "  ${GREEN}✓ Local backend running.${RESET}"
else
    echo -e "  ${DIM}Backend not currently running on port ${PORT}.${RESET}"
fi
echo ""
echo -e "${GREEN}✓ Initial setup checks completed!${RESET}"
