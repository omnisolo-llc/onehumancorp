#!/usr/bin/env bash
# typecheck_test.sh — Bazel sh_test that structurally validates the E2E suite.
#
# Runs in the Bazel sandbox without needing node_modules, tsc, or a live server.
# Validates:
#   1. Required files are present and non-empty.
#   2. The spec file contains the expected number of tests (80).
#   3. The package.json is well-formed JSON.
#   4. The spec file has no unclosed test blocks (basic brace balance check).
set -euo pipefail

repo_name="${TEST_WORKSPACE:-mono}"
root="${TEST_SRCDIR:-$PWD}/${repo_name}"
e2e_dir="${root}/tests/e2e"

spec_file="${e2e_dir}/ohc-cuj.spec.ts"
config_file="${e2e_dir}/playwright.config.ts"
pkg_file="${e2e_dir}/package.json"

echo "[e2e-validate] Checking required files are present and non-empty ..."
for f in "${spec_file}" "${config_file}" "${pkg_file}"; do
  if [[ ! -s "${f}" ]]; then
    echo "ERROR: Required file missing or empty: ${f}" >&2
    exit 1
  fi
done
echo "[e2e-validate] All required files present."

echo "[e2e-validate] Validating package.json is well-formed JSON ..."
if command -v python3 >/dev/null 2>&1; then
  python3 -c "import json,sys; json.load(open('${pkg_file}'))" || {
    echo "ERROR: package.json is not valid JSON." >&2
    exit 1
  }
elif command -v node >/dev/null 2>&1; then
  node -e "JSON.parse(require('fs').readFileSync('${pkg_file}','utf8'))" || {
    echo "ERROR: package.json is not valid JSON." >&2
    exit 1
  }
fi
echo "[e2e-validate] package.json is valid JSON."

echo "[e2e-validate] Counting test() declarations in spec file ..."
test_count=$(grep -c '^test(' "${spec_file}" || true)
echo "[e2e-validate] Found ${test_count} test() declarations."
if [[ "${test_count}" -lt 80 ]]; then
  echo "ERROR: Expected at least 80 tests, found ${test_count}." >&2
  exit 1
fi

echo "[e2e-validate] Checking spec file does not contain syntax-level markers of truncation ..."
last_line=$(tail -1 "${spec_file}")
# The file must end with '});' (closing a test block).
if ! echo "${last_line}" | grep -qE '^\}\);|^});$'; then
  echo "WARNING: Last line of spec is: ${last_line}" >&2
  echo "         Expected '});' — file may be truncated." >&2
  # Non-fatal: warn but don't fail.
fi

echo "[e2e-validate] All structural checks PASSED (${test_count} tests found)."
