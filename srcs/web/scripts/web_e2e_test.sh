#!/usr/bin/env bash
# web_e2e_test.sh — Bazel sh_test wrapper for React web app E2E tests.
#
# Responsibilities:
#   1. Start a simple HTTP server serving the pre-built React web app.
#   2. Run Playwright tests against the server.
#   3. Clean up the server on exit.
#
# All AI model responses are mocked inside the Playwright tests via
# page.route() and page.addInitScript() so no external API is needed.

set -euo pipefail

WORKSPACE="${TEST_WORKSPACE:-mono}"
RUNFILES="${TEST_SRCDIR:-$PWD}"
TMPDIR="${TEST_TMPDIR:-/tmp/web_e2e_$$}"

export HOME="${TMPDIR}/home"
export XDG_CONFIG_HOME="${TMPDIR}/xdg-config"
export XDG_CACHE_HOME="${TMPDIR}/xdg-cache"
mkdir -p "${HOME}" "${XDG_CONFIG_HOME}" "${XDG_CACHE_HOME}"

# Locate the web app build artifacts
WEB_BUILD_DIR="${RUNFILES}/${WORKSPACE}/srcs/web/build"

if [[ ! -d "${WEB_BUILD_DIR}" ]]; then
  echo "ERROR: Web app build directory not found at: ${WEB_BUILD_DIR}" >&2
  echo "Run 'npm run build' in srcs/web first." >&2
  exit 1
fi

# Find a free port
find_free_port() {
  python3 -c "import socket; s=socket.socket(); s.bind(('',0)); print(s.getsockname()[1]); s.close()"
}

PORT=$(find_free_port)
export WEB_APP_BASE_URL="http://localhost:${PORT}"
export PLAYWRIGHT_BASE_URL="${WEB_APP_BASE_URL}"

echo "Starting HTTP server on port ${PORT}..."
python3 -m http.server "${PORT}" --directory "${WEB_BUILD_DIR}" &
HTTP_PID=$!
trap "kill ${HTTP_PID} 2>/dev/null || true" EXIT

# Wait for the server to be ready
for i in $(seq 1 20); do
  if curl -sf "${WEB_APP_BASE_URL}" > /dev/null 2>&1; then
    echo "Server ready at ${WEB_APP_BASE_URL}"
    break
  fi
  if [[ $i -eq 20 ]]; then
    echo "ERROR: Server failed to start within 10 seconds" >&2
    exit 1
  fi
  sleep 0.5
done

# Locate the Playwright runner
PLAYWRIGHT="${RUNFILES}/${WORKSPACE}/node_modules/@playwright/test/cli.js"
if [[ ! -f "${PLAYWRIGHT}" ]]; then
  PLAYWRIGHT="${RUNFILES}/node_modules/@playwright/test/cli.js"
fi

E2E_SPEC_DIR="${RUNFILES}/${WORKSPACE}/srcs/web/e2e"
CONFIG="${RUNFILES}/${WORKSPACE}/srcs/web/playwright.config.ts"

echo "Running Playwright E2E tests..."
node "${PLAYWRIGHT}" test \
  --config "${CONFIG}" \
  "${E2E_SPEC_DIR}"
