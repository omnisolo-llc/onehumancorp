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
echo -e "${BOLD}${CYAN}   🚀 OHC Quick Start (Day One Onboarding)    ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

echo -e "${DIM}[1/4] Verifying dependencies...${RESET}"
command -v bazelisk >/dev/null 2>&1 || { echo -e "${PURPLE}Bazelisk is required.${RESET}"; false; }
command -v go >/dev/null 2>&1 || { echo -e "${PURPLE}Go is required.${RESET}"; false; }

echo -e "${DIM}[2/4] Setting up environment...${RESET}"
if [ ! -f .env ]; then
  echo "LOG_LEVEL=info" > .env
  echo "PORT=8080" >> .env
  echo "OHC_MULTITENANT=false" >> .env
  echo "OHC_HEADLESS=false" >> .env
  echo "OHC_SOURCE_MODE=standalone" >> .env
  chmod 0600 .env
fi

echo -e "${DIM}[3/4] Launching local backend...${RESET}"
export OHC_MULTITENANT=false
export OHC_SOURCE_MODE=standalone
bazelisk run //srcs/server:ohc &
SERVER_PID=$!
echo -e "${GREEN}✓ Server started with PID $SERVER_PID. To stop, run: kill $SERVER_PID${RESET}"

echo -e "${DIM}[4/4] Running Diagnostics...${RESET}"
yes | bash deploy/scripts/ohc-diagnostics.sh || true

echo -e "${BOLD}Next steps:${RESET}"
echo -e "  Use ${CYAN}./deploy/scripts/ohc_hybrid_cli.sh${RESET} to manage the OS, switch to Cloud Mode, or seed mock data."
