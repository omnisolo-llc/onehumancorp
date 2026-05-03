#!/bin/bash
set -e

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
RED="\033[38;5;196m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC Standalone Desktop Wrapper Lifecycle        ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

# Handle graceful shutdown
cleanup() {
    echo -e "\n${DIM}[Shutting down Standalone Wrapper]${RESET}"
    if [ -n "$SERVER_PID" ]; then
        echo -e "Stopping Server (PID: $SERVER_PID)..."
        kill -TERM "$SERVER_PID" 2>/dev/null || true
    fi
    if [ -n "$APP_PID" ]; then
        echo -e "Stopping Desktop App (PID: $APP_PID)..."
        kill -TERM "$APP_PID" 2>/dev/null || true
    fi
    wait "$SERVER_PID" 2>/dev/null || true
    wait "$APP_PID" 2>/dev/null || true
    echo -e "${DIM}[Stopping local dependencies]${RESET}"
    docker-compose -f deploy/docker-compose.yml stop postgres redis 2>/dev/null || true
    echo -e "${GREEN}✓ All processes stopped successfully.${RESET}"
}
trap cleanup SIGINT SIGTERM EXIT

# Source standalone environment
echo -e "${DIM}[1/4] Configuring standalone environment...${RESET}"
if [ -f "deploy/scripts/ohc-mode.sh" ]; then
    source deploy/scripts/ohc-mode.sh standalone
else
    echo -e "  ${PURPLE}✗ Could not find deploy/scripts/ohc-mode.sh${RESET}"
    exit 1
fi

echo -e "${DIM}[2/4] Starting local dependencies (Postgres & Redis)...${RESET}"
docker-compose -f deploy/docker-compose.yml up -d postgres redis
export DATABASE_URL="postgres://ohc:ohc@localhost:5432/ohc?sslmode=disable"
export REDIS_URL="redis://localhost:6379"

echo -e "${DIM}[3/4] Pre-building binaries...${RESET}"
bazelisk build //src/server:server //src/app:app
echo -e "  ${GREEN}✓ Build complete${RESET}"

echo -e "${DIM}[4/4] Starting processes...${RESET}"
export LOG_LEVEL="info"
export OHC_STANDALONE=true

bazelisk run //src/server:server &
SERVER_PID=$!
echo -e "  ${GREEN}✓ Server started with PID $SERVER_PID${RESET}"

# Give server a moment to bind ports and initialize db
sleep 5

bazelisk run //src/app:app &
APP_PID=$!
echo -e "  ${GREEN}✓ App started with PID $APP_PID${RESET}"

echo -e "\n${BOLD}${GREEN}Standalone environment is fully operational!${RESET}"
echo -e "${DIM}Press Ctrl+C to stop all processes.${RESET}"

# Wait indefinitely until interrupted
wait
