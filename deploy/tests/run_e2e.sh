#!/usr/bin/env bash
# E2E Test Wrapper Script
# Starts the server, runs vitest, and cleans up.
set -euo pipefail

TEST_FILE="$1"
PORT=18080

echo "Starting OHC Backend in headless mode..."
export OHC_HEADLESS=true
export OHC_STANDALONE=false
export DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@localhost:5432/ohc}
export REDIS_ADDR=${REDIS_ADDR:-localhost:6379}

# Start server
# Note: In Bazel, we use the binary path provided in data
./src/server/server-rust &
SERVER_PID=$!

cleanup() {
  echo "Stopping server (PID: $SERVER_PID)..."
  kill $SERVER_PID 2>/dev/null || true
}
trap cleanup EXIT

# Wait for server to be healthy
echo "Waiting for server to be healthy on port $PORT..."
MAX_ATTEMPTS=60
ATTEMPT=0
while [[ $ATTEMPT -lt $MAX_ATTEMPTS ]]; do
  if curl -sf http://127.0.0.1:$PORT/healthz >/dev/null 2>&1; then
    echo "Server is healthy!"
    break
  fi
  ATTEMPT=$((ATTEMPT + 1))
  sleep 1
done

if [[ $ATTEMPT -eq $MAX_ATTEMPTS ]]; then
  echo "Server failed to become healthy"
  exit 1
fi

# Run Vitest
echo "Running Vitest for $TEST_FILE..."
npx vitest run "$TEST_FILE"
