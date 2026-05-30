#!/bin/bash
# Remove set -e to prevent the interactive loop from exiting on sub-script failure

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
WHITE="\033[38;5;231m"
BG_GLASS="\033[48;5;236m" # Dark gray for glassmorphism
ACCENT="\033[38;5;33m" # Apple Blue
BORDER="\033[38;5;240m" # Border color

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

clear
echo -e "${BORDER}╭────────────────────────────────────────────────────────╮${RESET}"
echo -e "${BORDER}│${RESET}   ${BOLD}${ACCENT}OHC Hybrid Agentic OS${RESET}                              ${BORDER}│${RESET}"
echo -e "${BORDER}│${RESET}   ${DIM}${WHITE}Master CLI • System Console${RESET}                        ${BORDER}│${RESET}"
echo -e "${BORDER}╰────────────────────────────────────────────────────────╯${RESET}"

while true; do
    echo -e "\n${BORDER}╭─ ${BOLD}Select an action${RESET} ${BORDER}─────────────────────────────────────╮${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}1)${RESET} ${WHITE}Run Developer Setup${RESET}                               ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}2)${RESET} ${WHITE}Configure Environment (.env)${RESET}                      ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}3)${RESET} ${WHITE}Run Diagnostics${RESET}                                   ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}4)${RESET} ${WHITE}Launch Quick Start (Standalone)${RESET}                   ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}5)${RESET} ${WHITE}Provision AI Agent${RESET}                                ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}6)${RESET} ${WHITE}Standalone DB Health Check${RESET}                        ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}7)${RESET} ${WHITE}Launch Cloud Start${RESET}                                ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}8)${RESET} ${WHITE}Seed Database with Mock Data${RESET}                      ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}9)${RESET} ${WHITE}Check Swarm Status${RESET}                                ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET} ${DIM}10)${RESET} ${WHITE}Verify Setup${RESET}                                      ${BORDER}│${RESET}"
    echo -e "${BORDER}│${RESET}  ${DIM}0)${RESET} ${WHITE}Exit${RESET}                                              ${BORDER}│${RESET}"
    echo -e "${BORDER}╰────────────────────────────────────────────────────────╯${RESET}"
    echo -e -n "  ${ACCENT}▶${RESET} Choice: "
    read choice

    case $choice in
        1) (set -e; bash "$SCRIPT_DIR/ohc-setup.sh") || echo -e "${PURPLE}Developer Setup returned non-zero exit status ($?).${RESET}" ;;
        2) (set -e; bash "$SCRIPT_DIR/ohc-env-wizard.sh") || echo -e "${PURPLE}Environment Wizard returned non-zero exit status ($?).${RESET}" ;;
        3) (set -e; bash "$SCRIPT_DIR/ohc-diagnostics.sh") || echo -e "${PURPLE}Diagnostics returned non-zero exit status ($?).${RESET}" ;;
        4) (set -e; bash "$SCRIPT_DIR/ohc-standalone.sh") || echo -e "${PURPLE}Quick Start returned non-zero exit status ($?).${RESET}" ;;
        5) (bash "$SCRIPT_DIR/ohc-agent-wizard.sh") || echo -e "${PURPLE}Agent Provisioning returned non-zero exit status ($?).${RESET}" ;;
        6)
            if ! command -v sqlite3 &> /dev/null; then
                echo -e "${PURPLE}✗ sqlite3 is not installed. Please install it to perform the DB Health Check.${RESET}"
            elif [ -f "$HOME/.ohc-local-data/standalone.db" ]; then
                echo -e "${GREEN}✓ Standalone DB found. Checking tables...${RESET}"
                sqlite3 "$HOME/.ohc-local-data/standalone.db" ".tables" || echo -e "${PURPLE}DB Check failed with exit status $?.${RESET}"
            else
                echo -e "${PURPLE}✗ standalone.db not found in $HOME/.ohc-local-data/.${RESET}"
            fi
            ;;
        7) (set -e; bash "$SCRIPT_DIR/ohc-cloud-start.sh") || echo -e "${PURPLE}Cloud Start returned non-zero exit status ($?).${RESET}" ;;
        8) (set -e; bash "$SCRIPT_DIR/ohc-seed-data.sh") || echo -e "${PURPLE}Data Seeder returned non-zero exit status ($?).${RESET}" ;;
        9) (set -e; bash "$SCRIPT_DIR/ohc-swarm-status.sh") || echo -e "${PURPLE}Swarm Status returned non-zero exit status ($?).${RESET}" ;;
        10) (set -e; bash "$SCRIPT_DIR/ohc-verify-setup.sh") || echo -e "${PURPLE}Verify Setup returned non-zero exit status ($?).${RESET}" ;;
        0) echo -e "\n${DIM}Exiting OHC CLI...${RESET}\n"; exit 0 ;;
        *) echo -e "${PURPLE}Invalid choice.${RESET}" ;;
    esac
done
