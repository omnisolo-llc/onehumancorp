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

# Set the executable path for playwright (only relevant if running on host, but kept for compatibility)
export PLAYWRIGHT_BROWSERS_PATH="$workspace_root/.playwright-browsers"
export PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH="$PLAYWRIGHT_BROWSERS_PATH/chromium-1222/chrome-linux64/chrome"

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

# Copy docker-compose into sandbox if it's missing or a symlink
# In Bazel, the file should be available in the runfiles
if [[ ! -f "$workspace_root/deploy/docker-compose.e2e.yml" ]]; then
  # Try to find it in the runfiles
  DOCKER_COMPOSE_SRC="$(find "$workspace_root" -name docker-compose.e2e.yml -type f | head -1)"
  if [[ -n "$DOCKER_COMPOSE_SRC" ]]; then
    mkdir -p "$workspace_root/deploy"
    cp "$DOCKER_COMPOSE_SRC" "$workspace_root/deploy/docker-compose.e2e.yml"
  fi
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

# Run playwright inside Docker to avoid host dependency issues (Node, glibc, etc.)
# We use --network host so the container can reach the server running on the host port 18789
echo "[playwright] Running tests inside Playwright Docker container..."
docker run --rm --network host \
  -v "$workspace_root:/work" \
  -w /work \
  -e CI=true \
  -e BASE_URL="http://localhost:18789" \
  mcr.microsoft.com/playwright:v1.40.0-jammy \
  sh -c "corepack enable && pnpm install --frozen-lockfile && (pnpm exec playwright install chromium || true) && pnpm exec playwright test --config playwright.config.ts ${spec_file:+src/e2e/$spec_file}"
echo "[playwright] Playwright command finished"
