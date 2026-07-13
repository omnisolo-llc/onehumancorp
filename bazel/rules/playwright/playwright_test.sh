#!/bin/bash
set -euo pipefail

if [[ -z "${TEST_SRCDIR:-}" || -z "${TEST_TMPDIR:-}" ]]; then
  echo "[playwright] Error: Playwright tests must run under Bazel with TEST_SRCDIR and TEST_TMPDIR set." >&2
  exit 1
fi

RUNFILES_ROOT="${RUNFILES_ROOT:-}"
if [[ -z "$RUNFILES_ROOT" ]]; then
  if [[ -n "${TEST_WORKSPACE:-}" && -d "$TEST_SRCDIR/$TEST_WORKSPACE" ]]; then
    RUNFILES_ROOT="$TEST_SRCDIR/$TEST_WORKSPACE"
  elif [[ -d "$TEST_SRCDIR/_main" ]]; then
    RUNFILES_ROOT="$TEST_SRCDIR/_main"
  else
    RUNFILES_ROOT="$TEST_SRCDIR"
  fi
fi

workspace_root="$RUNFILES_ROOT"
if [[ ! -f "$workspace_root/package.json" || ! -d "$workspace_root/node_modules" ]]; then
  echo "[playwright] Error: Bazel runfiles are missing package.json or node_modules under $workspace_root" >&2
  exit 1
fi

SOURCE_REPO_ROOT_CANDIDATES=(
  "${SOURCE_REPO_ROOT:-}"
  "${GITHUB_WORKSPACE:-}"
  "$(pwd)"
  "/home/kevin/mono"
  "/home/runner/work/mono/mono"
  "$workspace_root"
)
for candidate in "${SOURCE_REPO_ROOT_CANDIDATES[@]}"; do
  if [[ -n "$candidate" && -f "$candidate/src/server/lib.rs" ]]; then
    export SOURCE_REPO_ROOT="$(realpath "$candidate")"
    break
  fi
done
export SOURCE_REPO_ROOT="${SOURCE_REPO_ROOT:-$(pwd)}"
if [[ -f "$SOURCE_REPO_ROOT/.env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source "$SOURCE_REPO_ROOT/.env"
  set +a
fi
cd "$workspace_root"

# Resolve spec files to absolute paths if passed as arguments.
ABS_SPEC_FILES=()
for spec_file in "$@"; do
  ABS_SPEC_FILES+=("$(realpath "$spec_file" 2>/dev/null || echo "$spec_file")")
done

playwright_spec_workspace_name() {
  local spec_file="$1"
  local rel="$spec_file"
  for root in "$workspace_root" "$RUNFILES_ROOT"; do
    if [[ -n "$root" && "$rel" == "$root/"* ]]; then
      rel="${rel#$root/}"
      break
    fi
  done
  rel="${rel#./}"
  echo "$rel" | sed -E 's#[^A-Za-z0-9._-]+#__#g'
}

copy_spec_fixtures() {
  local spec_file="$1"
  local fixture_dir
  fixture_dir="$(dirname "$spec_file")/fixtures"
  if [[ -d "$fixture_dir" ]]; then
    mkdir -p "$WORK_DIR/e2e/fixtures"
    cp -R "$fixture_dir/." "$WORK_DIR/e2e/fixtures/"
  fi
}

# Resolve the Bazel-provided Playwright browser repository to an absolute path.
# Every shard gets the same runfiles-backed browser directory instead of a
# per-shard install under the temporary Playwright workspace.
if [[ -n "${PLAYWRIGHT_BROWSERS_PATH:-}" ]]; then
  echo "[playwright] Original browsers path: $PLAYWRIGHT_BROWSERS_PATH"

  BROWSER_PATH_CANDIDATES=(
    "$PLAYWRIGHT_BROWSERS_PATH"
    "$RUNFILES_ROOT/$PLAYWRIGHT_BROWSERS_PATH"
  )
  if [[ -n "${TEST_SRCDIR:-}" ]]; then
    BROWSER_PATH_CANDIDATES+=(
      "$TEST_SRCDIR/$PLAYWRIGHT_BROWSERS_PATH"
    )
  fi
  if [[ -n "${TEST_WORKSPACE:-}" && -n "${TEST_SRCDIR:-}" ]]; then
    BROWSER_PATH_CANDIDATES+=(
      "$TEST_SRCDIR/$TEST_WORKSPACE/$PLAYWRIGHT_BROWSERS_PATH"
    )
  fi

  for candidate in "${BROWSER_PATH_CANDIDATES[@]}"; do
    if [[ -d "$candidate" ]]; then
      export PLAYWRIGHT_BROWSERS_PATH="$(realpath "$candidate")"
      break
    fi
  done

  if [[ ! -d "$PLAYWRIGHT_BROWSERS_PATH" ]]; then
      ACTUAL_SHELL=$(find "$RUNFILES_ROOT" \( -name "chrome-headless-shell" -o -name "headless_shell" \) -type f -executable 2>/dev/null | head -n 1)
      if [[ -n "$ACTUAL_SHELL" ]]; then
          ACTUAL_SHELL_ABS="$(realpath "$ACTUAL_SHELL")"
          export PLAYWRIGHT_BROWSERS_PATH="$(dirname "$(dirname "$(dirname "$ACTUAL_SHELL_ABS")")")"
      fi
  fi

  if [[ -d "$PLAYWRIGHT_BROWSERS_PATH" ]]; then
      export PLAYWRIGHT_BROWSERS_PATH="$HOME/.cache/ms-playwright"
      echo "[playwright] Resolved browsers path: $PLAYWRIGHT_BROWSERS_PATH"
  else
      echo "[playwright] Error: Bazel Playwright browsers path not found: $PLAYWRIGHT_BROWSERS_PATH"
      exit 1
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

# Resolve built-in agent binary path for workflow-spawned agents.
AGENT_BIN=""
for candidate in "src/agents/builtin/ohc-builtin-agent" "../_main/src/agents/builtin/ohc-builtin-agent"; do
  if [[ -x "$candidate" ]]; then
    AGENT_BIN="$(realpath "$candidate")"
    break
  fi
done

export NODE_DISABLE_COMPILE_CACHE=1
export HOME="${HOME:-$TEST_TMPDIR/home}"
mkdir -p "$HOME"

# Run Playwright from a writable project-shaped directory.
WORK_DIR="$TEST_TMPDIR/playwright-workspace"
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
    spec_workspace_name="$(playwright_spec_workspace_name "$abs_spec_file")"
    cp "$abs_spec_file" "$WORK_DIR/src/e2e/$spec_workspace_name"
    copy_spec_fixtures "$abs_spec_file"
    PLAYWRIGHT_SPEC_ARGS+=("src/e2e/$spec_workspace_name")
  done
else
  while IFS= read -r -d '' spec_file; do
    spec_file="$(realpath "$spec_file")"
    spec_workspace_name="$(playwright_spec_workspace_name "$spec_file")"
    cp "$spec_file" "$WORK_DIR/src/e2e/$spec_workspace_name"
    copy_spec_fixtures "$spec_file"
  done < <(
    find "$workspace_root" \
      -path '*/node_modules/*' -prune -o \
      -path '*/.next/*' -prune -o \
      -path '*/e2e/*.spec.ts' -type f -print0
  )
fi

for support_file in fixtures.ts current_app_smoke.ts ai-judge.ts global-setup.ts e2e-seed.sql; do
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

if ! docker info >/dev/null 2>&1; then
  echo "[playwright] Error: Docker is required for Bazel Playwright E2E tests."
  exit 1
fi

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

playwright_port_window_start() {
  local target="${TEST_TARGET:-playwright}"
  if [[ "$target" =~ playwright_shard_([0-9]+)_of_([0-9]+) ]]; then
    local shard_index="${BASH_REMATCH[1]}"
    echo $((30000 + (shard_index - 1) * 20))
    return
  fi

  local hash
  hash="$(printf '%s' "$target" | cksum | awk '{print $1}')"
  echo $((30400 + (hash % 40) * 20))
}

is_port_free() {
  local port="$1"
  python3 - "$port" <<'PY'
import socket
import sys

port = int(sys.argv[1])
with socket.socket() as sock:
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    try:
        sock.bind(("127.0.0.1", port))
    except OSError:
        sys.exit(1)
PY
}

pick_window_port() {
  local window_start="$1"
  local offset="$2"
  local port
  for step in $(seq 0 9); do
    port=$((window_start + offset + step))
    if is_port_free "$port"; then
      echo "$port"
      return
    fi
  done

  pick_free_port
}

cleanup() {
  local exit_code=$?
  if [[ -n "${NEXT_PID:-}" ]]; then
    kill "$NEXT_PID" >/dev/null 2>&1 || true
    wait "$NEXT_PID" >/dev/null 2>&1 || true
  fi
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
  docker rm -f "$POSTGRES_NAME" "$VALKEY_NAME" >/dev/null 2>&1 || true
  exit "$exit_code"
}
trap cleanup EXIT

echo "[playwright] Starting E2E infrastructure..."
echo "[playwright] Pre-pulling docker images with retries..."

postgres_exec() {
  local sql="$1"
  local label="$2"
  for i in $(seq 1 30); do
    if docker exec "$POSTGRES_NAME" psql -v ON_ERROR_STOP=1 -U ohc -d ohc -c "$sql"; then
      return 0
    fi
    if ! docker inspect -f '{{.State.Running}}' "$POSTGRES_NAME" 2>/dev/null | grep -q true; then
      echo "[playwright] Postgres container exited while running: $label"
      docker logs "$POSTGRES_NAME" || true
      return 1
    fi
    sleep 1
  done
  echo "[playwright] Error: failed to run Postgres setup SQL: $label"
  docker logs "$POSTGRES_NAME" || true
  return 1
}

USE_STANDALONE_MODE=false
PULL_PG_SUCCESS=false
for i in {1..3}; do
  if docker pull mirror.gcr.io/pgvector/pgvector:pg15 >/dev/null 2>&1; then
    PULL_PG_SUCCESS=true
    break
  fi
  sleep 2
done

if [ "$PULL_PG_SUCCESS" = true ]; then
  PULL_VK_SUCCESS=false
  for i in {1..3}; do
  if docker pull mirror.gcr.io/valkey/valkey:8-alpine >/dev/null 2>&1; then
      PULL_VK_SUCCESS=true
      break
    fi
    sleep 2
  done

  if [ "$PULL_VK_SUCCESS" = true ]; then
    if docker rm -f "$POSTGRES_NAME" >/dev/null 2>&1 || true; docker run -d --name "$POSTGRES_NAME" -p 127.0.0.1:0:5432 -e POSTGRES_USER=ohc -e POSTGRES_PASSWORD=ohc -e POSTGRES_DB=ohc mirror.gcr.io/pgvector/pgvector:pg15; then
      docker run -d --name "$VALKEY_NAME" -p 127.0.0.1:0:6379 mirror.gcr.io/valkey/valkey:8-alpine
      PG_PORT="$(docker port "$POSTGRES_NAME" 5432/tcp | sed -E 's/.*:([0-9]+)$/\1/' | head -n 1)"
      VK_PORT="$(docker port "$VALKEY_NAME" 6379/tcp | sed -E 's/.*:([0-9]+)$/\1/' | head -n 1)"
      echo "[playwright] E2E infrastructure ports (PG:$PG_PORT VK:$VK_PORT)"
      echo "[playwright] Waiting for postgres on port $PG_PORT..."
      for i in $(seq 1 120); do
        if docker exec "$POSTGRES_NAME" psql -U ohc -d ohc -c "SELECT 1;" >/dev/null 2>&1; then
          break
        fi
        if ! docker inspect -f '{{.State.Running}}' "$POSTGRES_NAME" 2>/dev/null | grep -q true; then
          echo "[playwright] Postgres container exited before readiness. Falling back to Standalone Mode (SQLite)."
          USE_STANDALONE_MODE=true
          break
        fi
        if (( i == 120 )); then
          echo "[playwright] Error: Postgres failed to become ready after 120 seconds. Falling back to Standalone Mode (SQLite)."
          USE_STANDALONE_MODE=true
          break
        fi
        sleep 1
      done

      if [ "$USE_STANDALONE_MODE" = false ]; then
        echo "[playwright] Initializing database roles..."
        postgres_exec "DO \$\$ BEGIN IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN CREATE ROLE ohc_bypassrls NOLOGIN; END IF; END \$\$;" "create ohc_bypassrls role"
        postgres_exec "GRANT ohc_bypassrls TO ohc;" "grant ohc_bypassrls role"
      fi
    else
      echo "[playwright] Docker run for Postgres failed. Falling back to Standalone Mode (SQLite)."
      USE_STANDALONE_MODE=true
    fi
  else
    echo "[playwright] Docker pull for valkey failed. Falling back to Standalone Mode (SQLite)."
    USE_STANDALONE_MODE=true
  fi
else
  echo "[playwright] Docker pull for pgvector failed. Falling back to Standalone Mode (SQLite)."
  USE_STANDALONE_MODE=true
fi
if [[ -z "$SERVER_BIN" ]]; then
  for candidate in "$workspace_root/bazel-bin/src/server/server" "$workspace_root/src/server/server"; do
    if [[ -x "$candidate" ]]; then
      SERVER_BIN="$candidate"
      break
    fi
  done
fi

if [[ -z "$AGENT_BIN" ]]; then
  for candidate in "$workspace_root/bazel-bin/src/agents/builtin/ohc-builtin-agent" "$workspace_root/src/agents/builtin/ohc-builtin-agent"; do
    if [[ -x "$candidate" ]]; then
      AGENT_BIN="$candidate"
      break
    fi
  done
fi

if [[ -n "${MINIMAX_API_KEY:-}" ]]; then
  export OHC_LLM_PROVIDER="${OHC_LLM_PROVIDER:-minimax}"
  export OHC_LLM_MODEL="${OHC_LLM_MODEL:-MiniMax-M3}"
  export MINIMAX_MODEL="${MINIMAX_MODEL:-MiniMax-M3}"
fi
export OHC_AGENT_TASK_TIMEOUT_SECS="${OHC_AGENT_TASK_TIMEOUT_SECS:-240}"
export OHC_LLM_TIMEOUT_SECS="${OHC_LLM_TIMEOUT_SECS:-180}"
if [[ -n "$AGENT_BIN" ]]; then
  if [[ -z "${OHC_BUILTIN_AGENT_BINARY:-}" || ! -x "${OHC_BUILTIN_AGENT_BINARY:-}" ]]; then
    export OHC_BUILTIN_AGENT_BINARY="$AGENT_BIN"
  fi
fi

# Pick ports from a target-specific window. Plain "bind to port 0, close, then
# later start the server" is racy when CI runs all Playwright shard targets in
# parallel.
PORT_WINDOW_START="$(playwright_port_window_start)"
OHC_SERVER_PORT="$(pick_window_port "$PORT_WINDOW_START" 0)"
OHC_GRPC_SERVER_PORT="$(pick_window_port "$PORT_WINDOW_START" 10)"
export OHC_PORT="$OHC_SERVER_PORT"
export OHC_GRPC_PORT="$OHC_GRPC_SERVER_PORT"
export OHC_DEFAULT_TENANT_ID="${OHC_DEFAULT_TENANT_ID:-e2e-tenant}"
export OHC_AGENT_TOKEN="test"
export OHC_AGENT_AUTH_KEY="test_agent_auth_key_must_be_at_least_32_bytes_long_xxxxxxxxx"
export E2E_POSTGRES_CONTAINER="$POSTGRES_NAME"
export API_BASE_URL="http://127.0.0.1:$OHC_SERVER_PORT"
export BACKEND_URL="$API_BASE_URL"
export OHC_BACKEND_URL="$API_BASE_URL"
export OHC_API_URL="$API_BASE_URL"
export OHC_STANDALONE_MODE="${OHC_STANDALONE_MODE:-false}"

if [[ -n "${SERVER_BIN:-}" && -x "${SERVER_BIN:-}" ]]; then
  echo "[playwright] Starting server on ports (API:$OHC_SERVER_PORT gRPC:$OHC_GRPC_SERVER_PORT) from $SERVER_BIN..."
  if [ "$USE_STANDALONE_MODE" = true ]; then
    DB_URL="sqlite://$TEST_TMPDIR/ohc-e2e.db?mode=rwc"
    RD_URL="redis://127.0.0.1:12345"
    OHC_STANDALONE="true"
    export REDIS_URL="redis://127.0.0.1:12345"
  else
    DB_URL="postgres://ohc:ohc@127.0.0.1:$PG_PORT/ohc"
    RD_URL="redis://127.0.0.1:$VK_PORT"
    OHC_STANDALONE="false"
    export REDIS_URL="$RD_URL"
  fi
  export DATABASE_URL="$DB_URL"

  if [[ "$DATABASE_URL" == *"127.0.0.1:5432"* ]] || [[ "$DATABASE_URL" == *"localhost:5432"* ]]; then
    echo "Error: DATABASE_URL is hardcoded to port 5432 ($DATABASE_URL)"
    exit 1
  fi

  DATABASE_URL="$DB_URL" \
  REDIS_URL="$RD_URL" \
  OHC_STANDALONE_MODE="$OHC_STANDALONE" \
  JWT_SECRET="test_jwt_secret_must_be_at_least_32_bytes_long" \
  OHC_SQLITE_KEY="test_sqlite_key" \
  MINIMAX_API_KEY="${MINIMAX_API_KEY:-}" \
  OHC_LLM_PROVIDER="${OHC_LLM_PROVIDER:-}" \
  OHC_LLM_MODEL="${OHC_LLM_MODEL:-}" \
  MINIMAX_MODEL="${MINIMAX_MODEL:-}" \
  OHC_STANDALONE_MODE="$OHC_STANDALONE_MODE" \
  OHC_AGENT_TASK_TIMEOUT_SECS="$OHC_AGENT_TASK_TIMEOUT_SECS" \
  OHC_LLM_TIMEOUT_SECS="$OHC_LLM_TIMEOUT_SECS" \
  OHC_BUILTIN_AGENT_BINARY="${OHC_BUILTIN_AGENT_BINARY:-}" \
  OHC_PORT="$OHC_SERVER_PORT" \
  OHC_GRPC_PORT="$OHC_GRPC_SERVER_PORT" \
  OHC_DEFAULT_TENANT_ID="$OHC_DEFAULT_TENANT_ID" \
    "$SERVER_BIN" >"$TEST_TMPDIR/server.log" 2>&1 &
  SERVER_PID=$!

  echo "[playwright] Waiting for server on port $OHC_SERVER_PORT..."
  for i in $(seq 1 120); do
    if curl -s "http://127.0.0.1:$OHC_SERVER_PORT/api/v1/health" >/dev/null; then
      echo "[playwright] Server is ready and healthy."
      break
    fi
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
      echo "[playwright] Server process died."
      tail -20 "$TEST_TMPDIR/server.log"
      exit 1
    fi
    if (( i == 120 )); then
      echo "[playwright] Error: Server failed to become healthy after 120 seconds."
      tail -50 "$TEST_TMPDIR/server.log"
      exit 1
    fi
    sleep 1
  done
else
  echo "[playwright] Error: server binary not found"
  exit 1
fi

NEXT_APP_ROOT=""
if [[ -n "${NEXT_APP_PACKAGE_JSON:-}" ]]; then
  NEXT_APP_PACKAGE_JSON_CANDIDATES=(
    "$NEXT_APP_PACKAGE_JSON"
    "$RUNFILES_ROOT/$NEXT_APP_PACKAGE_JSON"
    "$workspace_root/$NEXT_APP_PACKAGE_JSON"
  )
  if [[ -n "${TEST_SRCDIR:-}" ]]; then
    NEXT_APP_PACKAGE_JSON_CANDIDATES+=(
      "$TEST_SRCDIR/$NEXT_APP_PACKAGE_JSON"
    )
  fi
  if [[ -n "${TEST_WORKSPACE:-}" && -n "${TEST_SRCDIR:-}" ]]; then
    NEXT_APP_PACKAGE_JSON_CANDIDATES+=(
      "$TEST_SRCDIR/$TEST_WORKSPACE/$NEXT_APP_PACKAGE_JSON"
    )
  fi

  for candidate in "${NEXT_APP_PACKAGE_JSON_CANDIDATES[@]}"; do
    if [[ -f "$candidate" ]]; then
      candidate_dir="$(dirname "$candidate")"
      if [[ -d "$candidate_dir/src/app" && -d "$candidate_dir/node_modules" ]]; then
        NEXT_APP_ROOT="$(cd "$candidate_dir" && pwd)"
        break
      fi
    fi
  done
fi

if [[ -z "$NEXT_APP_ROOT" ]]; then
  for package_json in $(find "$workspace_root" -path '*/node_modules/*' -prune -o -name package.json -print); do
    candidate="$(dirname "$package_json")"
    if [[ -d "$candidate/src/app" ]]; then
      NEXT_APP_ROOT="$(realpath "$candidate")"
      break
    fi
  done
fi

if [[ -n "$NEXT_APP_ROOT" && ! -d "$NEXT_APP_ROOT/src/app" ]]; then
  NEXT_APP_ROOT=""
fi

if [[ -z "$NEXT_APP_ROOT" ]]; then
  for package_json in $(find "$RUNFILES_ROOT" -path '*/node_modules/*' -prune -o -name package.json -print); do
    candidate="$(dirname "$package_json")"
    if [[ -d "$candidate/src/app" ]]; then
      NEXT_APP_ROOT="$(realpath "$candidate")"
      break
    fi
  done
fi

if [[ -n "$NEXT_APP_ROOT" ]]; then
  if [[ ! -f "$NEXT_APP_ROOT/package.json" || ! -d "$NEXT_APP_ROOT/src/app" ]]; then
    NEXT_APP_ROOT=""
  fi
fi

if [[ -z "$NEXT_APP_ROOT" ]]; then
  echo "[playwright] Error: Next UI app not found in Bazel runfiles."
  exit 1
fi

if [[ ! -d "$NEXT_APP_ROOT/node_modules" ]]; then
  if [[ -d "$workspace_root/node_modules" ]]; then
    echo "[playwright] Next node_modules not found in $NEXT_APP_ROOT/node_modules, falling back to $workspace_root/node_modules"
    ln -s "$workspace_root/node_modules" "$NEXT_APP_ROOT/node_modules" || true
  else
    echo "[playwright] Error: Next node_modules not found in Bazel runfiles at $NEXT_APP_ROOT/node_modules and fallback failed"
    exit 1
  fi
fi

NEXT_WORK_DIR="$WORK_DIR/src/ui/next"
mkdir -p "$WORK_DIR/src/ui"
mkdir -p "$NEXT_WORK_DIR"
tar -C "$NEXT_APP_ROOT" \
  --exclude='./node_modules' \
  --exclude='./.next' \
  --exclude='./out' \
  --exclude='./test-results' \
  -cf - . | tar -C "$NEXT_WORK_DIR" -xf -
ln -s "$NEXT_APP_ROOT/node_modules" "$NEXT_WORK_DIR/node_modules"

NEXT_PORT="$(pick_free_port)"
export BASE_URL="http://127.0.0.1:$NEXT_PORT"
export CI=false
export NODE_DISABLE_COMPILE_CACHE=1
echo "[playwright] Starting Next UI on port $NEXT_PORT from $NEXT_WORK_DIR..."
(
  cd "$NEXT_WORK_DIR"
  BACKEND_URL="$API_BASE_URL" \
  OHC_BACKEND_URL="$API_BASE_URL" \
  OHC_API_URL="$API_BASE_URL" \
  NEXT_PUBLIC_E2E=true \
  node ./node_modules/next/dist/bin/next dev --hostname 127.0.0.1 --port "$NEXT_PORT"
) >"$TEST_TMPDIR/next.log" 2>&1 &
NEXT_PID=$!

echo "[playwright] Waiting for Next UI on port $NEXT_PORT..."
for i in $(seq 1 120); do
  if curl -sS -o /dev/null "$BASE_URL/login" >/dev/null 2>&1; then
    echo "[playwright] Next UI is ready."
    break
  fi
  if ! kill -0 "$NEXT_PID" 2>/dev/null; then
    echo "[playwright] Next UI process died."
    tail -50 "$TEST_TMPDIR/next.log"
    exit 1
  fi
  if (( i == 120 )); then
    echo "[playwright] Error: Next UI failed to become ready after 120 seconds."
    tail -80 "$TEST_TMPDIR/next.log"
    exit 1
  fi
  sleep 1
done

export PLAYWRIGHT_LIST_REPORTER="${PLAYWRIGHT_LIST_REPORTER:-1}"
export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1

# Use unique output directories for parallel isolation
BASE_OUTPUT_DIR="${TEST_UNDECLARED_OUTPUTS_DIR:-$TEST_TMPDIR/playwright-results}"
PLAYWRIGHT_OUTPUT_DIR="$BASE_OUTPUT_DIR/results"
export PLAYWRIGHT_HTML_REPORT="$BASE_OUTPUT_DIR/report"
mkdir -p "$PLAYWRIGHT_OUTPUT_DIR"
mkdir -p "$PLAYWRIGHT_HTML_REPORT"
PLAYWRIGHT_SPEC_MANIFEST="$BASE_OUTPUT_DIR/playwright-specs.txt"
PLAYWRIGHT_LIST_LOG="$BASE_OUTPUT_DIR/playwright-list.log"
PLAYWRIGHT_RUN_LOG="$BASE_OUTPUT_DIR/playwright-run.log"
PLAYWRIGHT_SUMMARY="$BASE_OUTPUT_DIR/playwright-summary.md"

# Prepare sharding argument if running under Bazel sharding or a generated
# shard target.
PLAYWRIGHT_SHARD_ARG=""
if [[ -n "${PLAYWRIGHT_SHARD:-}" ]]; then
  PLAYWRIGHT_SHARD_ARG="--shard=${PLAYWRIGHT_SHARD}"
  echo "[playwright] Playwright sharding active: running shard ${PLAYWRIGHT_SHARD}"
elif [[ -n "${TEST_TOTAL_SHARDS:-}" ]]; then
  SHARD_INDEX=$((TEST_SHARD_INDEX + 1))
  PLAYWRIGHT_SHARD_ARG="--shard=${SHARD_INDEX}/${TEST_TOTAL_SHARDS}"
  echo "[playwright] Bazel sharding active: running shard ${SHARD_INDEX} of ${TEST_TOTAL_SHARDS}"
  
  # Advertise sharding support to Bazel by touching the status file
  if [[ -n "${TEST_SHARD_STATUS_FILE:-}" ]]; then
    touch "$TEST_SHARD_STATUS_FILE"
  fi
fi

find src/e2e -maxdepth 1 -name '*.spec.ts' -type f -printf '%P\n' | sort > "$PLAYWRIGHT_SPEC_MANIFEST"
{
  echo "# Playwright Bazel Test Details"
  echo
  echo "- Target: ${TEST_TARGET:-unknown}"
  echo "- Shard: ${PLAYWRIGHT_SHARD:-${TEST_SHARD_INDEX:-none}/${TEST_TOTAL_SHARDS:-none}}"
  echo "- Base URL: ${BASE_URL:-unknown}"
  echo "- Spec files copied into Playwright workspace: $(wc -l < "$PLAYWRIGHT_SPEC_MANIFEST" | tr -d ' ')"
  echo "- HTML report directory: $PLAYWRIGHT_HTML_REPORT"
  echo "- Output directory: $PLAYWRIGHT_OUTPUT_DIR"
  echo
  echo "## Spec Files"
  sed 's/^/- `/' "$PLAYWRIGHT_SPEC_MANIFEST" | sed 's/$/`/'
} > "$PLAYWRIGHT_SUMMARY"

# Run Playwright
if (( ${#PLAYWRIGHT_SPEC_ARGS[@]} > 0 )); then
  echo "[playwright] Validating spec discovery: ${PLAYWRIGHT_SPEC_ARGS[*]}"
  if ! "$PLAYWRIGHT_CLI" test --config ./playwright.config.ts --list "${PLAYWRIGHT_SPEC_ARGS[@]}" ${PLAYWRIGHT_SHARD_ARG} 2>&1 | tee "$PLAYWRIGHT_LIST_LOG"; then
    if grep -q "No tests found" "$PLAYWRIGHT_LIST_LOG"; then
      echo "[playwright] No tests found in selected specs."
    else
      exit 1
    fi
  fi

  echo "[playwright] Running specs: ${PLAYWRIGHT_SPEC_ARGS[*]}"
  set +e
  "$PLAYWRIGHT_CLI" test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR" --workers 1 "${PLAYWRIGHT_SPEC_ARGS[@]}" ${PLAYWRIGHT_SHARD_ARG} 2>&1 | tee "$PLAYWRIGHT_RUN_LOG"
  playwright_status=${PIPESTATUS[0]}
  set -e
  exit "$playwright_status"
else
  echo "[playwright] Listing selected specs/tests"
  if ! "$PLAYWRIGHT_CLI" test --config ./playwright.config.ts --list ${PLAYWRIGHT_SHARD_ARG} 2>&1 | tee "$PLAYWRIGHT_LIST_LOG"; then
    if grep -q "No tests found" "$PLAYWRIGHT_LIST_LOG"; then
      echo "[playwright] No tests found in selected specs."
    else
      exit 1
    fi
  fi

  echo "[playwright] Running all specs on host"
  set +e
  "$PLAYWRIGHT_CLI" test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR" ${PLAYWRIGHT_SHARD_ARG} 2>&1 | tee "$PLAYWRIGHT_RUN_LOG"
  playwright_status=${PIPESTATUS[0]}
  set -e
  exit "$playwright_status"
fi
