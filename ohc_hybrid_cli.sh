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
    echo -e "  ${PURPLE}1)${RESET} Run Initial Setup (Generate default .env, Verify build)"
    echo -e "  ${PURPLE}e)${RESET} Interactive Environment Wizard (Configure API Keys/Ports)"
    echo -e "  ${PURPLE}2)${RESET} Switch Environment Mode (Cloud / Standalone / Headless)"
    echo -e "  ${PURPLE}3)${RESET} Launch Standalone Desktop Mode"
    echo -e "  ${PURPLE}4)${RESET} Launch Cloud Backend"
    echo -e "  ${PURPLE}5)${RESET} Run All Tests"
    echo -e "  ${PURPLE}6)${RESET} Verify System State (Diagnostics)"
    echo -e "  ${PURPLE}7)${RESET} Standalone DB Health Check"
    echo -e "  ${PURPLE}8)${RESET} Seed Database with Mock Data"
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


run_env_wizard() {
    echo -e "${DIM}[Executing deploy/scripts/ohc-env-wizard.sh...]${RESET}"
    if bash deploy/scripts/ohc-env-wizard.sh; then
        echo -e "${GREEN}Environment Wizard completed successfully.${RESET}\n"
    else
        echo -e "${PURPLE}Environment Wizard failed.${RESET}\n"
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
    export OHC_STANDALONE=true

    echo -e "${DIM}[Starting Local SQLite Database]${RESET}"
    # Ensure the standalone local database path exists
    mkdir -p "$HOME/.ohc-local-data"
    export DATABASE_URL="file:$HOME/.ohc-local-data/standalone.db"

    # In standalone mode, we optionally run Prometheus locally
    echo -e "${DIM}[Starting local Prometheus Agent if available]${RESET}"
    if command -v prometheus >/dev/null 2>&1; then
        prometheus --config.file=deploy/docker/prometheus/prometheus-agent.yml \
                   --storage.tsdb.path="$HOME/.ohc-local-data/prometheus" \
                   --web.listen-address="127.0.0.1:9091" > /dev/null 2>&1 &
        PROMETHEUS_PID=$!
    else
         echo -e "${DIM}Local Prometheus not found, skipping standalone observability agent...${RESET}"
    fi

    if ! bazelisk run //:desktop; then
        echo -e "${PURPLE}Failed to launch Standalone Desktop.${RESET}\n"
    fi

    if [ -n "$PROMETHEUS_PID" ]; then
        echo -e "${DIM}[Shutting down local Prometheus Agent]${RESET}"
        kill $PROMETHEUS_PID 2>/dev/null || true
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

    # Redis CLI (Cloud Mode requirement)
    if command -v redis-cli >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ redis-cli installed${RESET} (Cloud Mode Ready)"
    else
        echo -e "  ${PURPLE}✗ redis-cli not found${RESET} (Consider installing for Cloud Mode debugging)"
    fi
    echo ""
}

check_system() {
    verify_dependencies
}

seed_mock_data() {
    echo -e "${DIM}[Executing deploy/scripts/ohc-seed-data.sh...]${RESET}"
    if bash deploy/scripts/ohc-seed-data.sh; then
        echo -e "${GREEN}Mock data seeding attempt finished.${RESET}\n"
    else
        echo -e "${PURPLE}Mock data seeding failed.${RESET}\n"
    fi
}

standalone_db_check() {
    echo -e "${DIM}[Standalone DB Health Check]${RESET}"
    DB_FILE="local_standalone.db"
    if [ ! -f "$DB_FILE" ]; then
        echo -e "  ${PURPLE}✗ Database file $DB_FILE not found!${RESET}"
        echo -e "    Migrations have not been executed or db path is incorrect."
        return
    fi
    echo -e "  ${GREEN}✓ Database file $DB_FILE exists.${RESET}"

    if ! command -v sqlite3 >/dev/null 2>&1; then
        echo -e "  ${PURPLE}✗ sqlite3 not found! Cannot verify internal tables.${RESET}"
        return
    fi

    echo -e "${DIM}Checking internal tables...${RESET}"
    TABLES=$(sqlite3 "$DB_FILE" ".tables")
    if echo "$TABLES" | grep -q "agent_missions" && echo "$TABLES" | grep -q "meeting_rooms"; then
         echo -e "  ${GREEN}✓ Migrations appear successful (found agent_missions and meeting_rooms).${RESET}\n"
    else
         echo -e "  ${PURPLE}✗ Critical tables missing. Migrations might have failed.${RESET}\n"
    fi
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
            e|E) run_env_wizard ;;
            2) switch_mode ;;
            3) launch_desktop ;;
            4) launch_cloud ;;
            5) run_tests ;;
            6) check_system ;;
            7) standalone_db_check ;;
            8) seed_mock_data ;;
            q|Q) echo "Exiting."; break ;;
            *) echo "Invalid choice." ;;
        esac
    done
fi
