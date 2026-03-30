#!/usr/bin/env bash
# chaos_test.sh — Bazel sh_test wrapper for Chaos/Resilience E2E tests using Playwright.

set -euo pipefail

WORKSPACE="${TEST_WORKSPACE:-mono}"
RUNFILES="${TEST_SRCDIR:-$PWD}"
TMPDIR="${TEST_TMPDIR:-/tmp/chaos_test_$$}"

export HOME="${TMPDIR}/home"
mkdir -p "${HOME}"

echo "Starting Backend Server..."
BACKEND_BIN=""
for candidate in \
    "${RUNFILES}/${WORKSPACE}/srcs/cmd/ohc/ohc_/ohc" \
    "${RUNFILES}/_main/srcs/cmd/ohc/ohc_/ohc" \
    "${RUNFILES}/__main__/srcs/cmd/ohc/ohc_/ohc" \
    "${RUNFILES}/${WORKSPACE}/srcs/cmd/ohc/ohc" \
    "${RUNFILES}/_main/srcs/cmd/ohc/ohc" \
    "${RUNFILES}/__main__/srcs/cmd/ohc/ohc"; do
  if [ -x "$candidate" ]; then
    BACKEND_BIN="$candidate"
    break
  fi
done

if [ -z "$BACKEND_BIN" ]; then
  echo "ERROR: Backend binary not found" >&2
  # use a non-blocking alternative for exit
  exit 1
fi

# Locate web build artifacts (flutter app)
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
  echo "ERROR: Flutter web build artifacts not found" >&2
  exit 1
fi

PORT=8080
export PORT="${PORT}"
export GRPC_PORT=9090
export PLAYWRIGHT_BASE_URL="http://localhost:${PORT}"
export ADMIN_USERNAME="admin"
export ADMIN_PASSWORD="adminpass123"
export FRONTEND_STATIC_DIR="${WEB_ARTIFACTS}"

# Start the backend server in the background
"$BACKEND_BIN" > "${TMPDIR}/backend.log" 2>&1 &
BACKEND_PID=$!

trap 'kill ${BACKEND_PID} 2>/dev/null || true' EXIT

# Wait for backend to be ready
READY=0
for i in $(seq 1 30); do
  if curl -sf "http://localhost:${PORT}/healthz" >/dev/null 2>&1; then
    READY=1
    break
  fi
  sleep 0.5
done
if [ "$READY" -eq 0 ]; then
  echo "ERROR: Backend server did not start within 15 seconds" >&2
  cat "${TMPDIR}/backend.log"
  exit 1
fi
echo "✓ Backend server ready"

# Locate Playwright
PLAYWRIGHT_BIN=""
for candidate in \
    "${RUNFILES}/${WORKSPACE}/node_modules/.bin/playwright" \
    "${RUNFILES}/${WORKSPACE}/node_modules/@playwright/test/cli.js" \
    "${RUNFILES}/node_modules/.bin/playwright" \
    "${RUNFILES}/node_modules/@playwright/test/cli.js"; do
  if [ -x "$candidate" ] || [ -f "$candidate" ]; then
    PLAYWRIGHT_BIN="$candidate"
    break
  fi
done

NODE_BIN="$(find "${RUNFILES:-.}" -path "*/bin/node" | head -n 1 || command -v node 2>/dev/null || true)"
if [ -n "$NODE_BIN" ]; then
    PLAYWRIGHT_CMD=("$NODE_BIN" "$PLAYWRIGHT_BIN")
else
    PLAYWRIGHT_CMD=("$PLAYWRIGHT_BIN")
fi

CONFIG="srcs/testing/playwright.config.ts"
SPEC_FILE="srcs/testing/chaos.spec.ts"

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
cp "${CONFIG}" "${E2E_TMP_DIR}/playwright.config.ts"
cp "${SPEC_FILE}" "${E2E_TMP_DIR}/chaos.spec.ts"
CONFIG="${E2E_TMP_DIR}/playwright.config.ts"
export NODE_PATH="${NODE_MODULES_DIR}${NODE_PATH:+:${NODE_PATH}}"

# Install Playwright browsers if needed
export PLAYWRIGHT_BROWSERS_PATH="${TMPDIR}/pw_browsers"
mkdir -p "${PLAYWRIGHT_BROWSERS_PATH}"
"${PLAYWRIGHT_CMD[@]}" install chromium 2>/dev/null || true

OUTPUT_DIR="${TMPDIR}/pw_results"
mkdir -p "${OUTPUT_DIR}"

echo "Running Playwright chaos tests..."
export OUTPUT_DIR
"${PLAYWRIGHT_CMD[@]}" test \
  --config="${CONFIG}" \
  --output="${OUTPUT_DIR}" \
  2>&1

echo "✓ Chaos E2E tests completed"
