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
# Tuning memory limits for standalone wrapper
export TOKIO_WORKER_THREADS=2
export MALLOC_ARENA_MAX=2
export RAYON_NUM_THREADS=4
export OHC_STANDALONE=true
export LOG_FORMAT="json"
export LOG_LEVEL="info"
export RUST_LOG="info"
export OHC_RUNTIME_DIR=".ohc/runtime"
export OHC_MEMORY_DIR="${OHC_RUNTIME_DIR}/memory"
export OHC_STATUS_DIR="${OHC_RUNTIME_DIR}/status"

if [ "$OHC_TELEMETRY_ENABLED" != "true" ]; then
  export OHC_TELEMETRY_ENABLED=false
fi

echo -e "${DIM}[1/2] Provisioning local standalone state boundaries...${RESET}"
mkdir -p "${OHC_MEMORY_DIR}/auto/" "${OHC_MEMORY_DIR}/team/" "${OHC_STATUS_DIR}" "${OHC_RUNTIME_DIR}/tmp/" "${OHC_RUNTIME_DIR}/.cache/" "${OHC_RUNTIME_DIR}/downloads/"
chmod 700 "${OHC_RUNTIME_DIR}/tmp/" "${OHC_RUNTIME_DIR}/.cache/" "${OHC_RUNTIME_DIR}/downloads/"
chmod 700 "${OHC_RUNTIME_DIR}" "${OHC_MEMORY_DIR}" "${OHC_STATUS_DIR}" "${OHC_MEMORY_DIR}/auto/" "${OHC_MEMORY_DIR}/team/"
find "${OHC_RUNTIME_DIR}" -type f -exec chmod 600 {} \+
find "${OHC_RUNTIME_DIR}" -type d -exec chmod 700 {} \+

if [ -z "$OHC_SQLITE_KEY" ]; then
  KEY_FILE="${OHC_RUNTIME_DIR}/.sqlite_key"
  if [ ! -f "$KEY_FILE" ]; then
    (umask 077 && openssl rand -hex 32 > "$KEY_FILE")
    chmod 600 "$KEY_FILE"
  fi
  export OHC_SQLITE_KEY="$(cat "$KEY_FILE")"
fi

echo -e "${DIM}[2/2] Launching internal standalone architecture...${RESET}"

# Build optimized binaries instead of running through Bazelisk repeatedly
echo -e "${DIM}  Compiling optimized binaries...${RESET}"
# Optimize caching
npx @bazel/bazelisk build -c opt --disk_cache=~/.cache/bazel-disk-cache //src/server:server //src/ui/tauri:app --//src/ui/tauri:build_tauri=true > /dev/null 2>&1
echo -e "  ${GREEN}✓ Binaries compiled${RESET}"

# Prune stale memory files (older than 60 mins) periodically to prevent unbounded growth
(while true; do
  find "${OHC_MEMORY_DIR}" -type f -mmin +60 -delete > /dev/null 2>&1
  # Resource Cleanup: Also clean unbounded tmp, cache, and download directories
  find "${OHC_RUNTIME_DIR}/tmp/" -type f -mmin +60 -delete > /dev/null 2>&1 || true
  find "${OHC_RUNTIME_DIR}/.cache/" -type f -mmin +60 -delete > /dev/null 2>&1 || true
  find "${OHC_RUNTIME_DIR}/downloads/" -type f -mmin +60 -delete > /dev/null 2>&1 || true
  sleep 3600
done) &
PRUNE_PID=$!

# Launch the API Server (local persistence)
./bazel-bin/src/server/server &
SERVER_PID=$!
echo -e "  ${GREEN}✓ Server started with PID $SERVER_PID${RESET}"

# Launch the UI Desktop wrapper
echo -e "${DIM}  Waiting for backend to be ready...${RESET}"
until curl -s http://localhost:8080/health > /dev/null 2>&1; do
  sleep 1
done

./bazel-bin/src/ui/tauri/app > /dev/null 2>&1 &
APP_PID=$!
echo -e "  ${GREEN}✓ UI Desktop app started with PID $APP_PID${RESET}"

# Launch the Prometheus agent
if [ "$OHC_TELEMETRY_ENABLED" = "true" ]; then
  docker rm -f ohc-prometheus-agent >/dev/null 2>&1 || true
  docker run --name ohc-prometheus-agent \
    --memory="32m" --cpus="0.05" \
    --log-driver json-file --log-opt max-size=10m --log-opt max-file=3 \
    --network host \
    -v $(pwd)/deploy/docker/prometheus/prometheus-agent.yml:/etc/prometheus/prometheus.yml \
    prom/prometheus:latest --config.file=/etc/prometheus/prometheus.yml --enable-feature=agent > /dev/null 2>&1 &
  PROMETHEUS_PID=$!
  echo -e "  ${GREEN}✓ Prometheus agent started in Docker (resource constrained) with PID $PROMETHEUS_PID${RESET}"
fi

# Trap INT and EXIT signals to gracefully shutdown all local processes
function cleanup {
  echo -e "\n${DIM}[Shutting down Standalone Desktop...]${RESET}"
  # Terminate child processes gracefully
  kill -TERM $APP_PID $SERVER_PID $PRUNE_PID 2>/dev/null || true

  # Resource Cleanup: Clean additional temporary artifact directories
  echo -e "${DIM}  Cleaning temporary artifacts...${RESET}"
  rm -rf "${OHC_STATUS_DIR}"/* 2>/dev/null || true
  find "${OHC_RUNTIME_DIR}/tmp/" -type f -delete > /dev/null 2>&1 || true
  find "${OHC_RUNTIME_DIR}/.cache/" -type f -delete > /dev/null 2>&1 || true
  find "${OHC_RUNTIME_DIR}/downloads/" -type f -delete > /dev/null 2>&1 || true

  docker stop ohc-prometheus-agent > /dev/null 2>&1 || true
  docker rm ohc-prometheus-agent > /dev/null 2>&1 || true

  # Wait for processes to exit
  wait $APP_PID 2>/dev/null || true
  wait $SERVER_PID 2>/dev/null || true
  wait $PRUNE_PID 2>/dev/null || true

  echo -e "${GREEN}✓ Local standalone processes terminated successfully.${RESET}"
}

trap cleanup EXIT INT TERM

echo -e "\n${BOLD}${GREEN}Standalone Runtime is active. Press Ctrl+C to terminate.${RESET}"
# Wait indefinitely for processes
wait $APP_PID $SERVER_PID 2>/dev/null || true
