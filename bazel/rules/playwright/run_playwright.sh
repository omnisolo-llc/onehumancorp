#!/bin/bash
set -euo pipefail

if [[ -n "${RUNFILES_DIR:-}" ]]; then
    ROOT="${RUNFILES_DIR}/${TEST_WORKSPACE:-mono}"
    if [[ ! -d "${ROOT}/scripts" ]]; then
        ROOT="${RUNFILES_DIR}/mono"
    fi
else
    ROOT="$(pwd)"
fi

SERVER_BIN="${ROOT}/src/server/server"

if [[ ! -e "${SERVER_BIN}" && ! -L "${SERVER_BIN}" ]]; then
    echo "error: server binary not found at ${SERVER_BIN}"
    exit 1
fi

# Run playwright tests
echo "[playwright-e2e] Running playwright tests..."
cd "${ROOT}"
SERVER_BIN="${SERVER_BIN}" node scripts/run-playwright.mjs "$@"

echo "[playwright-e2e] Done"
