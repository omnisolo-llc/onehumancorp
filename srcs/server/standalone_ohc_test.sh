#!/usr/bin/env bash
set -euo pipefail

# Find the script to test.
SCRIPT_PATH="srcs/server/standalone_ohc.sh"
if [[ ! -f "${SCRIPT_PATH}" ]]; then
  # Fallback for bazel runfiles
  SCRIPT_PATH="${TEST_SRCDIR}/${TEST_WORKSPACE}/srcs/server/standalone_ohc.sh"
fi

if [[ ! -f "${SCRIPT_PATH}" ]]; then
  echo "Could not find standalone_ohc.sh at ${SCRIPT_PATH}"
  exit 1
fi

export STATE_DIR=$(mktemp -d)
export OHC_STANDALONE="true"

# Mock files
touch "${STATE_DIR}/importantLinearState.txt"
touch "${STATE_DIR}/oldLinear.txt"
touch -d "2 hours ago" "${STATE_DIR}/oldLinear.txt"
touch "${STATE_DIR}/recent.tmp"
touch "${STATE_DIR}/old.tmp"
# Make old.tmp older than 60 mins
touch -d "2 hours ago" "${STATE_DIR}/old.tmp"

# Extract the cleanup function using sed
sed -n '/^cleanup_tmp_files() {/,/^}/p' "${SCRIPT_PATH}" > "${STATE_DIR}/test_env.sh"
source "${STATE_DIR}/test_env.sh"

cleanup_tmp_files

# Verification
if [[ ! -f "${STATE_DIR}/importantLinearState.txt" ]]; then
  echo "FAIL: Linear state file was deleted"
  exit 1
fi

if [[ ! -f "${STATE_DIR}/recent.tmp" ]]; then
  echo "FAIL: Recent tmp file was deleted"
  exit 1
fi

if [[ -f "${STATE_DIR}/old.tmp" ]]; then
  echo "FAIL: Old tmp file was NOT deleted"
  exit 1
fi

if [[ -f "${STATE_DIR}/oldLinear.txt" ]]; then
  echo "FAIL: oldLinear txt file was NOT deleted"
  exit 1
fi

echo "PASS: Cleanup logic behaves correctly"
rm -rf "${STATE_DIR}"