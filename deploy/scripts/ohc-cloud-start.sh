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
echo -e "${BOLD}${CYAN}   🚀 OHC Cloud Start (Cloud-Native K8s Onboarding)    ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

echo -e "${DIM}[1/3] Verifying dependencies...${RESET}"
command -v kubectl >/dev/null 2>&1 || { echo -e "${PURPLE}kubectl is required.${RESET}"; false; }
command -v minikube >/dev/null 2>&1 || { echo -e "${PURPLE}minikube is required.${RESET}"; false; }

echo -e "${DIM}[2/3] Setting up cloud environment...${RESET}"
if [ ! -f .env.cloud ]; then
  echo "OHC_MULTITENANT=true" > .env.cloud
  echo "OHC_HEADLESS=true" >> .env.cloud
  echo "OHC_SOURCE_MODE=cloud" >> .env.cloud
  chmod 0600 .env.cloud
fi

echo -e "${DIM}[3/3] Deploying to Kubernetes...${RESET}"
minikube start
kubectl apply -f deploy/k8s/
echo -e "${GREEN}✓ Cloud-native environment deployed.${RESET}"
