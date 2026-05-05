#!/bin/bash
set -euo pipefail

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

# Ensure playwright browsers are installed (shared cache)
export PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-/tmp/ohc-playwright-browsers}"
mkdir -p "$PLAYWRIGHT_BROWSERS_PATH"

# Install chromium if not cached
if ! find "$PLAYWRIGHT_BROWSERS_PATH" -maxdepth 3 -type f -name 'chrome-headless-shell' 2>/dev/null | grep -q .; then
  echo "[playwright] Installing Chromium..."
  npx playwright install chromium 2>&1 || true
fi

# Cleanup handler
cleanup() {
  local exit_code=$?
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
  docker rm -f e2e_postgres e2e_redis >/dev/null 2>&1 || true
  exit "$exit_code"
}
trap cleanup EXIT

# Start infrastructure
echo "[playwright] Starting E2E infrastructure..."
docker run -d --privileged --name e2e_postgres -p 5432:5432 -e POSTGRES_USER=ohc -e POSTGRES_PASSWORD=ohc -e POSTGRES_DB=ohc postgres:16-alpine || true && docker run -d --privileged --name e2e_redis -p 6379:6379 redis:7-alpine || true

# Wait for postgres
echo "[playwright] Waiting for postgres..."
for i in $(seq 1 60); do
  if pg_isready -h 127.0.0.1 -p 5432 -U ohc >/dev/null 2>&1 || true; then
    break
  fi
  if nc -z 127.0.0.1 5432 2>/dev/null; then
    break
  fi
  sleep 1
done

# Start the server binary
SERVER_BIN="${workspace_root}/bazel-bin/src/server/server"
if [[ ! -x "$SERVER_BIN" ]]; then
  # Try building it
  SERVER_BIN="$(find "${workspace_root}/bazel-bin" -name server -type f -executable 2>/dev/null | head -1)"
fi

if [[ -n "${SERVER_BIN:-}" && -x "${SERVER_BIN:-}" ]]; then
  echo "[playwright] Starting server from $SERVER_BIN..."
  DATABASE_URL="postgres://ohc:ohc@127.0.0.1:5432/ohc" \
  REDIS_URL="redis://127.0.0.1:6379" \
    "$SERVER_BIN" >"${TEST_TMPDIR:-/tmp}/server.log" 2>&1 &
  SERVER_PID=$!

  # Wait for server
  echo "[playwright] Waiting for server on port 18789..."
  for i in $(seq 1 30); do
    if nc -z 127.0.0.1 18789 2>/dev/null; then
      echo "[playwright] Server is ready."
      break
    fi
    # Check if server crashed
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
      echo "[playwright] Server process died. Log:"
      tail -20 "${TEST_TMPDIR:-/tmp}/server.log" 2>/dev/null || true
      break
    fi
    sleep 1
  done
else
  echo "[playwright] Warning: server binary not found, tests may fail"
fi

# Run the specific spec file
export CI=true
export BASE_URL="${BASE_URL:-http://localhost:18789}"

if [[ -n "$spec_file" ]]; then
  echo "[playwright] Running spec: $spec_file"
  npx playwright test --config playwright.config.ts "src/e2e/$spec_file"
else
  echo "[playwright] Running all specs"
  npx playwright test --config playwright.config.ts
fi
