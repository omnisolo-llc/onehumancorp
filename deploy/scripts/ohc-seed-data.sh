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

if [ -f ".env" ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

echo -e "${DIM}[Executing Go Seeder tool...]${RESET}"

if bazelisk run //srcs/server/cmd/seeder; then
    echo -e "${GREEN}✓ Mock Data seeded successfully!${RESET}"
    echo -e "${DIM}Your dashboard is now populated with 'Launch Readiness' demo data.${RESET}"
else
    echo -e "${PURPLE}✗ Failed to seed data.${RESET}"
fi
echo ""
