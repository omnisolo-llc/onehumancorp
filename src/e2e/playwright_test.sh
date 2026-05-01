#!/bin/bash
set -euo pipefail

# playwright_test.sh — Bazel sh_test wrapper for individual Playwright specs.
#
# Usage (invoked by Bazel):
#   playwright_test.sh <spec_file.spec.ts>

spec_file="${1:-}"

# Resolve workspace root from Bazel runfiles
if [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" ]]; then
  workspace_root="${TEST_SRCDIR}/${TEST_WORKSPACE}"
elif [[ -n "${RUNFILES_DIR:-}" && -n "${TEST_WORKSPACE:-}" ]]; then
  workspace_root="${RUNFILES_DIR}/${TEST_WORKSPACE}"
else
  # Fallback for local invocation
  workspace_root="$(pwd)"
fi

# Resolve the server binary
server_bin=""
for candidate in \
  "${workspace_root}/bazel-bin/src/server/server" \
  "${workspace_root}/src/server/server" \
  ; do
  if [[ -x "$candidate" ]]; then
    server_bin="$candidate"
    break
  fi
done

if [[ -z "$server_bin" ]]; then
  echo "Warning: server binary not found in runfiles, tests may fail to start backend" >&2
fi

export HOME="${HOME:-${TEST_TMPDIR:-/tmp}/home}"
mkdir -p "$HOME"

# Ensure playwright browsers are installed (shared cache)
export PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-/tmp/ohc-playwright-browsers}"
mkdir -p "$PLAYWRIGHT_BROWSERS_PATH"

# Find node + npx
if command -v npx &>/dev/null; then
  NPX="npx"
elif [[ -x "${workspace_root}/node_modules/.bin/npx" ]]; then
  NPX="${workspace_root}/node_modules/.bin/npx"
else
  echo "npx not found" >&2
  exit 1
fi

# Install chromium if not cached
if ! find "$PLAYWRIGHT_BROWSERS_PATH" -maxdepth 3 -type f -name 'chrome-headless-shell' 2>/dev/null | grep -q .; then
  echo "[playwright] Installing Chromium..."
  $NPX playwright install chromium >/dev/null 2>&1 || true
fi

# Start infrastructure (postgres, redis via docker compose)
cleanup() {
  local exit_code=$?
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
  # Tear down docker compose
  docker compose -f "${workspace_root}/deploy/docker-compose.e2e.yml" down >/dev/null 2>&1 || true
  exit "$exit_code"
}
trap cleanup EXIT

echo "[playwright] Starting E2E infrastructure..."
docker compose -f "${workspace_root}/deploy/docker-compose.e2e.yml" up -d >/dev/null 2>&1

# Wait for postgres
wait_for_port() {
  local port=$1 label=$2 attempts=${3:-60}
  for ((i = 1; i <= attempts; i++)); do
    if nc -z 127.0.0.1 "$port" 2>/dev/null; then
      return 0
    fi
    sleep 1
  done
  echo "Timed out waiting for $label on port $port" >&2
  return 1
}

wait_for_port 5432 "postgres"

# Start the server binary if available
if [[ -n "$server_bin" ]]; then
  echo "[playwright] Starting server..."
  DATABASE_URL="postgres://ohc:ohc@127.0.0.1:5432/ohc" \
    "$server_bin" >"${TEST_TMPDIR:-/tmp}/server.log" 2>&1 &
  SERVER_PID=$!
  wait_for_port 18789 "server" 30 || true
fi

# Run the specific spec file (or all if not specified)
export CI=true
export BASE_URL="${BASE_URL:-http://localhost:18789}"

if [[ -n "$spec_file" ]]; then
  echo "[playwright] Running spec: $spec_file"
  $NPX playwright test --config "${workspace_root}/playwright.config.ts" "$spec_file"
else
  echo "[playwright] Running all specs"
  $NPX playwright test --config "${workspace_root}/playwright.config.ts"
fi
