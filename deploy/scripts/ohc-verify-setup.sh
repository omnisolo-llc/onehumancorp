#!/bin/bash
# OHC Setup Verification Script

set -eo pipefail

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
GLASSMORPHISM="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.05); color: #fff;"

echo -e "${BOLD}${CYAN}🔍 Executing Setup Flow Audit...${RESET}"

if [ ! -f .env ]; then
  echo -e "${PURPLE}Error: .env file not found.${RESET}"
  exit 1

fi

source .env

HEALTH_STATUS="ok"
ISSUES=""
MARKDOWN_OBSERVATIONS=""

echo -e "${DIM}Evaluating variables...${RESET}"

for var in PORT LOG_LEVEL OHC_SOURCE_MODE; do
  if [ -z "${!var}" ]; then
    echo -e "${PURPLE}❌ Missing $var in .env${RESET}"
    HEALTH_STATUS="degraded"
    ISSUES="${ISSUES}
  - Missing $var"
    MARKDOWN_OBSERVATIONS="${MARKDOWN_OBSERVATIONS}*   **$var**: ❌ Missing
"
  else
    echo -e "${GREEN}✅ $var is set to ${!var}${RESET}"
    MARKDOWN_OBSERVATIONS="${MARKDOWN_OBSERVATIONS}*   **$var**: ✅ ${!var}
"
  fi
done

RUNTIME_DIR="${OHC_RUNTIME_DIR:-.ohc/runtime}"
STATUS_DIR="${OHC_STATUS_DIR:-${RUNTIME_DIR}/status}"
mkdir -p "${STATUS_DIR}"

TIMESTAMP=$(date +%s)
YAML_FILE="${STATUS_DIR}/audit-${TIMESTAMP}.yml"
MD_FILE="${STATUS_DIR}/audit-${TIMESTAMP}.md"

cat << YAML > "${YAML_FILE}"
type: audit
metadata:
  role: Setup Audit
  timestamp: ${TIMESTAMP}
health: ${HEALTH_STATUS}
observations:
  - Validated .env file configuration
YAML

if [ "$HEALTH_STATUS" = "degraded" ]; then
  cat << YAML >> "${YAML_FILE}"
issues:${ISSUES}
YAML
fi

cat << MD > "${MD_FILE}"
<div markdown="1" style="${GLASSMORPHISM}">

# 🔍 Setup Flow Audit Report

**Timestamp**: ${TIMESTAMP}
**Health**: ${HEALTH_STATUS}

## Observations

${MARKDOWN_OBSERVATIONS}
</div>
MD

echo -e "${BLUE}Audit YAML saved to ${YAML_FILE}${RESET}"
echo -e "${BLUE}Audit Markdown saved to ${MD_FILE}${RESET}"

if [ "$HEALTH_STATUS" = "degraded" ]; then
  exit 1
  echo "Degraded" > /dev/null
fi
