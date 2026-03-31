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

export HOME="${TMPDIR}/home"
export XDG_CONFIG_HOME="${TMPDIR}/xdg-config"
export XDG_CACHE_HOME="${TMPDIR}/xdg-cache"
mkdir -p "${HOME}" "${XDG_CONFIG_HOME}" "${XDG_CACHE_HOME}"

# ── Locate web build artifacts ─────────────────────────────────────────────
# Depending on rule naming, Bazel may emit either:
#   srcs/app/app_web.web_build_artifacts/
#   srcs/app/app_web_build_artifacts/
WEB_ARTIFACTS=""
WEB_ARTIFACTS_RELS=(
  "srcs/app/app_web.web_build_artifacts"
  "srcs/app/app_web_build_artifacts"
)

for rel in "${WEB_ARTIFACTS_RELS[@]}"; do
  for candidate in \
      "${RUNFILES}/${WORKSPACE}/${rel}" \
      "${RUNFILES}/_main/${rel}" \
      "${RUNFILES}/__main__/${rel}"; do
    if [ -d "$candidate" ]; then
      WEB_ARTIFACTS="$candidate"
      break 2
    fi
  done
done

if [ -z "$WEB_ARTIFACTS" ] || [ ! -d "$WEB_ARTIFACTS" ]; then
  echo "ERROR: Flutter web build artifacts not found in expected runfiles paths" >&2
  echo "       Make sure //srcs/app:app_web is built before running this test." >&2
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

# ── Start Python HTTP server ───────────────────────────────────────────────
python3 -m http.server "${PORT}" --directory "${WEB_ARTIFACTS}" &
SERVER_PID=$!
trap 'kill ${SERVER_PID} 2>/dev/null; rm -rf "${TMPDIR}/pw_results" 2>/dev/null' EXIT

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

CONFIG_REL="srcs/app/e2e/playwright.config.ts"
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

# We dynamically accept any spec file based on Bazel TARGET
# We will just copy the entire e2e dir so all specs are available.
E2E_DIR_REL="srcs/app/e2e"
E2E_DIR=""
for candidate in \
    "${RUNFILES}/${WORKSPACE}/${E2E_DIR_REL}" \
    "${RUNFILES}/_main/${E2E_DIR_REL}" \
    "${RUNFILES}/__main__/${E2E_DIR_REL}"; do
  if [ -d "$candidate" ]; then
    E2E_DIR="$candidate"
    break
  fi
done

if [ -z "$E2E_DIR" ]; then
  echo "ERROR: e2e directory not found" >&2
  exit 1
fi

NODE_MODULES_DIR=""
for candidate in \
    "${RUNFILES}/${WORKSPACE}/node_modules" \
    "${RUNFILES}/_main/node_modules" \
    "${RUNFILES}/__main__/node_modules"; do
  if [ -d "$candidate" ]; then
    NODE_MODULES_DIR="$candidate"
    break
  fi
done

if [ -z "$NODE_MODULES_DIR" ]; then
  echo "ERROR: node_modules not found in runfiles" >&2
  exit 1
fi

# Run tests from temporary real files to avoid symlink path resolution pulling
# in a different @playwright/test instance from the host workspace.
E2E_TMP_DIR="${TMPDIR}/e2e"
mkdir -p "${E2E_TMP_DIR}"
cp -r "${E2E_DIR}/"* "${E2E_TMP_DIR}/"
CONFIG="${E2E_TMP_DIR}/playwright.config.ts"
export NODE_PATH="${NODE_MODULES_DIR}${NODE_PATH:+:${NODE_PATH}}"

# ── Install Playwright browsers if needed ─────────────────────────────────
export PLAYWRIGHT_BROWSERS_PATH="${TMPDIR}/pw_browsers"
mkdir -p "${PLAYWRIGHT_BROWSERS_PATH}"

# In sandboxed environments, --with-deps may fail due lack of root privileges.
if ! "${PLAYWRIGHT_CMD[@]}" install chromium --with-deps 2>/dev/null; then
  if ! "${PLAYWRIGHT_CMD[@]}" install chromium 2>/dev/null; then
    # Fall back – if install still fails, try with any preinstalled browser.
    echo "WARNING: Could not install browser; trying with system browser..." >&2
  fi
fi

# ── Run tests ─────────────────────────────────────────────────────────────
OUTPUT_DIR="${TMPDIR}/pw_results"
mkdir -p "${OUTPUT_DIR}"

echo "Running Playwright tests…"
if [ -n "${TEST_SPEC:-}" ]; then
  "${PLAYWRIGHT_CMD[@]}" test "${E2E_TMP_DIR}/${TEST_SPEC}" \
    --config="${CONFIG}" \
    --output="${OUTPUT_DIR}" \
    2>&1
else
  "${PLAYWRIGHT_CMD[@]}" test \
    --config="${CONFIG}" \
    --output="${OUTPUT_DIR}" \
    2>&1
fi

echo "✓ Playwright web e2e tests completed"
