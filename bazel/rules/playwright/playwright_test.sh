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
# Incorporate a random component to prevent collisions even if TEST_TARGET is duplicated or missing
RAND_ID=$(head /dev/urandom | tr -dc a-z0-9 | head -c 6)
CONTAINER_SUFFIX="$(echo "${TEST_TARGET:-playwright}" | md5sum | cut -c1-8)_${RAND_ID}"
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

# Check if docker is available
if ! docker info >/dev/null 2>&1; then
  echo "[playwright] Error: docker daemon is not available or /var/run/docker.sock is not accessible."
  echo "[playwright] If running in Bazel sandbox, ensure 'no-sandbox' tag is present or use --sandbox_add_mount_pair=/var/run/docker.sock"
  exit 1
fi

echo "[playwright] Starting E2E infrastructure (PG:$PG_PORT VK:$VK_PORT)..."
docker run -d --name "$POSTGRES_NAME" -p "$PG_PORT:5432" -e POSTGRES_USER=ohc -e POSTGRES_PASSWORD=ohc -e POSTGRES_DB=ohc pgvector/pgvector:pg16
docker run -d --name "$VALKEY_NAME" -p "$VK_PORT:6379" valkey/valkey:8-alpine

# Wait for postgres
echo "[playwright] Waiting for postgres on port $PG_PORT..."
for i in $(seq 1 60); do
  if nc -z 127.0.0.1 "$PG_PORT" 2>/dev/null; then
    # Give postgres a moment to finish starting up even after the port is open
    sleep 2
    break
  fi
  sleep 1
done

echo "[playwright] Initializing database roles..."
docker exec "$POSTGRES_NAME" psql -h 127.0.0.1 -U ohc -d ohc -c "DO \$\$ BEGIN IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN CREATE ROLE ohc_bypassrls NOLOGIN; END IF; END \$\$;"
docker exec "$POSTGRES_NAME" psql -h 127.0.0.1 -U ohc -d ohc -c "GRANT ohc_bypassrls TO ohc;"

echo "[playwright] Workspace root: $workspace_root"
echo "[playwright] Searching for server binary..."

SERVER_BIN=""
# First check runfiles (relative to current sandbox)
for candidate in "src/server/server" "../_main/src/server/server"; do
  if [[ -x "$candidate" ]]; then
    SERVER_BIN="$(realpath "$candidate")"
    echo "[playwright] Found server in runfiles: $SERVER_BIN"
    break
  fi
done

# If not found, check relative to workspace_root
if [[ -z "$SERVER_BIN" ]]; then
  for candidate in "$workspace_root/bazel-bin/src/server/server" "$workspace_root/src/server/server"; do
    if [[ -x "$candidate" ]]; then
      SERVER_BIN="$candidate"
      echo "[playwright] Found server relative to workspace: $SERVER_BIN"
      break
    fi
  done
fi

if [[ -z "$SERVER_BIN" ]]; then
  # Try finding it via find
  SERVER_BIN=$(find "$workspace_root" -name server -type f -executable 2>/dev/null | grep -m1 "src/server/server" || echo "")
  [[ -n "$SERVER_BIN" ]] && echo "[playwright] Found server via find: $SERVER_BIN"
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
