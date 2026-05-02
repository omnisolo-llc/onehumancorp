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

# Chromium is pre-cached at /tmp/ohc-playwright-browsers/chromium-1222
# Skip playwright install since it fails on Ubuntu 26.04
# Use the full Chrome installation from /tmp/chrome-extract which has all required resources
CHROMIUM_SRC="/tmp/chrome-extract/opt/google/chrome/chrome"
PLAYWRIGHT_BROWSERS_PATH="$workspace_root/.playwright-browsers"
mkdir -p "$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64"
if [[ -f "$CHROMIUM_SRC" ]]; then
  echo "[playwright] Chromium found at $CHROMIUM_SRC"
  # Create a local copy that will be accessible in the sandbox
  cp "$CHROMIUM_SRC" "$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/chrome"
  chmod +x "$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/chrome"
  echo "[playwright] Chromium copied to $PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/chrome"
  # Copy ALL required resources - entire chrome directory contents
  cp -r /tmp/chrome-extract/opt/google/chrome/* "$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/" 2>/dev/null || true
  # Copy the chrome-sandbox with correct permissions
  if [[ -f "/tmp/chrome-extract/opt/google/chrome/chrome-sandbox" ]]; then
    cp "/tmp/chrome-extract/opt/google/chrome/chrome-sandbox" "$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/chrome-sandbox"
    chmod 755 "$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/chrome-sandbox"
  fi
else
  echo "[playwright] WARNING: No chromium found at $CHROMIUM_SRC, tests may fail"
fi
# Set the executable path for playwright
export PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH="$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/chrome"
export PLAYWRIGHT_BROWSERS_PATH

# Cleanup handler
cleanup() {
  local exit_code=$?
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
  (cd "$workspace_root" && docker compose -f "$workspace_root/deploy/docker-compose.e2e.yml" down >/dev/null 2>&1) || true
  exit "$exit_code"
}
trap cleanup EXIT

# Copy docker-compose into sandbox since it may be a symlink
mkdir -p "$workspace_root/deploy"
if [[ -L "$workspace_root/deploy/docker-compose.e2e.yml" || "$(readlink -f "$workspace_root/deploy/docker-compose.e2e.yml" 2>/dev/null)" != "$(readlink -f "/home/kevin/mono/deploy/docker-compose.e2e.yml" 2>/dev/null)" ]]; then
  cp /home/kevin/mono/deploy/docker-compose.e2e.yml "$workspace_root/deploy/docker-compose.e2e.yml"
fi

# Ensure fresh infrastructure - stop any existing containers and remove volumes
echo "[playwright] Cleaning up any existing infrastructure..."
(cd "$workspace_root" && docker compose -f "$workspace_root/deploy/docker-compose.e2e.yml" down -v >/dev/null 2>&1) || true

# Start infrastructure
echo "[playwright] Starting E2E infrastructure..."
(cd "$workspace_root" && docker compose -f "$workspace_root/deploy/docker-compose.e2e.yml" up -d) 2>&1

# Wait for postgres
echo "[playwright] Waiting for postgres..."
for i in $(seq 1 60); do
  if pg_isready -h 127.0.0.1 -p 5432 -U ohc >/dev/null 2>&1; then
    echo "[playwright] Postgres is ready!"
    break
  fi
  if nc -z 127.0.0.1 5432 2>/dev/null; then
    echo "[playwright] Postgres port is open!"
    break
  fi
  if [[ $i -eq 60 ]]; then
    echo "[playwright] WARNING: Postgres not ready after 60 seconds, continuing anyway..."
  fi
  sleep 1
done

# Give postgres an extra moment to be fully ready
sleep 2

# Extra wait for any async cleanup
sleep 5

# Start the server binary
# Try to use the standard bazel runfiles path
if [[ -x "${workspace_root}/src/server/server" ]]; then
  SERVER_BIN="${workspace_root}/src/server/server"
elif [[ -x "${workspace_root}/bazel-bin/src/server/server" ]]; then
  SERVER_BIN="${workspace_root}/bazel-bin/src/server/server"
else
  # Search in current directory and subdirectories
  SERVER_BIN="$(find . -name server -type f -executable 2>/dev/null | grep -v test_log | grep -v _test | head -1)" || true
fi

if [[ -n "${SERVER_BIN:-}" && -x "${SERVER_BIN:-}" ]]; then
  echo "[playwright] Cleaning up any lingering processes on ports 18789, 50051, 8081..."
  fuser -k 18789/tcp 2>/dev/null || true
  fuser -k 50051/tcp 2>/dev/null || true
  fuser -k 8081/tcp 2>/dev/null || true

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

# Check server status before running tests
echo "[playwright] Checking server status..."
if nc -z 127.0.0.1 18789 2>/dev/null; then
  echo "[playwright] Server is running on port 18789"
else
  echo "[playwright] Server is NOT running on port 18789"
fi

# Check server log if it exists
if [[ -f "${TEST_TMPDIR:-/tmp}/server.log" ]]; then
  echo "[playwright] Server log contents:"
  cat "${TEST_TMPDIR:-/tmp}/server.log" | tail -50
fi

# Run playwright
if [[ -n "$spec_file" ]]; then
  echo "[playwright] Running spec: $spec_file"
  pnpm exec playwright test --config playwright.config.ts "src/e2e/$spec_file" 2>&1 || echo "[playwright] Playwright exited with code $?"
else
  echo "[playwright] Running all specs"
  pnpm exec playwright test --config playwright.config.ts 2>&1 || echo "[playwright] Playwright exited with code $?"
fi
echo "[playwright] Playwright command finished"
