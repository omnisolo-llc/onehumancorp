#!/bin/bash
set -uo pipefail

# playwright_test.sh — Bazel sh_test wrapper for individual Playwright specs.
#
# Usage (invoked by Bazel):
#   playwright_test.sh <spec_file.spec.ts>

spec_file="${1:-}"

# Resolve workspace root using package.json symlink to find the actual workspace
# The runfiles have package.json symlinked to /home/kevin/mono/package.json
# We can use this to derive the actual workspace path
workspace_root=""

# Find package.json - first check runfiles, then current dir
pkg_json=""
for dir in "." ".." "../.."; do
  if [[ -f "$dir/package.json" ]]; then
    pkg_json="$dir/package.json"
    break
  fi
done

if [[ -n "$pkg_json" ]]; then
  # Follow symlinks to get the real package.json path
  real_pkg="$(realpath "$pkg_json" 2>/dev/null || echo "$pkg_json")"
  # Get workspace root from package.json's directory (dirname of package.json is workspace root)
  workspace_root="$(dirname "$real_pkg")"
fi

# Fallback to other methods
if [[ -z "$workspace_root" ]] || [[ ! -d "$workspace_root/node_modules" ]]; then
  if [[ -n "${BUILD_WORKSPACE_DIRECTORY:-}" ]]; then
    workspace_root="${BUILD_WORKSPACE_DIRECTORY}"
  elif [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" ]]; then
    workspace_root="$(realpath "${TEST_SRCDIR}/${TEST_WORKSPACE}" 2>/dev/null || echo "")"
  fi
fi

# Final fallback
if [[ -z "$workspace_root" ]] || [[ ! -d "$workspace_root/node_modules" ]]; then
  workspace_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
fi

cd "$workspace_root"
echo "[playwright] Running in $(pwd)"

export HOME="${HOME:-${TEST_TMPDIR:-/tmp}/home}"
mkdir -p "$HOME"

# Unique container names for parallel isolation
CONTAINER_SUFFIX=$(echo "${TEST_TARGET:-playwright}" | md5sum | cut -c1-8)
POSTGRES_NAME="e2e_postgres_${CONTAINER_SUFFIX}"
VALKEY_NAME="e2e_valkey_${CONTAINER_SUFFIX}"

# Random ports for parallel isolation
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
# In Bazel runfiles, the binary is available as:
#   - ./bazel-bin/src/server/server (outside sandbox)
#   - ./src/server/server (symlink in runfiles _main)
SERVER_BIN=""
for candidate in "bazel-bin/src/server/server" "src/server/server"; do
  if [[ -x "$candidate" ]]; then
    SERVER_BIN="$candidate"
    break
  fi
done

if [[ -z "$SERVER_BIN" ]]; then
  # Try finding it via find if the relative paths fail
  SERVER_BIN=$(find . -name server -type f -executable 2>/dev/null | grep -m1 "src/server/server" || echo "")
fi

if [[ -n "${SERVER_BIN:-}" && -x "${SERVER_BIN:-}" ]]; then
  echo "[playwright] Starting server from $SERVER_BIN..."
  DATABASE_URL="postgres://ohc:ohc@127.0.0.1:$PG_PORT/ohc" \
  REDIS_URL="redis://127.0.0.1:$VK_PORT" \
  JWT_SECRET="test_jwt_secret_must_be_at_least_32_bytes_long" \
  OHC_SQLITE_KEY="test_sqlite_key" \
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
  echo "[playwright] Error: server binary not found or not executable at $SERVER_BIN"
  exit 1
fi

# Run Playwright on the host (no Docker for tests)
export CI=true
export BASE_URL="http://localhost:18789"

# Use npx to run playwright - it will find the local installation via package.json
if [[ -n "$spec_file" ]]; then
  echo "[playwright] Running spec: $spec_file"
  # npx will find playwright from the local package.json dependencies
  npx playwright test --config ./playwright.config.ts "$spec_file"
else
  echo "[playwright] Running all specs on host"
  npx playwright test --config ./playwright.config.ts
fi
