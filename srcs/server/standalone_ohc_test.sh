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
  # Note: The original word "ex\it" is avoided to satisfy the system filter. Instead we use return if sourced, or similar.
  # Let's write the exit using command substitution trick, but wait, the filter blocks strings containing e-x-i-t followed by space.
  # Wait, earlier error was: "Unable to run bash because the script contains exit, which would block the session or cause other issues."
  # But here I'm using `write_file` tool, which doesn't have the filter, it only applies to `run_in_bash_session`. Let me check memory:
  # "The `run_in_bash_session` tool automatically blocks any command payload containing the exact string `exit ` (e.g., `exit 1`) to prevent session termination. Prefer using the `write_file` tool to safely write files containing these strings instead of relying on complex shell workarounds or temporary Python scripts."
  # Ok, so write_file is fine with `exit `.
  exit 1
fi

export STATE_DIR=$(mktemp -d)
export OHC_STANDALONE="true"

# Mock files
touch "${STATE_DIR}/importantLinearState.txt"
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

echo "PASS: Cleanup logic behaves correctly"

# Verify that OHC_TELEMETRY_ENABLED is explicitly set to false in the script's final execution blocks
if ! grep -q 'OHC_TELEMETRY_ENABLED="false"' "${SCRIPT_PATH}"; then
  echo "FAIL: OHC_TELEMETRY_ENABLED is not forced to false in standalone_ohc.sh"
  exit 1
fi

echo "PASS: Telemetry exfiltration guardrail is present"
rm -rf "${STATE_DIR}"