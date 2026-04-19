#!/usr/bin/env bash
set -euo pipefail

spec_path="${1:-}"

if [[ -z "$spec_path" ]]; then
  echo "Usage: $0 <spec_path>" >&2
  exit 1
fi

# Resolve the Bazel workspace root.
# In Bazel 9 + Bzlmod the main workspace is named "_main"; TEST_WORKSPACE is set
# automatically by Bazel so this default only fires in non-Bazel invocations.
repo_name="${TEST_WORKSPACE:-_main}"
root="${TEST_SRCDIR:-$PWD}/${repo_name}"
e2e_dir="${root}/srcs/tests/e2e"

if [[ ! -d "$e2e_dir" ]]; then
  echo "ERROR: e2e directory not found at ${e2e_dir}" >&2
  exit 1
fi

cd "$e2e_dir"

# Skip gracefully when the OHC server is not reachable.
# This allows `bazel test //...` to pass on machines without a running stack.
# Use a short timeout so CI doesn't stall when the server is absent.
base_url="${OHC_E2E_BASE_URL:-http://localhost:8080}"
if ! curl -sf --connect-timeout 2 --max-time 3 "${base_url}/healthz" &>/dev/null &&
   ! curl -sf --connect-timeout 2 --max-time 3 "${base_url}" &>/dev/null; then
  echo "SKIP: OHC server is not reachable at ${base_url}."
  echo "To run E2E tests:"
  echo "  1. Start the OHC stack: cd deploy && docker compose up -d"
  echo "  2. Wait for services to be healthy"
  echo "  3. Run: bazel test //srcs/tests/e2e:playwright_tests"
  exit 0
fi

# The Playwright CLI is provided via the Bazel-managed node_modules symlink store.
# aspect_rules_js links //:node_modules/@playwright/test at ${root}/node_modules/
# which is a symlink into the package store; Node follows it to resolve all
# transitive deps (playwright, playwright-core) automatically.
PLAYWRIGHT_BIN="${root}/node_modules/@playwright/test/cli.js"
if [[ ! -f "$PLAYWRIGHT_BIN" ]]; then
  echo "ERROR: Playwright not found at ${PLAYWRIGHT_BIN}" >&2
  exit 1
fi

NODE_BIN="$(command -v node)" || {
  echo "ERROR: Node.js not found" >&2
  exit 1
}

export HOME="${HOME:-${TEST_TMPDIR}/home}"
export PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-/tmp/ohc-playwright-browsers}"

mkdir -p "$HOME"
mkdir -p "$PLAYWRIGHT_BROWSERS_PATH"

echo "Installing Playwright browsers if needed..."
# playwright install is idempotent: it exits quickly when browsers already exist.
# Use a glob to detect any versioned chromium-<rev> directory.
if ! ls "${PLAYWRIGHT_BROWSERS_PATH}"/chromium-* &>/dev/null 2>&1; then
  "$NODE_BIN" "$PLAYWRIGHT_BIN" install chromium --with-deps >/dev/null 2>&1 || true
fi

echo "Running Playwright test: $spec_path"
"$NODE_BIN" "$PLAYWRIGHT_BIN" test --config playwright.config.ts "$spec_path"
