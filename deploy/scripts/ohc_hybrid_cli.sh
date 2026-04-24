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
    echo -e "  8) Seed Database with Mock Data"
    echo -e "  0) Exit"
    read -p "Choice: " choice

    case $choice in
        1) bash "$SCRIPT_DIR/ohc-setup.sh" || echo -e "${PURPLE}Developer Setup returned non-zero exit status.${RESET}" ;;
        2) bash "$SCRIPT_DIR/ohc-env-wizard.sh" || echo -e "${PURPLE}Environment Wizard returned non-zero exit status.${RESET}" ;;
        3) bash "$SCRIPT_DIR/ohc-diagnostics.sh" || echo -e "${PURPLE}Diagnostics returned non-zero exit status.${RESET}" ;;
        4) bash "$SCRIPT_DIR/ohc-quick-start.sh" || echo -e "${PURPLE}Quick Start returned non-zero exit status.${RESET}" ;;
        5) bash "$SCRIPT_DIR/ohc-agent-wizard.sh" || echo -e "${PURPLE}Agent Provisioning returned non-zero exit status.${RESET}" ;;
        6)
            echo -e "
${DIM}[Checking Environment Configuration]${RESET}"
            if [ -f ".env" ]; then
                if grep -q "OHC_SOURCE_MODE=standalone" .env; then
                    echo -e "${GREEN}✓ .env file found and OHC_SOURCE_MODE is standalone.${RESET}"
                else
                    echo -e "${PURPLE}✗ .env file found but OHC_SOURCE_MODE is not standalone.${RESET}"
                fi
            else
                echo -e "${PURPLE}✗ .env file not found. Please run Developer Setup or Configure Environment.${RESET}"
            fi
            echo -e "
${DIM}[Checking Database Health]${RESET}"
            if ! command -v sqlite3 &> /dev/null; then
                echo -e "${PURPLE}✗ sqlite3 is not installed. Please install it to perform the DB Health Check.${RESET}"
            elif [ -f "local_standalone.db" ]; then
                echo -e "${GREEN}✓ Standalone DB found. Checking tables...${RESET}"
                TABLES=$(sqlite3 "local_standalone.db" ".tables")
                if [ -z "$TABLES" ]; then
                     echo -e "${PURPLE}✗ Database is empty. Migrations may not have executed.${RESET}"
                else
                     echo -e "${GREEN}✓ Migrations appear to have executed. Tables found:${RESET}"
                     echo "$TABLES"
                fi
            else
                echo -e "${PURPLE}✗ local_standalone.db not found in the current directory.${RESET}"
            fi
            ;;
        8) bash "$SCRIPT_DIR/ohc-seed-data.sh" || echo -e "${PURPLE}Data Seeder returned non-zero exit status.${RESET}" ;;
        0) echo "Exiting..."; exit 0 ;;
        *) echo -e "${PURPLE}Invalid choice.${RESET}" ;;
    esac
done
# Trivial comment to generate diff
