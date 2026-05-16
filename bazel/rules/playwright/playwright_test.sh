#!/bin/bash
set -uo pipefail

# playwright_test.sh — Bazel sh_test wrapper for individual Playwright specs.
#
# Usage (invoked by Bazel):
#   playwright_test.sh <spec_file.spec.ts>

spec_file="${1:-}"

# Store the original runfiles root
RUNFILES_ROOT="$(pwd)"

# Find package.json and follow symlink to find the real workspace root (where node_modules is)
find_real_workspace_root() {
    local pkg_json=""
    # Check current dir and parents for package.json
    local current_dir="$(pwd)"
    while [[ "$current_dir" != "/" ]]; do
        if [[ -f "$current_dir/package.json" ]]; then
            pkg_json="$current_dir/package.json"
            break
        fi
        current_dir="$(dirname "$current_dir")"
    done

    if [[ -n "$pkg_json" ]]; then
        # Follow symlink to get to the real source tree
        local real_pkg="$(realpath "$pkg_json")"
        echo "$(dirname "$real_pkg")"
        return 0
    fi
    return 1
}

workspace_root=$(find_real_workspace_root)

if [[ -z "$workspace_root" ]] || [[ ! -d "$workspace_root/node_modules" ]]; then
    if [[ -n "${BUILD_WORKSPACE_DIRECTORY:-}" ]]; then
      workspace_root="${BUILD_WORKSPACE_DIRECTORY}"
    fi
fi

if [[ -z "$workspace_root" ]] || [[ ! -d "$workspace_root/node_modules" ]]; then
    workspace_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
fi

# Final check: if we still don't have node_modules, we might be in a pure sandbox
if [[ ! -d "$workspace_root/node_modules" ]]; then
    workspace_root="$(pwd)"
fi

# Resolve spec file to absolute path while still in original directory.
ABS_SPEC_FILE=""
if [[ -n "$spec_file" ]]; then
    ABS_SPEC_FILE="$(realpath "$spec_file" 2>/dev/null || echo "$spec_file")"
fi

# Resolve browsers path to absolute
if [[ -n "${PLAYWRIGHT_BROWSERS_PATH:-}" ]]; then
  echo "[playwright] Original browsers path: $PLAYWRIGHT_BROWSERS_PATH"
  
  # Resolve relative to runfiles root if it starts with ../
  if [[ "$PLAYWRIGHT_BROWSERS_PATH" == ../* ]]; then
      # If we have bazel-out link, we can find the output base
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
  
  # If it's still relative or doesn't exist, try to find it
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

# Run Playwright from a writable project-shaped directory. Some specs and
# reporters write relative paths such as test-results/ and playwright-report/.
# Writing those into Bazel runfiles leaves behind sandbox-owned directories that
# can break later runfiles tree creation.
WORK_DIR="${TEST_TMPDIR:-/tmp}/playwright-workspace"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR/src/server"
ln -s "$workspace_root/package.json" "$WORK_DIR/package.json"
ln -s "$workspace_root/package-lock.json" "$WORK_DIR/package-lock.json"
ln -s "$workspace_root/playwright.config.ts" "$WORK_DIR/playwright.config.ts"
ln -s "$workspace_root/node_modules" "$WORK_DIR/node_modules"
ln -s "$workspace_root/src/e2e" "$WORK_DIR/src/e2e"
ln -s "$workspace_root/src/server/migrations" "$WORK_DIR/src/server/migrations"

if [[ -n "$ABS_SPEC_FILE" ]]; then
  spec_base="$(basename "$ABS_SPEC_FILE")"
  if [[ -f "$WORK_DIR/src/e2e/$spec_base" ]]; then
    ABS_SPEC_FILE="$WORK_DIR/src/e2e/$spec_base"
  fi
fi

cd "$WORK_DIR"

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

if ! docker info >/dev/null 2>&1; then
  echo "[playwright] Error: docker daemon is not available"
  exit 1
fi


echo "[playwright] Skipping docker to use SQLite..."

if [[ -z "$SERVER_BIN" ]]; then
  for candidate in "$workspace_root/bazel-bin/src/server/server" "$workspace_root/src/server/server"; do
    if [[ -x "$candidate" ]]; then
      SERVER_BIN="$candidate"
      break
    fi
  done
fi

if [[ -n "${SERVER_BIN:-}" && -x "${SERVER_BIN:-}" ]]; then
  echo "[playwright] Starting server from $SERVER_BIN..."
  # Clean up any existing DB to avoid schema conflicts
  rm -f "${TEST_TMPDIR:-/tmp}/ohc.db"

  DATABASE_URL="sqlite://${TEST_TMPDIR:-/tmp}/ohc.db"   REDIS_URL="redis://127.0.0.1:6379"   JWT_SECRET="test_jwt_secret_must_be_at_least_32_bytes_long"   OHC_SQLITE_KEY="test_sqlite_key"     "$SERVER_BIN" >"${TEST_TMPDIR:-/tmp}/server.log" 2>&1 &
  SERVER_PID=$!


  echo "[playwright] Waiting for server on port 18789..."
  for i in $(seq 1 120); do
    if curl -s http://127.0.0.1:18789/api/v1/health >/dev/null; then
      echo "[playwright] Server is ready and healthy."
      break
    fi
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
      echo "[playwright] Server process died."
      tail -20 "${TEST_TMPDIR:-/tmp}/server.log"
      exit 1
    fi
    sleep 1
  done
else
  echo "[playwright] Error: server binary not found"
  exit 1
fi

export CI=true
export BASE_URL="http://localhost:18789"
export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1

# Use a unique output directory for parallel isolation
# Bazel provides TEST_UNDECLARED_OUTPUTS_DIR for capturing artifacts
PLAYWRIGHT_OUTPUT_DIR="${TEST_UNDECLARED_OUTPUTS_DIR:-$TEST_TMPDIR/playwright-results}"
mkdir -p "$PLAYWRIGHT_OUTPUT_DIR"
export PLAYWRIGHT_HTML_REPORT="${TEST_UNDECLARED_OUTPUTS_DIR:-$TEST_TMPDIR}/playwright-report"
mkdir -p "$PLAYWRIGHT_HTML_REPORT"

# Use npx to run playwright - it will find the local installation via package.json
if [[ -n "$ABS_SPEC_FILE" ]]; then
  echo "[playwright] Running spec: $ABS_SPEC_FILE"
  echo "[playwright] Listing all discovered tests:"
  npx playwright test --config ./playwright.config.ts --list
  # npx will find playwright from the local package.json dependencies
  npx playwright test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR" "$ABS_SPEC_FILE"
else
  echo "[playwright] Running all specs on host"
  npx playwright test --config ./playwright.config.ts --output "$PLAYWRIGHT_OUTPUT_DIR"
fi
