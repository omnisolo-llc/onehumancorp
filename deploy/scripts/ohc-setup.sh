#!/bin/bash
# OHC Hybrid Developer Setup Script

set -e

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

echo -e "${BOLD}${BLUE}===============================================${RESET}"
echo -e "${BOLD}${CYAN}   🚀 OHC Hybrid Agentic OS Developer Setup    ${RESET}"
echo -e "${BOLD}${BLUE}===============================================${RESET}"

# Check requirements
if ! command -v bazelisk >/dev/null 2>&1; then echo -e "${PURPLE}Bazelisk is required but not installed. Aborting.${RESET}"; false; fi
if ! command -v docker >/dev/null 2>&1; then echo -e "${PURPLE}Docker is required but not installed. Aborting.${RESET}"; false; fi

echo -e "${DIM}[1/5] Checking environment configuration...${RESET}"
if [ ! -f .env ]; then
  echo "Creating default .env file..."
  cat << 'ENV' > .env
# Default Local Config
LOG_LEVEL=info
PORT=8080
OHC_MULTITENANT=false
OHC_HEADLESS=false
OHC_SOURCE_MODE=standalone
OHC_RUNTIME_DIR=.ohc/runtime
OHC_MEMORY_DIR=.ohc/runtime/memory
OHC_STATUS_DIR=.ohc/runtime/status
ENV
  chmod 0600 .env
fi

echo -e "${DIM}[2/5] Verifying Standalone Mode...${RESET}"
export OHC_MULTITENANT=false
export OHC_HEADLESS=false
export OHC_SOURCE_MODE=standalone
bazelisk test //srcs/server/api/...

echo -e "${DIM}[3/5] Verifying Cloud Mode...${RESET}"
export OHC_MULTITENANT=true
export OHC_HEADLESS=false
export OHC_SOURCE_MODE=cloud
bazelisk test //srcs/server/api/...

echo -e "${DIM}[5/5] Generating Local Memory Log...${RESET}"
RUNTIME_DIR="${OHC_RUNTIME_DIR:-.ohc/runtime}"
MEMORY_DIR="${OHC_MEMORY_DIR:-${RUNTIME_DIR}/memory}"
STATUS_DIR="${OHC_STATUS_DIR:-${RUNTIME_DIR}/status}"
mkdir -p "${MEMORY_DIR}" "${STATUS_DIR}"
TIMESTAMP=$(date +%s)

cat << MEM > "${MEMORY_DIR}/setup-${TIMESTAMP}.yml"
type: memory
metadata:
  role: Developer Setup
  timestamp: ${TIMESTAMP}
observations:
  - Developer executed ohc-setup.sh
actions_taken:
  - Verified local environment
resolution: Developer environment successfully initialized.
MEM

cat << STAT > "${STATUS_DIR}/${TIMESTAMP}.yml"
type: status
metadata:
  role: Developer Setup
  timestamp: ${TIMESTAMP}
health: ok
observations:
  - ohc-setup.sh completed successfully.
STAT

echo -e "${BOLD}${BLUE}===============================================${RESET}"
echo -e "${BOLD}${GREEN}   ✅ Setup Complete!                          ${RESET}"
echo -e "${DIM}   Use 'source deploy/scripts/ohc-mode.sh' to switch contexts.${RESET}"
echo -e "${BOLD}${BLUE}===============================================${RESET}"
