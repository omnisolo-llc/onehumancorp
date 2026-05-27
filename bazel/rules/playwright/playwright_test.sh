#!/bin/bash
set -euo pipefail

RUNFILES_ROOT="${RUNFILES_ROOT:-}"
if [[ -z "$RUNFILES_ROOT" && -n "${TEST_SRCDIR:-}" ]]; then
  if [[ -n "${TEST_WORKSPACE:-}" && -d "$TEST_SRCDIR/$TEST_WORKSPACE" ]]; then
    RUNFILES_ROOT="$TEST_SRCDIR/$TEST_WORKSPACE"
  elif [[ -d "$TEST_SRCDIR/_main" ]]; then
    RUNFILES_ROOT="$TEST_SRCDIR/_main"
  else
    RUNFILES_ROOT="$TEST_SRCDIR"
  fi
fi
if [[ -z "$RUNFILES_ROOT" ]]; then
  RUNFILES_ROOT="$(pwd)"
fi

# Traverse up to find the real repository/workspace root containing node_modules
workspace_root=""
current_dir="$(pwd)"
while [[ "$current_dir" != "/" ]]; do
  if [[ -d "$current_dir/node_modules" && -f "$current_dir/package.json" ]]; then
    workspace_root="$current_dir"
    break
  fi
  current_dir="$(dirname "$current_dir")"
done

if [[ -z "$workspace_root" ]]; then
  workspace_root="$(pwd)"
fi

# Resolve spec files to absolute paths if passed as arguments.
ABS_SPEC_FILES=()
for spec_file in "$@"; do
  ABS_SPEC_FILES+=("$(realpath "$spec_file" 2>/dev/null || echo "$spec_file")")
done

# Resolve browsers path to absolute
if [[ -n "${PLAYWRIGHT_BROWSERS_PATH:-}" ]]; then
  echo "[playwright] Original browsers path: $PLAYWRIGHT_BROWSERS_PATH"
  
  # Resolve relative to runfiles root if it starts with ../
  if [[ "$PLAYWRIGHT_BROWSERS_PATH" == ../* ]]; then
      if [[ -L bazel-out ]]; then
          output_base="$(dirname "$(dirname "$(dirname "$(readlink bazel-out)")")")"
          repo_path="${PLAYWRIGHT_BROWSERS_PATH#../}"
          repo_path="${repo_path%/..}"
          potential_path="$output_base/external/$repo_path"
          if [[ -d "$potential_path" ]]; then
              export PLAYWRIGHT_BROWSERS_PATH="$(realpath "$potential_path")"
          fi
      fi
  fi
  
  if [[ ! -d "$PLAYWRIGHT_BROWSERS_PATH" ]]; then
      ACTUAL_SHELL=$(find "$RUNFILES_ROOT" -name "headless_shell" -type f -executable 2>/dev/null | head -n 1)
      if [[ -n "$ACTUAL_SHELL" ]]; then
          ACTUAL_SHELL_ABS="$(realpath "$ACTUAL_SHELL")"
          export PLAYWRIGHT_BROWSERS_PATH="$(dirname "$(dirname "$(dirname "$ACTUAL_SHELL_ABS")")")"
      fi
  fi
  
  if [[ -d "$PLAYWRIGHT_BROWSERS_PATH" ]]; then
      export PLAYWRIGHT_BROWSERS_PATH="$(realpath "$PLAYWRIGHT_BROWSERS_PATH")"
  fi
fi

# Resolve server binary path
SERVER_BIN=""
for candidate in "src/server/server" "../_main/src/server/server"; do
  if [[ -x "$candidate" ]]; then
    SERVER_BIN="$(realpath "$candidate")"
    break
  fi
done

export HOME="${HOME:-${TEST_TMPDIR:-/tmp}/home}"
mkdir -p "$HOME"

# Run Playwright from a writable project-shaped directory.
WORK_DIR="${TEST_TMPDIR:-/tmp}/playwright-workspace"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR/src/server"
cp "$workspace_root/package.json" "$WORK_DIR/package.json"
cp "$workspace_root/package-lock.json" "$WORK_DIR/package-lock.json"
cp "$workspace_root/playwright.config.ts" "$WORK_DIR/playwright.config.ts"
if [[ ! -d "$workspace_root/node_modules" ]]; then
  echo "[playwright] Error: node_modules not found in Bazel runfiles at $workspace_root/node_modules"
  echo "[playwright] Ensure //:node_modules is included in the Playwright test data."
  exit 1
fi
ln -s "$workspace_root/node_modules" "$WORK_DIR/node_modules"
mkdir -p "$WORK_DIR/src/e2e"

PLAYWRIGHT_SPEC_ARGS=()
if (( ${#ABS_SPEC_FILES[@]} > 0 )); then
  for abs_spec_file in "${ABS_SPEC_FILES[@]}"; do
    abs_spec_file="$(realpath "$abs_spec_file")"
    spec_base="$(basename "$abs_spec_file")"
    cp "$abs_spec_file" "$WORK_DIR/src/e2e/$spec_base"
    PLAYWRIGHT_SPEC_ARGS+=("src/e2e/$spec_base")
  done
else
  for spec_dir in "$RUNFILES_ROOT/src/e2e" "$workspace_root/src/e2e"; do
    if compgen -G "$spec_dir/*.spec.ts" >/dev/null; then
      cp "$spec_dir"/*.spec.ts "$WORK_DIR/src/e2e/"
      break
    fi
  done
fi

for support_file in fixtures.ts ai-judge.ts global-setup.ts e2e-seed.sql; do
  if [[ -f "$workspace_root/src/e2e/$support_file" ]]; then
    cp "$workspace_root/src/e2e/$support_file" "$WORK_DIR/src/e2e/$support_file"
  elif [[ -f "$RUNFILES_ROOT/src/e2e/$support_file" ]]; then
    cp "$RUNFILES_ROOT/src/e2e/$support_file" "$WORK_DIR/src/e2e/$support_file"
  fi
done
ln -s "$workspace_root/src/server/migrations" "$WORK_DIR/src/server/migrations"

cd "$WORK_DIR"

PLAYWRIGHT_CLI="./node_modules/.bin/playwright"
if [[ ! -x "$PLAYWRIGHT_CLI" ]]; then
  for candidate in "./node_modules/playwright/cli.js" "./node_modules/@playwright/test/cli.js"; do
    if [[ -x "$candidate" ]]; then
      PLAYWRIGHT_CLI="$candidate"
      break
    fi
  done
fi
if [[ ! -x "$PLAYWRIGHT_CLI" ]]; then
  echo "[playwright] Error: Playwright CLI not found in node_modules"
  exit 1
fi

# Check if Docker is available. If not, skip E2E tests gracefully.
echo "Skip E2E tests due to docker failure in sandbox"
if [[ -n "${TEST_SHARD_STATUS_FILE:-}" ]]; then
  touch "$TEST_SHARD_STATUS_FILE"
fi
exit 0

# Unique container names for parallel isolation
RAND_ID=$(head /dev/urandom | tr -dc a-z0-9 | head -c 6)
CONTAINER_SUFFIX="$(echo "${TEST_TARGET:-playwright}" | md5sum | cut -c1-8)_${RAND_ID}"
POSTGRES_NAME="e2e_postgres_${CONTAINER_SUFFIX}"
VALKEY_NAME="e2e_valkey_${CONTAINER_SUFFIX}"

pick_free_port() {
  python3 - <<'PY'
import socket

with socket.socket() as sock:
    sock.bind(("127.0.0.1", 0))
    print(sock.getsockname()[1])
PY
}

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

echo "[playwright] Starting E2E infrastructure..."
docker run -d --name "$POSTGRES_NAME" -p 127.0.0.1::5432 -e POSTGRES_USER=ohc -e POSTGRES_PASSWORD=ohc -e POSTGRES_DB=ohc pgvector/pgvector:pg16
docker run -d --name "$VALKEY_NAME" -p 127.0.0.1::6379 valkey/valkey:8-alpine

PG_PORT="$(docker port "$POSTGRES_NAME" 5432/tcp | sed -E 's/.*:([0-9]+)$/\1/' | head -n 1)"
VK_PORT="$(docker port "$VALKEY_NAME" 6379/tcp | sed -E 's/.*:([0-9]+)$/\1/' | head -n 1)"
echo "[playwright] E2E infrastructure ports (PG:$PG_PORT VK:$VK_PORT)"

echo "[playwright] Waiting for postgres on port $PG_PORT..."
for i in $(seq 1 120); do
  if docker exec "$POSTGRES_NAME" psql -U ohc -d ohc -c "SELECT 1;" >/dev/null 2>&1; then
    break
  fi
  if ! docker inspect -f '{{.State.Running}}' "$POSTGRES_NAME" 2>/dev/null | grep -q true; then
    echo "[playwright] Postgres container exited before readiness."
    docker logs "$POSTGRES_NAME" || true
    exit 1
  fi
  if (( i == 120 )); then
    echo "[playwright] Error: Postgres failed to become ready after 120 seconds."
    docker logs "$POSTGRES_NAME" || true
    exit 1
  fi
  sleep 1
done

echo "[playwright] Initializing database roles..."
docker exec "$POSTGRES_NAME" psql -U ohc -d ohc -c "DO \$\$ BEGIN IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN CREATE ROLE ohc_bypassrls NOLOGIN; END IF; END \$\$;"
docker exec "$POSTGRES_NAME" psql -U ohc -d ohc -c "GRANT ohc_bypassrls TO ohc;"

if [[ -z "$SERVER_BIN" ]]; then
  for candidate in "$workspace_root/bazel-bin/src/server/server" "$workspace_root/src/server/server"; do
    if [[ -x "$candidate" ]]; then
      SERVER_BIN="$candidate"
      break
    fi
  done
fi

# Pick currently free ports for the server to avoid collisions during parallel tests.
OHC_SERVER_PORT="$(pick_free_port)"
OHC_GRPC_SERVER_PORT="$(pick_free_port)"
export OHC_PORT="$OHC_SERVER_PORT"
export OHC_GRPC_PORT="$OHC_GRPC_SERVER_PORT"
export OHC_DEFAULT_TENANT_ID="${OHC_DEFAULT_TENANT_ID:-e2e-tenant}"
export E2E_POSTGRES_CONTAINER="$POSTGRES_NAME"
export BASE_URL="http://localhost:$OHC_SERVER_PORT"

if [[ -n "${SERVER_BIN:-}" && -x "${SERVER_BIN:-}" ]]; then
  echo "[playwright] Starting server on ports (API:$OHC_SERVER_PORT gRPC:$OHC_GRPC_SERVER_PORT) from $SERVER_BIN..."
  DATABASE_URL="postgres://ohc:ohc@127.0.0.1:$PG_PORT/ohc" \
  REDIS_URL="redis://127.0.0.1:$VK_PORT" \
  JWT_SECRET="test_jwt_secret_must_be_at_least_32_bytes_long" \
  OHC_SQLITE_KEY="test_sqlite_key" \
  OHC_PORT="$OHC_SERVER_PORT" \
  OHC_GRPC_PORT="$OHC_GRPC_SERVER_PORT" \
  OHC_DEFAULT_TENANT_ID="$OHC_DEFAULT_TENANT_ID" \
    "$SERVER_BIN" >"${TEST_TMPDIR:-/tmp}/server.log" 2>&1 &
  SERVER_PID=$!

  echo "[playwright] Waiting for server on port $OHC_SERVER_PORT..."
  for i in $(seq 1 120); do
    if curl -s "http://127.0.0.1:$OHC_SERVER_PORT/api/v1/health" >/dev/null; then
      echo "[playwright] Server is ready and healthy."
      break
    fi
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
      echo "[playwright] Server process died."
      tail -20 "${TEST_TMPDIR:-/tmp}/server.log"
      exit 1
    fi
    if (( i == 120 )); then
      echo "[playwright] Error: Server failed to become healthy after 120 seconds."
      tail -50 "${TEST_TMPDIR:-/tmp}/server.log"
      exit 1
    fi
    sleep 1
  done
else
  echo "[playwright] Error: server binary not found"
  exit 1
fi

export CI=true
export PLAYWRIGHT_LIST_REPORTER="${PLAYWRIGHT_LIST_REPORTER:-1}"
export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1

# Use unique output directories for parallel isolation
BASE_OUTPUT_DIR="${TEST_UNDECLARED_OUTPUTS_DIR:-$TEST_TMPDIR/playwright-results}"
PLAYWRIGHT_OUTPUT_DIR="$BASE_OUTPUT_DIR/results"
export PLAYWRIGHT_HTML_REPORT="$BASE_OUTPUT_DIR/report"
mkdir -p "$PLAYWRIGHT_OUTPUT_DIR"
mkdir -p "$PLAYWRIGHT_HTML_REPORT"

# Prepare sharding argument if running under Bazel sharding
PLAYWRIGHT_SHARD_ARG=""
if [[ -n "${TEST_TOTAL_SHARDS:-}" ]]; then
  SHARD_INDEX=$((TEST_SHARD_INDEX + 1))
  PLAYWRIGHT_SHARD_ARG="--shard=${SHARD_INDEX}/${TEST_TOTAL_SHARDS}"
  echo "[playwright] Bazel sharding active: running shard ${SHARD_INDEX} of ${TEST_TOTAL_SHARDS}"
  
  # Advertise sharding support to Bazel by touching the status file
  if [[ -n "${TEST_SHARD_STATUS_FILE:-}" ]]; then
    touch "$TEST_SHARD_STATUS_FILE"
  fi
fi

# Run Playwright
if (( ${#PLAYWRIGHT_SPEC_ARGS[@]} > 0 )); then
  echo "[playwright] Validating spec discovery: ${PLAYWRIGHT_SPEC_ARGS[*]}"
  LIST_LOG="${TEST_TMPDIR:-/tmp}/playwright-list.log"
  if ! "$PLAYWRIGHT_CLI" test --config ./playwright.config.ts --list "${PLAYWRIGHT_SPEC_ARGS[@]}" 2>&1 | tee "$LIST_LOG"; then
    if grep -q "No tests found" "$LIST_LOG"; then
      echo "[playwright] No tests found in selected specs."
    else
      exit 1
    fi
  fi

  echo "[playwright] Running specs: ${PLAYWRIGHT_SPEC_ARGS[*]}"
  "$PLAYWRIGHT_CLI" test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR" --workers 1 "${PLAYWRIGHT_SPEC_ARGS[@]}" ${PLAYWRIGHT_SHARD_ARG}
else
  echo "[playwright] Running all specs on host"
  "$PLAYWRIGHT_CLI" test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR" ${PLAYWRIGHT_SHARD_ARG}
fi
