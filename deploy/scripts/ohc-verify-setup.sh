#!/bin/bash
set -e

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
YELLOW="\033[38;5;220m"

echo -e "${BOLD}${CYAN}🔍 Auditing Day One Environment Setup...${RESET}"

STATUS_DIR=".ohc/runtime/status"
mkdir -p "${STATUS_DIR}"

if [ ! -f .env ]; then
    echo -e "${PURPLE}Error: .env file missing!${RESET}"
    exit 1
fi

PORT=$(grep "^PORT=" .env | cut -d '=' -f 2 || true)
LOG_LEVEL=$(grep "^LOG_LEVEL=" .env | cut -d '=' -f 2 || true)
OHC_SOURCE_MODE=$(grep "^OHC_SOURCE_MODE=" .env | cut -d '=' -f 2 || true)

ERRORS=0

if [ -z "$PORT" ]; then
    echo -e "${YELLOW}Warning: PORT is not set.${RESET}"
    ERRORS=$((ERRORS+1))
else
    echo -e "${GREEN}PORT is set to: $PORT${RESET}"
fi

if [ -z "$LOG_LEVEL" ]; then
    echo -e "${YELLOW}Warning: LOG_LEVEL is not set.${RESET}"
    ERRORS=$((ERRORS+1))
else
    echo -e "${GREEN}LOG_LEVEL is set to: $LOG_LEVEL${RESET}"
fi

if [ -z "$OHC_SOURCE_MODE" ]; then
    echo -e "${YELLOW}Warning: OHC_SOURCE_MODE is not set.${RESET}"
    ERRORS=$((ERRORS+1))
else
    echo -e "${GREEN}OHC_SOURCE_MODE is set to: $OHC_SOURCE_MODE${RESET}"
fi

TIMESTAMP=$(date +%s)
YAML_FILE="${STATUS_DIR}/${TIMESTAMP}_audit.yml"

cat << YAML > "${YAML_FILE}"
type: audit
metadata:
  role: Setup Verifier
  timestamp: ${TIMESTAMP}
health: $(if [ $ERRORS -eq 0 ]; then echo "ok"; else echo "degraded"; fi)
observations:
  - PORT=${PORT:-}
  - LOG_LEVEL=${LOG_LEVEL:-}
  - OHC_SOURCE_MODE=${OHC_SOURCE_MODE:-}
  - errors=${ERRORS}
YAML

echo -e "${BOLD}${GREEN}Audit complete. Log saved to ${YAML_FILE}${RESET}"
