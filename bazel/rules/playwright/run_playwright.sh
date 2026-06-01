#!/bin/bash
set -euo pipefail

# Use TEST_SRCDIR and TEST_WORKSPACE from Bazel
SRCDIR="${TEST_SRCDIR:-$(pwd)}"
WORKSPACE="${TEST_WORKSPACE:-mono}"
ROOT="${SRCDIR}"

# The server binary is at //src/server:server
# When using Bazel runfiles, it's at TEST_SRCDIR/TEST_WORKSPACE/bazel-bin/src/server/server
export SERVER_BIN="${SRCDIR}/${WORKSPACE}/src/server/server"

if [[ ! -e "${SERVER_BIN}" && ! -L "${SERVER_BIN}" ]]; then
    # Try alternate location
    export SERVER_BIN="${SRCDIR}/src/server/server"
    if [[ ! -e "${SERVER_BIN}" && ! -L "${SERVER_BIN}" ]]; then
        echo "error: server binary not found at alternate ${SERVER_BIN} either"
        exit 1
    fi
fi

# Run playwright tests
echo "[playwright-e2e] Running playwright tests..."
if [[ -d "${ROOT}/${WORKSPACE}" ]]; then
    cd "${ROOT}/${WORKSPACE}"
else
    cd "${ROOT}"
fi
node scripts/run-playwright.mjs "$@"

echo "[playwright-e2e] Done"
