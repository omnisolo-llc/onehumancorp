#!/bin/bash
# OHC Hybrid Agentic OS - Day One Onboarding CLI
# Adheres to Zero Secrets Mandate and Premium Aesthetics

# Exit immediately if a command exits with a non-zero status
set -e

# Colors for "Premium" terminal aesthetics
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}       OHC: The Hybrid Agentic OS - Setup CLI         ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

show_menu() {
    echo -e "${BOLD}Select an action:${RESET}"
    echo -e "  ${PURPLE}1)${RESET} Run Initial Setup (Generate .env, Verify build)"
    echo -e "  ${PURPLE}2)${RESET} Switch Environment Mode (Cloud / Standalone / Headless)"
    echo -e "  ${PURPLE}3)${RESET} Launch Standalone Desktop Mode"
    echo -e "  ${PURPLE}4)${RESET} Launch Cloud Backend"
    echo -e "  ${PURPLE}5)${RESET} Run All Tests"
    echo -e "  ${PURPLE}6)${RESET} Verify System Dependencies"
    echo -e "  ${PURPLE}q)${RESET} Quit"
    echo ""
}

run_setup() {
    echo -e "${DIM}[Executing deploy/scripts/ohc-setup.sh...]${RESET}"
    if bash deploy/scripts/ohc-setup.sh; then
        echo -e "${GREEN}Setup completed successfully.${RESET}\n"
    else
        echo -e "${PURPLE}Setup failed.${RESET}\n"
    fi
}

switch_mode() {
    echo -e "Available modes: ${BOLD}cloud${RESET}, ${BOLD}standalone${RESET}, ${BOLD}headless${RESET}"
    read -p "Enter mode: " MODE
    if [[ "$MODE" == "cloud" || "$MODE" == "standalone" || "$MODE" == "headless" ]]; then
        if source deploy/scripts/ohc-mode.sh $MODE; then
            echo -e "${GREEN}Mode switched to $MODE.${RESET}"
            echo -e "${DIM}(Note: Run 'source deploy/scripts/ohc-mode.sh $MODE' in your main shell to persist)${RESET}\n"
        else
            echo -e "${PURPLE}Failed to switch mode.${RESET}\n"
        fi
    else
        echo -e "\nInvalid mode.\n"
    fi
}

launch_desktop() {
    echo -e "${DIM}[Launching Standalone Desktop...]${RESET}"
    if ! bazelisk run //:desktop; then
        echo -e "${PURPLE}Failed to launch Standalone Desktop.${RESET}\n"
    fi
}

launch_cloud() {
    echo -e "${DIM}[Launching Cloud Backend...]${RESET}"
    export OHC_MULTITENANT=true
    if ! bazelisk run //srcs/server:ohc; then
        echo -e "${PURPLE}Failed to launch Cloud Backend.${RESET}\n"
    fi
}

run_tests() {
    echo -e "${DIM}[Running bazelisk test //...]${RESET}"
    if bazelisk test //...; then
        echo -e "${GREEN}All tests passed successfully.${RESET}\n"
    else
        echo -e "${PURPLE}Some tests failed.${RESET}\n"
    fi
}

verify_dependencies() {
    echo -e "${DIM}[Verifying System Dependencies for OHC Hybrid OS]${RESET}"

    # Bazelisk
    if command -v bazelisk >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ bazelisk installed${RESET} ($(bazelisk version | grep 'Bazel version' || echo 'unknown version'))"
    else
        echo -e "  ${PURPLE}✗ bazelisk not found${RESET}"
    fi

    # Docker
    if command -v docker >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ docker installed${RESET} ($(docker --version))"
    else
        echo -e "  ${PURPLE}✗ docker not found${RESET}"
    fi

    # Go
    if command -v go >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ go installed${RESET} ($(go version))"
    else
        echo -e "  ${PURPLE}✗ go not found${RESET}"
    fi

    # SQLite3 (Standalone fallback)
    if command -v sqlite3 >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ sqlite3 installed${RESET} (Standalone Mode Ready)"
    else
        echo -e "  ${PURPLE}✗ sqlite3 not found${RESET} (Consider installing for local debugging)"
    fi

    # Redis CLI (Cloud Mode Tooling)
    if command -v redis-cli >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ redis-cli installed${RESET} (Cloud Mode Tooling Ready)"
    else
        echo -e "  ${PURPLE}✗ redis-cli not found${RESET} (Consider installing for cloud debugging)"
    fi

    # Verify System State / DB Check
    if [ -f ".agent-task/swarm.db" ]; then
        echo -e "  ${GREEN}✓ Local Swarm Database exists${RESET} (.agent-task/swarm.db)"
    else
        echo -e "  ${PURPLE}✗ Local Swarm Database not found${RESET}"
    fi

    echo ""
}

if [ "$1" == "--non-interactive" ]; then
    echo "Running in non-interactive verification mode."
    run_setup
    # run_tests
    echo -e "${GREEN}Verification completed.${RESET}\n"
else
    while true; do
        show_menu
        read -p "> " choice
        case $choice in
            1) run_setup ;;
            2) switch_mode ;;
            3) launch_desktop ;;
            4) launch_cloud ;;
            5) run_tests ;;
            6) verify_dependencies ;;
            q|Q) echo "Exiting."; break ;;
            *) echo "Invalid choice." ;;
        esac
    done
fi
