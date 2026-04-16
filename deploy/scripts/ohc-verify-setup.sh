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
echo -e "${BOLD}${CYAN}      OHC: Interactive Setup Verification             ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

if [ ! -f .env ]; then
    echo -e "${PURPLE}✗ .env file not found. Run ohc-setup.sh first.${RESET}"
    exit 1
fi

echo -e "${DIM}[1/2] Verifying .env configuration...${RESET}"

# Load env variables (ignore error if any)
set -a
source .env
set +a

ERRORS=0

if [ -z "$PORT" ]; then echo -e "  ${PURPLE}✗ PORT is missing${RESET}"; ERRORS=$((ERRORS+1)); else echo -e "  ${GREEN}✓ PORT=${PORT}${RESET}"; fi
if [ -z "$LOG_LEVEL" ]; then echo -e "  ${PURPLE}✗ LOG_LEVEL is missing${RESET}"; ERRORS=$((ERRORS+1)); else echo -e "  ${GREEN}✓ LOG_LEVEL=${LOG_LEVEL}${RESET}"; fi
if [ -z "$OHC_SOURCE_MODE" ]; then echo -e "  ${PURPLE}✗ OHC_SOURCE_MODE is missing${RESET}"; ERRORS=$((ERRORS+1)); else echo -e "  ${GREEN}✓ OHC_SOURCE_MODE=${OHC_SOURCE_MODE}${RESET}"; fi

if [ $ERRORS -gt 0 ]; then
    echo -e "${PURPLE}✗ Configuration validation failed.${RESET}"
    exit 1
fi

echo -e "${DIM}[2/2] Generating Audit Log...${RESET}"
RUNTIME_DIR="${OHC_RUNTIME_DIR:-.ohc/runtime}"
STATUS_DIR="${OHC_STATUS_DIR:-${RUNTIME_DIR}/status}"
mkdir -p "${STATUS_DIR}"
TIMESTAMP=$(date +%s)

cat << STAT > "${STATUS_DIR}/${TIMESTAMP}-verify.yml"
type: audit
metadata:
  role: Setup Verification
  timestamp: ${TIMESTAMP}
health: ok
observations:
  - .env configuration validated successfully.
STAT

echo -e "${GREEN}✓ Verification completed successfully. Audit log saved.${RESET}"
