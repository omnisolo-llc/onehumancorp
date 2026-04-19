#!/usr/bin/env bash
# playwright_test.sh — Bazel sh_test wrapper for the full Playwright E2E suite.
#
# Prerequisites (must be satisfied by the caller / CI environment):
#   1. docker and docker compose are on PATH.
#   2. The OHC stack is running:  cd deploy && docker compose up -d
#      OR the OHC_E2E_BASE_URL env var points to a live instance.
#   3. node_modules are installed:  cd tests/e2e && npm install
#
# Environment variables:
#   OHC_E2E_BASE_URL   — Override the target URL (default: http://localhost:8080)
#   OHC_E2E_ADMIN_USER — Admin username (default: admin)
#   OHC_E2E_ADMIN_PASS — Admin password (default: admin)
set -euo pipefail

repo_name="${TEST_WORKSPACE:-mono}"
root="${TEST_SRCDIR:-$PWD}/${repo_name}"
e2e_dir="${root}/tests/e2e"

echo "[playwright] Running Playwright E2E suite in ${e2e_dir} ..."
cd "${e2e_dir}"

# Install dependencies if not already present.
if [[ ! -d "node_modules" ]]; then
  npm install --silent
fi

# Install browser binaries if not already present.
npx playwright install chromium --with-deps 2>/dev/null || true

# Run the full test suite.
npx playwright test --reporter=list

echo "[playwright] Playwright E2E suite PASSED."
