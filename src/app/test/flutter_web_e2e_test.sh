#!/usr/bin/env bash
# flutter_web_e2e_test.sh — Bazel sh_test wrapper for Flutter web Playwright tests.
#
# Responsibilities:
#   1. Locate the pre-built Flutter web artifacts (from flutter_app.web rule).
#   2. Start a Python HTTP server serving the artifacts on a free port.
#   3. Run Playwright against that server.
#   4. Clean up the server on exit.
#
# Required environment variables (set by Bazel):
#   TEST_SRCDIR   – runfiles root
#   TEST_WORKSPACE – workspace name (default: mono)
#   TEST_TMPDIR   – writable tmpdir

set -euo pipefail

WORKSPACE="${TEST_WORKSPACE:-mono}"
RUNFILES="${TEST_SRCDIR:-$PWD}"
TMPDIR="${TEST_TMPDIR:-/tmp/flutter_web_e2e_$$}"
script_real_dir="$(cd -- "$(dirname -- "$(realpath "${BASH_SOURCE[0]}")")" && pwd)"
WORKSPACE_ROOT="${BUILD_WORKSPACE_DIRECTORY:-$(cd -- "${script_real_dir}/../../.." && pwd)}"

export HOME="${TMPDIR}/home"
export XDG_CONFIG_HOME="${TMPDIR}/xdg-config"
export XDG_CACHE_HOME="${TMPDIR}/xdg-cache"
mkdir -p "${HOME}" "${XDG_CONFIG_HOME}" "${XDG_CACHE_HOME}"

is_complete_web_bundle() {
  local candidate="$1"

  [[ -d "$candidate" ]] || return 1
  [[ -f "$candidate/assets/FontManifest.json" ]] || return 1
}

# ── Locate web build artifacts ─────────────────────────────────────────────
# Depending on rule naming, Bazel may emit either:
#   src/app/app.web_build_artifacts/
#   src/app/app_web_build_artifacts/
WEB_ARTIFACTS=""
WORKSPACE_WEB_BUNDLE="${WORKSPACE_ROOT}/src/app/build/web"
if is_complete_web_bundle "${WORKSPACE_WEB_BUNDLE}"; then
  WEB_ARTIFACTS="${WORKSPACE_WEB_BUNDLE}"
fi

WEB_ARTIFACTS_RELS=(
  "src/app/app.web_build_artifacts"
  "src/app/app_web_build_artifacts"
  "src/app/app_web.web_build_artifacts"
)

if [ -z "$WEB_ARTIFACTS" ]; then
  for rel in "${WEB_ARTIFACTS_RELS[@]}"; do
    for candidate in \
        "${RUNFILES}/${WORKSPACE}/${rel}" \
        "${RUNFILES}/_main/${rel}" \
        "${RUNFILES}/__main__/${rel}"; do
      if is_complete_web_bundle "$candidate"; then
        WEB_ARTIFACTS="$candidate"
        break 2
      fi
    done
  done
fi

if [ -z "$WEB_ARTIFACTS" ] || [ ! -d "$WEB_ARTIFACTS" ]; then
  echo "ERROR: complete Flutter web build artifacts were not found" >&2
  echo "       Build //src/app:app so ${WORKSPACE_WEB_BUNDLE} contains the full bundle." >&2
  exit 1
fi

echo "Serving Flutter web from: ${WEB_ARTIFACTS}"

# ── Pick a free port ───────────────────────────────────────────────────────
PORT=$(python3 -c "
import socket
s = socket.socket()
s.bind(('', 0))
port = s.getsockname()[1]
s.close()
print(port)
")
export PLAYWRIGHT_BASE_URL="http://localhost:${PORT}"
echo "HTTP server on port ${PORT} (${PLAYWRIGHT_BASE_URL})"

# ── Start the Go backend serving the Flutter app ───────────────────────────
# We run the real backend API so the Flutter app can communicate with it
# and we use an ephemeral SQLite DB to ensure deterministic seeded state.

BACKEND_BIN=""
for candidate in \
    "${RUNFILES}/${WORKSPACE}/src/server/ohc_/ohc" \
    "${RUNFILES}/_main/src/server/ohc_/ohc" \
    "${RUNFILES}/__main__/src/server/ohc_/ohc" \
    "${RUNFILES}/${WORKSPACE}/src/server/ohc" \
    "${RUNFILES}/_main/src/server/ohc" \
    "${RUNFILES}/__main__/src/server/ohc"; do
  if [ -x "$candidate" ]; then
    BACKEND_BIN="$candidate"
    break
  fi
done

if [ -z "$BACKEND_BIN" ]; then
  echo "ERROR: Go backend binary not found. Ensure //src/server:ohc is included in data." >&2
  # Fallback to Python server if we can't find the backend binary (e.g. during local dev)
  python3 -m http.server "${PORT}" --directory "${WEB_ARTIFACTS}" &
  SERVER_PID=$!
else
  export FRONTEND_STATIC_DIR="${WEB_ARTIFACTS}"
  export PORT="${PORT}"
  export GRPC_PORT="0"
  export OHC_DB_PATH="${TMPDIR}/ohc_e2e.db"
  export ADMIN_USERNAME="admin"
  export ADMIN_PASSWORD="adminpass123"

  # Seed the backend
  "${BACKEND_BIN}" &
  SERVER_PID=$!
  export API_BASE_URL="http://localhost:${PORT}"
fi

trap 'kill ${SERVER_PID} 2>/dev/null || true; rm -rf "${TMPDIR}/pw_results" 2>/dev/null || true' EXIT

# Wait for server to be ready
READY=0
for i in $(seq 1 30); do
  if curl -sf "http://localhost:${PORT}/" >/dev/null 2>&1; then
    READY=1
    break
  fi
  sleep 0.5
done
if [ "$READY" -eq 0 ]; then
  echo "ERROR: HTTP server did not start within 15 seconds" >&2
  exit 1
fi
echo "✓ HTTP server ready"

# ── Locate Playwright and its config ──────────────────────────────────────
PLAYWRIGHT_BIN=""
for candidate in \
    "${RUNFILES}/${WORKSPACE}/node_modules/.bin/playwright" \
    "${RUNFILES}/${WORKSPACE}/node_modules/@playwright/test/cli.js" \
    "${RUNFILES}/node_modules/.bin/playwright" \
    "${RUNFILES}/node_modules/@playwright/test/cli.js" \
    "$(command -v playwright 2>/dev/null)"; do
  if [ -x "$candidate" ]; then
    PLAYWRIGHT_BIN="$candidate"
    break
  fi
  if [ -f "$candidate" ]; then
    PLAYWRIGHT_BIN="$candidate"
    break
  fi
done

PLAYWRIGHT_CMD=()
if [ -n "$PLAYWRIGHT_BIN" ] && [ -x "$PLAYWRIGHT_BIN" ]; then
  PLAYWRIGHT_CMD=("$PLAYWRIGHT_BIN")
elif [ -n "$PLAYWRIGHT_BIN" ] && [ -f "$PLAYWRIGHT_BIN" ]; then
  NODE_BIN="$(command -v node 2>/dev/null || true)"
  if [ -z "$NODE_BIN" ]; then
    echo "ERROR: node is required to run Playwright CLI (${PLAYWRIGHT_BIN})" >&2
    exit 1
  fi
  PLAYWRIGHT_CMD=("$NODE_BIN" "$PLAYWRIGHT_BIN")
else
  echo "ERROR: Playwright CLI not found in runfiles." >&2
  exit 1
fi

CONFIG_REL="src/app/e2e/playwright.config.ts"
CONFIG=""
for candidate in \
    "${RUNFILES}/${WORKSPACE}/${CONFIG_REL}" \
    "${RUNFILES}/_main/${CONFIG_REL}" \
    "${RUNFILES}/__main__/${CONFIG_REL}"; do
  if [ -f "$candidate" ]; then
    CONFIG="$candidate"
    break
  fi
done

if [ -z "$CONFIG" ]; then
  echo "ERROR: playwright.config.ts not found" >&2
  exit 1
fi



# Run tests from temporary real files to avoid symlink path resolution pulling
# in a different @playwright/test instance from the host workspace.
E2E_TMP_DIR="${TMPDIR}/e2e"
mkdir -p "${E2E_TMP_DIR}"
cp "${CONFIG}" "${E2E_TMP_DIR}/playwright.config.ts"

for spec_rel in src/app/e2e/*.spec.ts; do
  for candidate in \
      "${RUNFILES}/${WORKSPACE}/${spec_rel}" \
      "${RUNFILES}/_main/${spec_rel}" \
      "${RUNFILES}/__main__/${spec_rel}"; do
    if [ -f "$candidate" ]; then
      cp "$candidate" "${E2E_TMP_DIR}/"
      break
    fi
  done
done

CONFIG="${E2E_TMP_DIR}/playwright.config.ts"
export NODE_PATH="${NODE_PATH:-}"

# ── Install Playwright browsers if needed ─────────────────────────────────
export PLAYWRIGHT_BROWSERS_PATH="${TMPDIR}/pw_browsers"
mkdir -p "${PLAYWRIGHT_BROWSERS_PATH}"

# In sandboxed environments, --with-deps may fail due lack of root privileges.
if ! "${PLAYWRIGHT_CMD[@]}" install chromium 2>/dev/null; then
  # Fall back – if install still fails, try with any preinstalled browser.
  echo "WARNING: Could not install browser; trying with system browser..." >&2
fi

# ── Run tests ─────────────────────────────────────────────────────────────
OUTPUT_DIR="${TMPDIR}/pw_results"
mkdir -p "${OUTPUT_DIR}"

echo "Running Playwright tests…"
npm install -g playwright && npx playwright test \
  --config="${CONFIG}" \
  --output="${OUTPUT_DIR}" \
  2>&1

echo "✓ Playwright web e2e tests completed"
