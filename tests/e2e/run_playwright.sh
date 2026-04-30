#!/bin/bash
set -euo pipefail

# Use TEST_SRCDIR and TEST_WORKSPACE from Bazel
SRCDIR="${TEST_SRCDIR:-$(pwd)}"
WORKSPACE="${TEST_WORKSPACE:-mono}"
ROOT="${SRCDIR}"

# The server binary is at //src/server:server
# When using Bazel runfiles, it's at TEST_SRCDIR/TEST_WORKSPACE/bazel-bin/src/server/server
SERVER_BIN="${SRCDIR}/bazel-bin/src/server/server"

if [[ ! -f "${SERVER_BIN}" ]]; then
    echo "error: server binary not found at ${SERVER_BIN}"
    exit 1
fi

# Start docker compose infrastructure
echo "[playwright-e2e] Starting infrastructure..."
docker compose -f "${ROOT}/deploy/docker-compose.e2e.yml" up -d

# Cleanup on exit
cleanup() {
    echo "[playwright-e2e] Cleaning up..."
    docker compose -f "${ROOT}/deploy/docker-compose.e2e.yml" down 2>/dev/null || true
}
trap cleanup EXIT

# Wait for postgres
echo "[playwright-e2e] Waiting for postgres..."
for i in $(seq 1 30); do
    if docker exec "$(docker compose -f "${ROOT}/deploy/docker-compose.e2e.yml ps -q postgres)" pg_isready -U ohc >/dev/null 2>&1; then
        echo "[playwright-e2e] postgres ready"
        break
    fi
    sleep 1
done

# Wait for redis
echo "[playwright-e2e] Waiting for redis..."
for i in $(seq 1 30); do
    if docker exec "$(docker compose -f "${ROOT}/deploy/docker-compose.e2e.yml ps -q redis)" redis-cli ping >/dev/null 2>&1; then
        echo "[playwright-e2e] redis ready"
        break
    fi
    sleep 1
done

# Start server
echo "[playwright-e2e] Starting server..."
DATABASE_URL="postgres://ohc:ohc@localhost:5432/ohc" "${SERVER_BIN}" &
SERVER_PID=$!
sleep 3

# Run playwright tests
echo "[playwright-e2e] Running playwright tests..."
cd "${ROOT}"
node scripts/run-playwright.mjs

# Kill server
kill ${SERVER_PID} 2>/dev/null || true

echo "[playwright-e2e] Done"
