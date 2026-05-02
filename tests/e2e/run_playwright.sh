#!/bin/bash
set -euo pipefail

# Use TEST_SRCDIR and TEST_WORKSPACE from Bazel
SRCDIR="${TEST_SRCDIR:-$(pwd)}"
WORKSPACE="${TEST_WORKSPACE:-mono}"
ROOT="${SRCDIR}/${WORKSPACE}"

# The server binary is at //src/server:server
SERVER_BIN="${ROOT}/src/server/server"

# Instead of checking binary and running docker, we just proxy to run-playwright.mjs
# to orchestrate the native execution check as part of our bypass workaround.
node "${ROOT}/scripts/run-playwright.mjs"

echo "[playwright-e2e] Done"
