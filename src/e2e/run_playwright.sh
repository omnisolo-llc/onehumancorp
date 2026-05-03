#!/bin/bash
set -euo pipefail

# Use TEST_SRCDIR and TEST_WORKSPACE from Bazel
SRCDIR="${TEST_SRCDIR:-$(pwd)}"
WORKSPACE="${TEST_WORKSPACE:-mono}"
ROOT="${SRCDIR}"

# The server binary is at //src/server:server
# When using Bazel runfiles, it's at TEST_SRCDIR/TEST_WORKSPACE/src/server/server
SERVER_BIN="${SRCDIR}/${WORKSPACE}/src/server/server"

if [[ ! -f "${SERVER_BIN}" ]]; then
  if [[ -f "${SRCDIR}/src/server/server" ]]; then
    SERVER_BIN="${SRCDIR}/src/server/server"
  else
    echo "error: server binary not found at ${SERVER_BIN} or ${SRCDIR}/src/server/server"
    exit 1
  fi
fi

# Run playwright tests
echo "[playwright-e2e] Running playwright tests..."
cd "${ROOT}"
node scripts/run-playwright.mjs

echo "[playwright-e2e] Done"
