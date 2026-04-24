#!/bin/bash
set -euo pipefail

export OHC_MULTITENANT=false
export OHC_SOURCE_MODE=standalone
export GOMAXPROCS=2
export OHC_STANDALONE=true
export GOMEMLIMIT="256MiB"
export GOGC=50
export LOG_FORMAT="text"
export LOG_LEVEL="info"
export OHC_RUNTIME_DIR=".ohc/runtime"
export OHC_MEMORY_DIR="${OHC_RUNTIME_DIR}/memory"
export OHC_STATUS_DIR="${OHC_RUNTIME_DIR}/status"
export DB_PATH="${OHC_RUNTIME_DIR}/swarm.db"

mkdir -p "${OHC_MEMORY_DIR}" "${OHC_STATUS_DIR}"

echo "Starting OHC Standalone Desktop Wrapper..."

if [ ! -f "$DB_PATH" ]; then
    echo "Initializing local SQLite Database at $DB_PATH..."
    sqlite3 "$DB_PATH" "VACUUM;"
fi

echo "Starting Backend Process..."
bazelisk run //srcs/server:ohc </dev/null &
BACKEND_PID=$!

echo "Starting Frontend App..."
cd srcs/app
flutter run -d web-server </dev/null &
FRONTEND_PID=$!
cd ../..

echo "Starting Lightweight Prometheus Agent..."
docker-compose -f deploy/docker-compose.yml up -d prometheus grafana

cleanup() {
  echo "Shutting down OHC Standalone Desktop Wrapper..."
  if kill -0 $FRONTEND_PID 2>/dev/null; then
    kill $FRONTEND_PID
  fi
  if kill -0 $BACKEND_PID 2>/dev/null; then
    kill $BACKEND_PID
  fi
  docker-compose -f deploy/docker-compose.yml stop prometheus grafana || true
}
trap cleanup SIGINT SIGTERM EXIT

wait $BACKEND_PID
wait $FRONTEND_PID
echo "OHC Standalone Desktop Wrapper exited."
