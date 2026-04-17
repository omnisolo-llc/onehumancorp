#!/usr/bin/env bash
set -euo pipefail

# Test to verify that the standalone script does not indiscriminately
# wipe Linear state files.

test_dir=$(mktemp -d)
trap "rm -rf \"${test_dir}\"" EXIT

export HOME="${test_dir}"
export STATE_DIR="${test_dir}/.openclaw"
export OHC_STANDALONE="true"
export OHC_LISTEN_ADDR="18789"

mkdir -p "${STATE_DIR}"

# Create some test files
touch "${STATE_DIR}/ohc.pid"
touch "${STATE_DIR}/ohc.log"
touch "${STATE_DIR}/linear.db"
touch "${STATE_DIR}/linear_old.tmp"
touch "${STATE_DIR}/linear_new.tmp"

# Make the old tmp file 2 hours old
touch -d "2 hours ago" "${STATE_DIR}/linear_old.tmp"

echo "9999999" > "${STATE_DIR}/ohc.pid"

# Run the script by mocking the server binary location
# We copy the script and replace the SERVER_BIN check so it can pass
cat srcs/server/standalone_ohc.sh | sed 's/SERVER_BIN="\(.*\)"/SERVER_BIN="\/bin\/true"/g' > "${test_dir}/standalone_ohc_mock.sh"
chmod +x "${test_dir}/standalone_ohc_mock.sh"

"${test_dir}/standalone_ohc_mock.sh" stop || true

FAIL=0

if [ ! -f "${STATE_DIR}/linear.db" ]; then
    echo "FAILED: linear.db was deleted!"
    FAIL=1
fi

if [ ! -f "${STATE_DIR}/linear_new.tmp" ]; then
    echo "FAILED: recent tmp file was deleted!"
    FAIL=1
fi

if [ -f "${STATE_DIR}/linear_old.tmp" ]; then
    echo "FAILED: old tmp file was not deleted!"
    FAIL=1
fi

if [ $FAIL -ne 0 ]; then
    false
else
    echo "All tests passed!"
fi
