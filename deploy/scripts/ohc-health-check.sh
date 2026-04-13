#!/bin/bash
set -e

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
RED="\033[31m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC: Onboarding Health Check Script             ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

# Verify .env exists
echo -e "${DIM}[Verifying Environment Configuration]${RESET}"
if [ -f ".env" ]; then
    echo -e "  ${GREEN}✓ .env file exists${RESET}"

    # Check PORT
    if grep -q "^PORT=" .env; then
        echo -e "  ${GREEN}✓ PORT variable is defined${RESET}"
    else
        echo -e "  ${PURPLE}✗ PORT variable is missing in .env${RESET}"
    fi

    # Check LOG_LEVEL
    if grep -q "^LOG_LEVEL=" .env; then
        echo -e "  ${GREEN}✓ LOG_LEVEL variable is defined${RESET}"
    else
        echo -e "  ${PURPLE}✗ LOG_LEVEL variable is missing in .env${RESET}"
    fi
else
    echo -e "  ${PURPLE}✗ .env file not found${RESET}"
    echo -e "    ${DIM}(Hint: Run the Interactive Environment Wizard first)${RESET}"
fi

echo -e "\n${DIM}[Verifying Required Tools]${RESET}"
TOOLS=("bazelisk" "docker" "go" "sqlite3")
MISSING_TOOLS=0
for tool in "${TOOLS[@]}"; do
    if command -v "$tool" >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ $tool installed${RESET}"
    else
        echo -e "  ${PURPLE}✗ $tool not found${RESET}"
        MISSING_TOOLS=$((MISSING_TOOLS+1))
    fi
done

echo -e "\n${BOLD}${BLUE}======================================================${RESET}"
if [ $MISSING_TOOLS -eq 0 ]; then
    echo -e "${BOLD}${GREEN}✓ Health Check Passed${RESET}"
else
    echo -e "${BOLD}${PURPLE}✗ Health Check failed. Missing $MISSING_TOOLS required tools.${RESET}"
fi
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""
