#!/bin/bash
# OHC Premium Setup Verification

set -e

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
RED="\033[38;5;196m"

echo -e "${BOLD}${BLUE}===============================================${RESET}"
echo -e "${BOLD}${CYAN}   🔍 OHC Environment Audit                    ${RESET}"
echo -e "${BOLD}${BLUE}===============================================${RESET}"

if [ ! -f .env ]; then
  echo -e "${BOLD}${RED}ERROR: .env file missing.${RESET}"
  exit 1
fi

# Load env safely
set -a
source .env
set +a

ISSUES=0

echo -e "${DIM}Auditing parameters...${RESET}"

# Check OHC_PORT
if [ -z "$OHC_PORT" ]; then
    echo -e "${RED}✗ OHC_PORT is not set.${RESET}"
    ISSUES=$((ISSUES+1))
else
    echo -e "${GREEN}✓ OHC_PORT is set to ${OHC_PORT}.${RESET}"
fi

# Check OHC_LOG_LEVEL
if [ -z "$OHC_LOG_LEVEL" ]; then
    echo -e "${RED}✗ OHC_LOG_LEVEL is not set.${RESET}"
    ISSUES=$((ISSUES+1))
else
    echo -e "${GREEN}✓ OHC_LOG_LEVEL is set to ${OHC_LOG_LEVEL}.${RESET}"
fi

# Check OHC_SOURCE_MODE
if [ -z "$OHC_SOURCE_MODE" ]; then
    echo -e "${RED}✗ OHC_SOURCE_MODE is not set.${RESET}"
    ISSUES=$((ISSUES+1))
else
    echo -e "${GREEN}✓ OHC_SOURCE_MODE is set to ${OHC_SOURCE_MODE}.${RESET}"
fi

# Generate telemetry-ready summary
RUNTIME_DIR="${OHC_RUNTIME_DIR:-.ohc/runtime}"
STATUS_DIR="${OHC_STATUS_DIR:-${RUNTIME_DIR}/status}"
mkdir -p "${STATUS_DIR}"
TIMESTAMP=$(date +%s)

YAML_FILE="${STATUS_DIR}/audit-${TIMESTAMP}.yml"
cat << YAMLEOF > "${YAML_FILE}"
type: audit
metadata:
  role: Environment Verification
  timestamp: ${TIMESTAMP}
health: $(if [ $ISSUES -eq 0 ]; then echo "ok"; else echo "degraded"; fi)
observations:
  - Checked .env file
  - Found ${ISSUES} issues
YAMLEOF

MD_FILE="${STATUS_DIR}/audit-${TIMESTAMP}.md"
cat << MDEOF > "${MD_FILE}"
# OHC Environment Audit

**Timestamp:** ${TIMESTAMP}
**Health:** $(if [ $ISSUES -eq 0 ]; then echo "OK"; else echo "DEGRADED"; fi)

## Observations
- Checked \`.env\` file
- Found ${ISSUES} issues

## Configuration Values
- **OHC_PORT:** ${OHC_PORT:-Unset}
- **OHC_LOG_LEVEL:** ${OHC_LOG_LEVEL:-Unset}
- **OHC_SOURCE_MODE:** ${OHC_SOURCE_MODE:-Unset}
MDEOF

if [ $ISSUES -gt 0 ]; then
  echo -e "${BOLD}${PURPLE}Audit failed with ${ISSUES} issues.${RESET}"
  exit 1
fi

echo -e "${BOLD}${GREEN}   ✅ Audit Complete! All parameters valid.    ${RESET}"
