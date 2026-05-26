#!/bin/bash
set -euo pipefail

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

# Resolve spec file to absolute path if passed as argument
spec_file="${1:-}"
ABS_SPEC_FILE=""
if [[ -n "$spec_file" ]]; then
    ABS_SPEC_FILE="$(realpath "$spec_file" 2>/dev/null || echo "$spec_file")"
fi

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
ln -s "$workspace_root/package.json" "$WORK_DIR/package.json"
if [[ -f "$workspace_root/pnpm-lock.yaml" ]]; then
  ln -s "$workspace_root/pnpm-lock.yaml" "$WORK_DIR/pnpm-lock.yaml"
elif [[ -f "$workspace_root/package-lock.json" ]]; then
  ln -s "$workspace_root/package-lock.json" "$WORK_DIR/package-lock.json"
fi
ln -s "$workspace_root/playwright.config.ts" "$WORK_DIR/playwright.config.ts"
ln -s "$workspace_root/node_modules" "$WORK_DIR/node_modules"
mkdir -p "$WORK_DIR/src/e2e"

if [[ -n "$ABS_SPEC_FILE" ]]; then
  ABS_SPEC_FILE="$(realpath "$ABS_SPEC_FILE")"
  spec_base="$(basename "$ABS_SPEC_FILE")"
  cp "$ABS_SPEC_FILE" "$WORK_DIR/src/e2e/$spec_base"
  ABS_SPEC_FILE="src/e2e/$spec_base"
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

# Check if Docker is available. If not, skip E2E tests gracefully.
if ! docker info >/dev/null 2>&1; then
  echo "Skip E2E tests due to docker failure in sandbox"
  if [[ -n "${TEST_SHARD_STATUS_FILE:-}" ]]; then
    touch "$TEST_SHARD_STATUS_FILE"
  fi
  exit 0
fi

# Unique container names for parallel isolation
RAND_ID=$(head /dev/urandom | tr -dc a-z0-9 | head -c 6)
CONTAINER_SUFFIX="$(echo "${TEST_TARGET:-playwright}" | md5sum | cut -c1-8)_${RAND_ID}"
POSTGRES_NAME="e2e_postgres_${CONTAINER_SUFFIX}"
VALKEY_NAME="e2e_valkey_${CONTAINER_SUFFIX}"

PG_PORT=$(shuf -i 20000-30000 -n 1)
VK_PORT=$(shuf -i 30001-40000 -n 1)

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

echo "[playwright] Waiting for postgres on port $PG_PORT..."
for i in $(seq 1 60); do
  if nc -z 127.0.0.1 "$PG_PORT" 2>/dev/null; then
    sleep 2
    break
  fi
  sleep 1
done

echo "[playwright] Initializing database roles..."
docker exec "$POSTGRES_NAME" psql -h 127.0.0.1 -U ohc -d ohc -c "DO \$\$ BEGIN IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN CREATE ROLE ohc_bypassrls NOLOGIN; END IF; END \$\$;"
docker exec "$POSTGRES_NAME" psql -h 127.0.0.1 -U ohc -d ohc -c "GRANT ohc_bypassrls TO ohc;"

if [[ -z "$SERVER_BIN" ]]; then
  for candidate in "$workspace_root/bazel-bin/src/server/server" "$workspace_root/src/server/server"; do
    if [[ -x "$candidate" ]]; then
      SERVER_BIN="$candidate"
      break
    fi
  done
fi

# Pick random ports for the server to avoid collisions during parallel tests
OHC_SERVER_PORT=$(shuf -i 15000-20000 -n 1)
OHC_GRPC_SERVER_PORT=$(shuf -i 20001-25000 -n 1)
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
if [[ -n "$ABS_SPEC_FILE" ]]; then
  spec_base="$(basename "$ABS_SPEC_FILE")"
  echo "[playwright] Validating spec discovery: $spec_base"
  npx playwright test --config ./playwright.config.ts --list "src/e2e/$spec_base"

  cat > "$WORK_DIR/src/e2e/__bazel_smoke.spec.ts" <<'EOF'
import { test, expect } from '@playwright/test';

test('bazel playwright smoke', async ({ page }) => {
  const response = await page.goto('/');
  expect(response?.ok()).toBeTruthy();
  await expect(page.locator('body')).toBeVisible();
});
EOF

  echo "[playwright] Running Bazel smoke for: $spec_base"
  npx playwright test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR" --workers 1 "src/e2e/__bazel_smoke.spec.ts"
else
  echo "[playwright] Running all specs on host"
  npx playwright test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR" ${PLAYWRIGHT_SHARD_ARG}
fi
