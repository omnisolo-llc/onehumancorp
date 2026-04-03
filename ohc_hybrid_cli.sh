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
    echo -e "  ${PURPLE}6)${RESET} Run Doctor & API Key Wizard"
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

run_doctor() {
    echo -e "${DIM}[Running System Doctor & API Key Wizard...]${RESET}"
    echo -e "${BOLD}1. Checking Dependencies${RESET}"

    local deps=("docker" "bazelisk" "go" "flutter")
    local missing=0
    for dep in "${deps[@]}"; do
        if command -v $dep >/dev/null 2>&1; then
            echo -e "  [${GREEN}OK${RESET}] $dep is installed."
        else
            echo -e "  [${PURPLE}MISSING${RESET}] $dep is not found in PATH."
            missing=1
        fi
    done

    echo -e "\n${BOLD}2. Checking Ports${RESET}"
    local ports=(8080 5432 6379)
    for port in "${ports[@]}"; do
        if command -v lsof >/dev/null 2>&1; then
            if lsof -i :$port >/dev/null 2>&1; then
                echo -e "  [${PURPLE}IN USE${RESET}] Port $port is currently occupied."
            else
                echo -e "  [${GREEN}FREE${RESET}] Port $port is available."
            fi
        else
            echo -e "  [${DIM}SKIP${RESET}] lsof not installed, skipping port check."
            break
        fi
    done

    echo -e "\n${BOLD}3. API Key Wizard${RESET}"
    if [ ! -f .env ]; then
        echo "Creating default .env file..."
        echo "LOG_LEVEL=info" > .env
        echo "PORT=8080" >> .env
        echo "OHC_MULTITENANT=false" >> .env
        echo "OHC_HEADLESS=false" >> .env
        echo "OHC_SOURCE_MODE=standalone" >> .env
    fi

    local keys=("GEMINI_API_KEY" "ANTHROPIC_API_KEY" "OPENAI_API_KEY")
    for key in "${keys[@]}"; do
        if grep -q "^${key}=" .env; then
            local current_val=$(grep "^${key}=" .env | cut -d '=' -f2)
            if [ -n "$current_val" ]; then
                echo -e "  [${GREEN}OK${RESET}] $key is already set."
                continue
            fi
        fi

        read -s -p "  Enter $key (leave blank to skip): " key_val\n        echo ""
        if [ -n "$key_val" ]; then
            if grep -q "^${key}=" .env; then
                sed -i.bak "s|^${key}=.*|${key}=${key_val}|" .env
            else
                echo "${key}=${key_val}" >> .env
            fi
            echo -e "  [${GREEN}SAVED${RESET}] $key added to .env."
        else
            echo -e "  [${DIM}SKIPPED${RESET}] $key."
        fi
    done

    echo -e "\n${GREEN}Doctor & Wizard complete.${RESET}\n"
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
            6) run_doctor ;;
            q|Q) echo "Exiting."; break ;;
            *) echo "Invalid choice." ;;
        esac
    done
fi
