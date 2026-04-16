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
echo -e "${BOLD}${CYAN}   🚀 OHC Desktop Start (Standalone Onboarding)    ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

echo -e "${DIM}[1/3] Verifying dependencies...${RESET}"
command -v flutter >/dev/null 2>&1 || { echo -e "${PURPLE}Flutter is required.${RESET}"; false; }

echo -e "${DIM}[2/3] Setting up standalone environment...${RESET}"
export OHC_DESKTOP_PLATFORM="linux"
export OHC_DESKTOP_ROOT="$(pwd)"

echo -e "${DIM}[3/3] Launching OHC Desktop App...${RESET}"
./srcs/app/standalone_linux_launcher.sh
