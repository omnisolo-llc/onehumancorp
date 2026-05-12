#!/bin/bash
set -uo pipefail

# playwright_test.sh — Bazel sh_test wrapper for individual Playwright specs.
#
# Usage (invoked by Bazel):
#   playwright_test.sh <spec_file.spec.ts>

spec_file="${1:-}"

# Resolve workspace root — we always run from the repo root
if [[ -n "${BUILD_WORKSPACE_DIRECTORY:-}" ]]; then
  workspace_root="${BUILD_WORKSPACE_DIRECTORY}"
elif [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" ]]; then
  workspace_root="${TEST_SRCDIR}/${TEST_WORKSPACE}"
else
  workspace_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
fi

cd "$workspace_root"

export HOME="${HOME:-${TEST_TMPDIR:-/tmp}/home}"
mkdir -p "$HOME"

# Playwright browsers are now handled by the official Docker image.

# Unique container names for parallel isolation
CONTAINER_SUFFIX=$(echo "${TEST_TARGET:-playwright}" | md5sum | cut -c1-8)
POSTGRES_NAME="e2e_postgres_${CONTAINER_SUFFIX}"
VALKEY_NAME="e2e_valkey_${CONTAINER_SUFFIX}"

# Random ports for parallel isolation
# We use a simple retry loop to find an available port if possible, 
# but for now we'll just use a random port in a high range.
PG_PORT=$(shuf -i 20000-30000 -n 1)
VK_PORT=$(shuf -i 30001-40000 -n 1)

# Cleanup handler
cleanup() {
  local exit_code=$?
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
  docker rm -f "$POSTGRES_NAME" "$VALKEY_NAME" >/dev/null 2>&1 || true
  exit "$exit_code"
}
trap cleanup EXIT

echo "[playwright] Starting E2E infrastructure (PG:$PG_PORT VK:$VK_PORT)..."
docker run -d --name "$POSTGRES_NAME" -p "$PG_PORT:5432" -e POSTGRES_USER=ohc -e POSTGRES_PASSWORD=ohc -e POSTGRES_DB=ohc pgvector/pgvector:pg16
docker run -d --name "$VALKEY_NAME" -p "$VK_PORT:6379" valkey/valkey:8-alpine

# Wait for postgres
echo "[playwright] Waiting for postgres on port $PG_PORT..."
for i in $(seq 1 60); do
  if nc -z 127.0.0.1 "$PG_PORT" 2>/dev/null; then
    break
  fi
  sleep 1
done

# Start the server binary
SERVER_BIN="${workspace_root}/src/server/server"

if [[ -n "${SERVER_BIN:-}" && -x "${SERVER_BIN:-}" ]]; then
  echo "[playwright] Starting server from $SERVER_BIN..."
  DATABASE_URL="postgres://ohc:ohc@127.0.0.1:$PG_PORT/ohc" \
  REDIS_URL="redis://127.0.0.1:$VK_PORT" \
    "$SERVER_BIN" >"${TEST_TMPDIR:-/tmp}/server.log" 2>&1 &
  SERVER_PID=$!

  echo "[playwright] Waiting for server on port 18789..."
  for i in $(seq 1 120); do
    if curl -s http://127.0.0.1:18789/api/v1/health >/dev/null; then
      echo "[playwright] Server is ready and healthy."
      break
    fi
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
      echo "[playwright] Server process died. Log:"
      tail -100 "${TEST_TMPDIR:-/tmp}/server.log" 2>/dev/null || true
      exit 1
    fi
    sleep 1
  done
else
  echo "[playwright] Warning: server binary not found at $SERVER_BIN, tests may fail"
fi

# Run the specific spec file via official Playwright Docker image
export CI=true
export BASE_URL="${BASE_URL:-http://localhost:18789}"

PLAYWRIGHT_IMAGE="mcr.microsoft.com/playwright:v1.59.1-noble"

run_playwright() {
  local target="$1"
  echo "[playwright] Streaming workspace to container..."
  
  # Use tar to stream the current directory into the container.
  # This avoids issues with Docker volume mounts in sandboxed CI environments.
  # We use -h to follow symlinks so the container gets the actual content.
  # We exclude node_modules to keep the stream small.
  tar -chf - --exclude=node_modules . | docker run --rm -i \
    --network=host \
    -e CI=true \
    -e BASE_URL="$BASE_URL" \
    "$PLAYWRIGHT_IMAGE" \
    bash -c "set -e && mkdir -p /work && tar -xf - -C /work && cd /work && npx --yes @playwright/test test --config playwright.config.ts $target"
}

if [[ -n "$spec_file" ]]; then
  echo "[playwright] Running spec: $spec_file"
  run_playwright "src/e2e/$spec_file"
else
  echo "[playwright] Running all specs"
  run_playwright ""
fi
