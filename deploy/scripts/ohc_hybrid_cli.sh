#!/bin/bash
# OHC Hybrid CLI Master Menu

# Remove set -e to prevent the interactive loop from exiting on sub-script failure

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

function show_menu() {
    clear
    echo -e "${BOLD}${BLUE}===============================================${RESET}"
    echo -e "${BOLD}${CYAN}   🚀 OHC Hybrid Agentic OS Master Menu        ${RESET}"
    echo -e "${BOLD}${BLUE}===============================================${RESET}"
    echo -e "${DIM}Please select an onboarding script to run:${RESET}"
    echo ""
    echo -e "  ${GREEN}1)${RESET} Developer Setup (ohc-setup.sh)"
    echo -e "  ${GREEN}2)${RESET} Environment Wizard (ohc-env-wizard.sh)"
    echo -e "  ${GREEN}3)${RESET} Agent Wizard (ohc-agent-wizard.sh)"
    echo -e "  ${GREEN}4)${RESET} Quick Start (ohc-quick-start.sh)"
    echo -e "  ${GREEN}5)${RESET} Cloud Start (ohc-cloud-start.sh)"
    echo -e "  ${GREEN}6)${RESET} Diagnostics (ohc-diagnostics.sh)"
    echo -e "  ${GREEN}7)${RESET} Swarm Status (ohc-swarm-status.sh)"
    echo -e "  ${GREEN}8)${RESET} Seed Data (ohc-seed-data.sh)"
    echo -e "  ${GREEN}9)${RESET} Day One Audit (ohc-audit-day-one.sh)"
    echo -e "  ${GREEN}0)${RESET} Switch Context (ohc-mode.sh)"
    echo -e "  ${PURPLE}q)${RESET} Quit"
    echo -e "${BOLD}${BLUE}===============================================${RESET}"
}

while true; do
    show_menu
    read -p "Enter your choice: " choice
    case $choice in
        1) bash "$SCRIPT_DIR/ohc-setup.sh" || echo -e "${PURPLE}Developer Setup returned non-zero exit status.${RESET}" ;;
        2) bash "$SCRIPT_DIR/ohc-env-wizard.sh" || echo -e "${PURPLE}Environment Wizard returned non-zero exit status.${RESET}" ;;
        3) bash "$SCRIPT_DIR/ohc-agent-wizard.sh" || echo -e "${PURPLE}Agent Provisioning returned non-zero exit status.${RESET}" ;;
        4) bash "$SCRIPT_DIR/ohc-quick-start.sh" || echo -e "${PURPLE}Quick Start returned non-zero exit status.${RESET}" ;;
        5) bash "$SCRIPT_DIR/ohc-cloud-start.sh" || echo -e "${PURPLE}Cloud Start returned non-zero exit status.${RESET}" ;;
        6) bash "$SCRIPT_DIR/ohc-diagnostics.sh" || echo -e "${PURPLE}Diagnostics returned non-zero exit status.${RESET}" ;;
        7) bash "$SCRIPT_DIR/ohc-swarm-status.sh" || echo -e "${PURPLE}Swarm Status returned non-zero exit status.${RESET}" ;;
        8) bash "$SCRIPT_DIR/ohc-seed-data.sh" || echo -e "${PURPLE}Seed Data returned non-zero exit status.${RESET}" ;;
        9) bash "$SCRIPT_DIR/ohc-audit-day-one.sh" || echo -e "${PURPLE}Day One Audit returned non-zero exit status.${RESET}" ;;
        0) echo "Source ohc-mode.sh directly to switch contexts in your shell: source deploy/scripts/ohc-mode.sh <mode>"; read -p "Press enter to continue..." ;;
        q|Q) echo -e "${CYAN}Exiting OHC Master Menu. Goodbye!${RESET}"; return 0 2>/dev/null || break ;;
        *) echo -e "${PURPLE}Invalid option. Please try again.${RESET}"; sleep 1 ;;
    esac
    if [[ "$choice" != "q" && "$choice" != "Q" && "$choice" != "0" ]]; then
        echo -e "\n${DIM}Script execution completed.${RESET}"
        read -p "Press [Enter] to return to the menu..."
    fi
done
