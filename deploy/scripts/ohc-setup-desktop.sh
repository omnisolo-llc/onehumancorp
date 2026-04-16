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
echo -e "${BOLD}${CYAN}   🚀 OHC Desktop Quick Start Mode Onboarding    ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

echo -e "${DIM}[1/3] Verifying dependencies...${RESET}"
command -v bazelisk >/dev/null 2>&1 || { echo -e "${PURPLE}Bazelisk is required.${RESET}"; false; }

echo -e "${DIM}[2/3] Setting up environment for Standalone Desktop Mode...${RESET}"
if [ ! -f .env.desktop ]; then
  echo "OHC_MULTITENANT=false" > .env.desktop
  echo "OHC_HEADLESS=false" >> .env.desktop
  echo "OHC_SOURCE_MODE=standalone" >> .env.desktop
  echo "OHC_STANDALONE=true" >> .env.desktop
  chmod 0600 .env.desktop
fi

echo -e "${DIM}[3/3] Launching Standalone Desktop Mode...${RESET}"
export OHC_MULTITENANT=false
export OHC_HEADLESS=false
export OHC_SOURCE_MODE=standalone
export OHC_STANDALONE=true

echo -e "${GREEN}✓ Desktop environment configured and ready for launch.${RESET}"
echo -e "${DIM}Run 'bazelisk run //:desktop' to launch the app.${RESET}"
