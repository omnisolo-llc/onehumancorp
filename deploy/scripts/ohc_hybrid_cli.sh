#!/bin/bash
# Remove set -e to prevent the interactive loop from exiting on sub-script failure

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC Hybrid Agentic OS - Master CLI              ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"

while true; do
    echo -e "\n${BOLD}Select an action:${RESET}"
    echo -e "  1) Run Developer Setup"
    echo -e "  2) Configure Environment (.env)"
    echo -e "  3) Run Diagnostics"
    echo -e "  4) Launch Quick Start (Standalone)"
    echo -e "  5) Provision AI Agent"
    echo -e "  6) Launch Cloud Start"
    echo -e "  7) Seed Mock Data"
    echo -e "  8) View Swarm Status"
    echo -e "  9) Switch Development Mode Instructions"
    echo -e "  0) Quit"
    read -p "Choice: " choice

    case $choice in
        1) bash "$SCRIPT_DIR/ohc-setup.sh" || echo -e "${PURPLE}Developer Setup returned non-zero exit status.${RESET}" ;;
        2) bash "$SCRIPT_DIR/ohc-env-wizard.sh" || echo -e "${PURPLE}Environment Wizard returned non-zero exit status.${RESET}" ;;
        3) bash "$SCRIPT_DIR/ohc-diagnostics.sh" || echo -e "${PURPLE}Diagnostics returned non-zero exit status.${RESET}" ;;
        4) bash "$SCRIPT_DIR/ohc-quick-start.sh" || echo -e "${PURPLE}Quick Start returned non-zero exit status.${RESET}" ;;
        5) bash "$SCRIPT_DIR/ohc-agent-wizard.sh" || echo -e "${PURPLE}Agent Provisioning returned non-zero exit status.${RESET}" ;;
        6) bash "$SCRIPT_DIR/ohc-cloud-start.sh" || echo -e "${PURPLE}Cloud Start returned non-zero exit status.${RESET}" ;;
        7) bash "$SCRIPT_DIR/ohc-seed-data.sh" || echo -e "${PURPLE}Seed Data returned non-zero exit status.${RESET}" ;;
        8) bash "$SCRIPT_DIR/ohc-swarm-status.sh" || echo -e "${PURPLE}Swarm Status returned non-zero exit status.${RESET}" ;;
        9) echo -e "${DIM}To switch development mode, source the mode script directly in your terminal:${RESET}"
           echo -e "  source $SCRIPT_DIR/ohc-mode.sh [cloud|standalone|headless]"
           ;;
        0) echo "Exiting..."; break ;;
        *) echo -e "${PURPLE}Invalid choice.${RESET}" ;;
    esac
done