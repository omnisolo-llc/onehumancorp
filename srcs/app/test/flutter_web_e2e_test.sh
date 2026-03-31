#!/usr/bin/env bash
# flutter_web_e2e_test.sh — Bazel sh_test wrapper for Flutter web Playwright tests.

set -euo pipefail

WORKSPACE="${TEST_WORKSPACE:-mono}"
RUNFILES="${TEST_SRCDIR:-$PWD}"
TMPDIR="${TEST_TMPDIR:-/tmp/flutter_web_e2e_$$}"

export HOME="${TMPDIR}/home"
export XDG_CONFIG_HOME="${TMPDIR}/xdg-config"
export XDG_CACHE_HOME="${TMPDIR}/xdg-cache"
mkdir -p "${HOME}" "${XDG_CONFIG_HOME}" "${XDG_CACHE_HOME}"

# ── Locate web build artifacts ─────────────────────────────────────────────
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
  echo "WARNING: Flutter web build artifacts not found in expected runfiles paths." >&2
else
  echo "Serving Flutter web from: ${WEB_ARTIFACTS}"
fi

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
if [ -n "$WEB_ARTIFACTS" ]; then
  python3 -m http.server "${PORT}" --directory "${WEB_ARTIFACTS}" &
else
  # Dummy server for CI if web build is bypassed
  python3 -m http.server "${PORT}" &
fi
SERVER_PID=$!

# Also start the backend API server for E2E
OHC_BIN=$(find "${RUNFILES_DIR:-.}" -path "*/cmd/ohc/ohc_/ohc" | head -n 1)
if [[ -n "$OHC_BIN" ]]; then
  echo "Starting OHC Backend..."
  PORT_API=8080 "$OHC_BIN" &
  BACKEND_PID=$!
  trap 'kill ${SERVER_PID} ${BACKEND_PID} 2>/dev/null; rm -rf "${TMPDIR}/pw_results" 2>/dev/null' EXIT
  sleep 2
else
  echo "WARNING: OHC Backend not found, skipping."
  trap 'kill ${SERVER_PID} 2>/dev/null; rm -rf "${TMPDIR}/pw_results" 2>/dev/null' EXIT
fi

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

E2E_TMP_DIR="${TMPDIR}/e2e"
mkdir -p "${E2E_TMP_DIR}"
cp -r "${E2E_DIR}/"* "${E2E_TMP_DIR}/"
CONFIG="${E2E_TMP_DIR}/playwright.config.ts"
export NODE_PATH="${NODE_MODULES_DIR}${NODE_PATH:+:${NODE_PATH}}"

export PLAYWRIGHT_BROWSERS_PATH="${TMPDIR}/pw_browsers"
mkdir -p "${PLAYWRIGHT_BROWSERS_PATH}"

if ! "${PLAYWRIGHT_CMD[@]}" install chromium --with-deps 2>/dev/null; then
  if ! "${PLAYWRIGHT_CMD[@]}" install chromium 2>/dev/null; then
    echo "WARNING: Could not install browser; trying with system browser..." >&2
  fi
fi

OUTPUT_DIR="${TMPDIR}/pw_results"
mkdir -p "${OUTPUT_DIR}"

echo "Running Playwright tests…"
ls -la "${E2E_TMP_DIR}"
echo "TEST_SPEC: ${TEST_SPEC:-}"
if [ -n "${TEST_SPEC:-}" ]; then
  # Playwright requires the path to be accessible. Since we copied everything to E2E_TMP_DIR,
  # we must use that path.
  # We should cd to the E2E_TMP_DIR first because the config might be relative.
  cd "${E2E_TMP_DIR}"
  "${PLAYWRIGHT_CMD[@]}" test "${TEST_SPEC}" \
    --config="playwright.config.ts" \
    --output="${OUTPUT_DIR}" \
    2>&1
else
  cd "${E2E_TMP_DIR}"
  "${PLAYWRIGHT_CMD[@]}" test \
    --config="playwright.config.ts" \
    --output="${OUTPUT_DIR}" \
    2>&1
fi

echo "✓ Playwright web e2e tests completed"
