#!/bin/bash
# OHC Hybrid Local Standalone Runtime

# Premium aesthetics colors
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC: Local Standalone Desktop Runtime           ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

# Export optimized variables for the local footprint
export OHC_MULTITENANT=false
export OHC_HEADLESS=false
export OHC_SOURCE_MODE=standalone
export GOMAXPROCS=2
export OHC_STANDALONE=true
export GOMEMLIMIT="128MiB"
export GOGC=50
export LOG_FORMAT="json"
export LOG_LEVEL="info"
export OHC_RUNTIME_DIR=".ohc/runtime"
export OHC_MEMORY_DIR="${OHC_RUNTIME_DIR}/memory"
export OHC_STATUS_DIR="${OHC_RUNTIME_DIR}/status"

if [ "$OHC_TELEMETRY_ENABLED" != "true" ]; then
  export OHC_TELEMETRY_ENABLED=false
fi

echo -e "${DIM}[1/2] Provisioning local standalone state boundaries...${RESET}"
mkdir -p "${OHC_MEMORY_DIR}/auto/" "${OHC_MEMORY_DIR}/team/" "${OHC_STATUS_DIR}"

echo -e "${DIM}[2/2] Launching internal standalone architecture...${RESET}"
# Launch the API Server (local persistence)
npx @bazel/bazelisk run //src/server:server &
SERVER_PID=$!
echo -e "  ${GREEN}✓ Server started with PID $SERVER_PID${RESET}"

# Launch the UI Desktop wrapper
echo -e "${DIM}  Waiting for backend to be ready...${RESET}"
until curl -s http://localhost:8080/health > /dev/null 2>&1; do
  sleep 1
done

npx @bazel/bazelisk run //src/app:app > /dev/null 2>&1 &
APP_PID=$!
echo -e "  ${GREEN}✓ UI Desktop app started with PID $APP_PID${RESET}"

# Launch the Prometheus agent
if [ "$OHC_TELEMETRY_ENABLED" = "true" ]; then
  docker rm -f ohc-prometheus-agent >/dev/null 2>&1 || true
  docker run -d --name ohc-prometheus-agent \
    --memory="128m" --cpus="0.1" \
    --log-driver json-file --log-opt max-size=10m --log-opt max-file=3 \
    --network host \
    -v $(pwd)/deploy/docker/prometheus/prometheus-agent.yml:/etc/prometheus/prometheus.yml \
    prom/prometheus:latest --config.file=/etc/prometheus/prometheus.yml --enable-feature=agent > /dev/null 2>&1
  echo -e "  ${GREEN}✓ Prometheus agent started in Docker (resource constrained)${RESET}"
fi

# Trap INT and EXIT signals to gracefully shutdown all local processes
function cleanup {
  echo -e "\n${DIM}[Shutting down Standalone Desktop...]${RESET}"
  kill $APP_PID 2>/dev/null || true
  kill $SERVER_PID 2>/dev/null || true
  docker stop ohc-prometheus-agent > /dev/null 2>&1 || true
  docker rm ohc-prometheus-agent > /dev/null 2>&1 || true
  wait $APP_PID 2>/dev/null || true
  wait $SERVER_PID 2>/dev/null || true
  echo -e "${GREEN}✓ Local standalone processes terminated successfully.${RESET}"
}

trap cleanup EXIT INT

echo -e "\n${BOLD}${GREEN}Standalone Runtime is active. Press Ctrl+C to terminate.${RESET}"
# Wait indefinitely for processes
wait $APP_PID
