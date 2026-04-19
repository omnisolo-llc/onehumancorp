#!/usr/bin/env bash
set -euo pipefail

spec_path="${1:-}"

if [[ -z "$spec_path" ]]; then
  echo "Usage: $0 <spec_path>" >&2
  exit 1
fi

repo_name="${TEST_WORKSPACE:-mono}"
root="${TEST_SRCDIR:-$PWD}/${repo_name}"
e2e_dir="${root}/srcs/tests/e2e"

if [[ ! -d "$e2e_dir" ]]; then
  echo "ERROR: e2e directory not found at ${e2e_dir}" >&2
  exit 1
fi

cd "$e2e_dir"

if [[ ! -f "package.json" ]]; then
  echo "SKIP: npm dependencies not installed. E2E tests require 'npm install' to be run first."
  echo "To run E2E tests:"
  echo "  1. cd srcs/tests/e2e && npm install"
  echo "  2. Start the OHC stack: cd deploy && docker compose up -d"
  echo "  3. Run: bazel test //srcs/tests/e2e:playwright_tests"
  exit 0
fi

if ! command -v docker &>/dev/null || ! docker info &>/dev/null; then
  echo "SKIP: Docker is not available. E2E tests require Docker to be running."
  echo "To run E2E tests manually:"
  echo "  1. Start the OHC stack: cd deploy && docker compose up -d"
  echo "  2. Run: cd srcs/tests/e2e && npm test"
  exit 0
fi

PLAYWRIGHT_BIN="./node_modules/@playwright/test/cli.js"
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
if [[ ! -d "$PLAYWRIGHT_BROWSERS_PATH/chromium" ]]; then
  "$NODE_BIN" "$PLAYWRIGHT_BIN" install chromium --with-deps >/dev/null 2>&1 || true
fi

escaped_spec="$(printf '%s' "$spec_path" | sed -e 's/[.[\*^$()+?{}|]/\\&/g')"
export PLAYWRIGHT_TEST_MATCH="(^|.*/)${escaped_spec}$"

echo "Running Playwright test: $spec_path"
"$NODE_BIN" "$PLAYWRIGHT_BIN" test --config playwright.config.ts
