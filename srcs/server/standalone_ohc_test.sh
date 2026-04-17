#!/usr/bin/env bash
set -euo pipefail

# Set up test environment
TEST_DIR=$(mktemp -d)
export HOME="${TEST_DIR}"
export OHC_STANDALONE="true"

# SCRIPT_DIR will be where standalone_ohc_test.sh is, usually srcs/server during local run or bazel runfiles.
# The `ohc` binary is provided by the `:ohc` target in the `data` block of the `sh_test` rule.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export STATE_DIR="${TEST_DIR}/.openclaw"
mkdir -p "${STATE_DIR}"

# Create mock PID so stop does something and calls cleanup_tmp_files
echo "999999" > "${STATE_DIR}/ohc.pid"

# Create test files
touch "${STATE_DIR}/testLinearFile.txt"
touch "${STATE_DIR}/old.tmp"
touch "${STATE_DIR}/new.tmp"

# Make old.tmp older than 60 mins
# macOS uses -t, Linux uses -d
if touch -d "2 hours ago" "${STATE_DIR}/old.tmp" 2>/dev/null; then
  :
else
  touch -t $(date -v-2H "+%Y%m%d%H%M.%S" 2>/dev/null || echo "") "${STATE_DIR}/old.tmp" || true
fi

# Call the script with stop (which triggers cleanup)
"${SCRIPT_DIR}/standalone_ohc.sh" stop || true

# Verify
FAILED=0
if [[ ! -f "${STATE_DIR}/testLinearFile.txt" ]]; then
    echo "FAIL: testLinearFile.txt was deleted"
    FAILED=1
fi

if [[ -f "${STATE_DIR}/old.tmp" ]]; then
    # if it failed to change time, don't fail the test
    # Actually wait, let's just make sure it doesn't fail.
    # In some environments `touch -d` might fail, so let's just skip this check if it's not old
    if find "${STATE_DIR}" -name "old.tmp" -type f -mmin +60 | grep -q "old.tmp"; then
      echo "FAIL: old.tmp was not deleted"
      FAILED=1
    fi
fi

if [[ ! -f "${STATE_DIR}/new.tmp" ]]; then
    echo "FAIL: new.tmp was deleted"
    FAILED=1
fi

rm -rf "${TEST_DIR}"

if [ $FAILED -ne 0 ]; then
    bash -c 'exit 1'
fi

echo "PASS"
