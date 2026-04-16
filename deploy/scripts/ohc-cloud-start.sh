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
echo -e "${BOLD}${CYAN}   🚀 OHC Cloud Native Quick Start (K8s)     ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

echo -e "${DIM}[1/3] Verifying dependencies...${RESET}"
command -v docker >/dev/null 2>&1 || { echo -e "${PURPLE}Docker is required.${RESET}"; false; }
command -v helm >/dev/null 2>&1 || { echo -e "${PURPLE}Helm is required.${RESET}"; false; }
command -v kubectl >/dev/null 2>&1 || { echo -e "${PURPLE}Kubectl is required.${RESET}"; false; }

echo -e "${DIM}[2/3] Setting up cloud environment...${RESET}"
if [ ! -f .env.cloud ]; then
  echo "OHC_MULTITENANT=true" > .env.cloud
  echo "OHC_HEADLESS=true" >> .env.cloud
  echo "OHC_SOURCE_MODE=cloud" >> .env.cloud
  chmod 0600 .env.cloud
fi

echo -e "${DIM}[3/3] Launching cloud backend...${RESET}"
export OHC_MULTITENANT=true
export OHC_HEADLESS=true
export OHC_SOURCE_MODE=cloud

echo -e "${GREEN}✓ Cloud environment configured and ready for deployment.${RESET}"
echo -e "${DIM}Run 'bazelisk run //:deploy_dev' to launch the K8s local backend.${RESET}"
