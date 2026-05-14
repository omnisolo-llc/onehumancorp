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
    echo -e "  6) Standalone DB Health Check"
    echo -e "  7) Launch Cloud Start"
    echo -e "  8) Seed Database with Mock Data"
    echo -e "  9) Check Swarm Status"
    echo -e "  10) Verify Setup"
    echo -e "  0) Exit"
    read -p "Choice: " choice

    case $choice in
        1) (set -e; bash "$SCRIPT_DIR/ohc-setup.sh") || echo -e "${PURPLE}Developer Setup returned non-zero exit status ($?).${RESET}" ;;
        2) (set -e; bash "$SCRIPT_DIR/ohc-env-wizard.sh") || echo -e "${PURPLE}Environment Wizard returned non-zero exit status ($?).${RESET}" ;;
        3) (set -e; bash "$SCRIPT_DIR/ohc-diagnostics.sh") || echo -e "${PURPLE}Diagnostics returned non-zero exit status ($?).${RESET}" ;;
        4) (set -e; bash "$SCRIPT_DIR/ohc-standalone.sh") || echo -e "${PURPLE}Quick Start returned non-zero exit status ($?).${RESET}" ;;
        5) (set -e; bash "$SCRIPT_DIR/ohc-agent-wizard.sh") || echo -e "${PURPLE}Agent Provisioning returned non-zero exit status ($?).${RESET}" ;;
        6)
            if ! command -v sqlite3 &> /dev/null; then
                echo -e "${PURPLE}✗ sqlite3 is not installed. Please install it to perform the DB Health Check.${RESET}"
            elif [ -f "local_standalone.db" ]; then
                echo -e "${GREEN}✓ Standalone DB found. Checking tables...${RESET}"
                sqlite3 "local_standalone.db" ".tables" || echo -e "${PURPLE}DB Check failed with exit status $?.${RESET}"
            else
                echo -e "${PURPLE}✗ local_standalone.db not found in the current directory.${RESET}"
            fi
            ;;
        7) (set -e; bash "$SCRIPT_DIR/ohc-cloud-start.sh") || echo -e "${PURPLE}Cloud Start returned non-zero exit status ($?).${RESET}" ;;
        8) (set -e; bash "$SCRIPT_DIR/ohc-seed-data.sh") || echo -e "${PURPLE}Data Seeder returned non-zero exit status ($?).${RESET}" ;;
        9) (set -e; bash "$SCRIPT_DIR/ohc-swarm-status.sh") || echo -e "${PURPLE}Swarm Status returned non-zero exit status ($?).${RESET}" ;;
        10) (set -e; bash "$SCRIPT_DIR/ohc-verify-setup.sh") || echo -e "${PURPLE}Verify Setup returned non-zero exit status ($?).${RESET}" ;;
        0) echo "Exiting..."; exit 0 ;;
        *) echo -e "${PURPLE}Invalid choice.${RESET}" ;;
    esac
done
# Trivial comment to generate diff
