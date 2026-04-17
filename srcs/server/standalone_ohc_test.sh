#!/usr/bin/env bash
set -euo pipefail

# Setup fake home directory for the test
TEST_HOME="$(mktemp -d)"
export HOME="${TEST_HOME}"
STATE_DIR="${HOME}/.openclaw"
mkdir -p "${STATE_DIR}"

# Create a mock server binary
MOCK_SERVER="${TEST_HOME}/ohc"
touch "${MOCK_SERVER}"
chmod +x "${MOCK_SERVER}"
export PATH="${TEST_HOME}:${PATH}"

# Create an old temp file that should be deleted
touch -d "2 hours ago" "${STATE_DIR}/old.tmp"

# Create a new temp file that shouldn't be deleted yet
touch "${STATE_DIR}/new.tmp"

# Create a Linear file that must be preserved
touch "${STATE_DIR}/myLinear_file.txt"

SCRIPT_PATH="$(pwd)/srcs/server/standalone_ohc.sh"
# If running under Bazel, we need to adjust SCRIPT_PATH
if [ -n "${TEST_WORKSPACE:-}" ]; then
    # It might be in the runfiles
    if [ -f "srcs/server/standalone_ohc.sh" ]; then
        SCRIPT_PATH="$(pwd)/srcs/server/standalone_ohc.sh"
    fi
fi

# We don't want to modify the source tree to mock the find_server_bin logic.
# So we copy the script to TEST_HOME and mock the server next to it.
MOCK_SCRIPT_DIR="${TEST_HOME}/mock_bin"
mkdir -p "${MOCK_SCRIPT_DIR}"
cp "${SCRIPT_PATH}" "${MOCK_SCRIPT_DIR}/standalone_ohc.sh"
touch "${MOCK_SCRIPT_DIR}/ohc"
chmod +x "${MOCK_SCRIPT_DIR}/ohc"

# Run stop which will invoke cleanup_tmp_files
# We bypass the start check by mocking PID_FILE to some fake process
PID_FILE="${STATE_DIR}/ohc.pid"
# Fake process
echo "99999" > "${PID_FILE}"

# We execute the standalone_ohc.sh copy in a subshell, redirecting stop output
bash "${MOCK_SCRIPT_DIR}/standalone_ohc.sh" stop > /dev/null 2>&1 || true

# Assert old tmp file is deleted
if [ -f "${STATE_DIR}/old.tmp" ]; then
    echo "FAIL: old.tmp was not deleted"
    exit 1
fi

# Assert new tmp file is NOT deleted
if [ ! -f "${STATE_DIR}/new.tmp" ]; then
    echo "FAIL: new.tmp was deleted"
    exit 1
fi

# Assert Linear file is NOT deleted
if [ ! -f "${STATE_DIR}/myLinear_file.txt" ]; then
    echo "FAIL: myLinear_file.txt was deleted"
    exit 1
fi

echo "PASS: standalone_ohc_test.sh"
rm -rf "${TEST_HOME}"
