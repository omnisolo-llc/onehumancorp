#!/usr/bin/env bash
set -euo pipefail

# Create a temporary state directory
TEMP_STATE_DIR="$(mktemp -d)"

# Ensure cleanup on exit
trap 'rm -rf "${TEMP_STATE_DIR}"' EXIT

# Setup mock environment
export HOME="${TEMP_STATE_DIR}"
export OHC_STANDALONE="true"

# Mock the ohc server binary so find_server_bin can find it
mkdir -p "${TEMP_STATE_DIR}/mock_bin"
touch "${TEMP_STATE_DIR}/mock_bin/ohc-server"
chmod +x "${TEMP_STATE_DIR}/mock_bin/ohc-server"
export PATH="${TEMP_STATE_DIR}/mock_bin:$PATH"

cp "${BASH_SOURCE[0]%/*}/standalone_ohc.sh" "${TEMP_STATE_DIR}/standalone_ohc.sh"
touch "${TEMP_STATE_DIR}/ohc"
chmod +x "${TEMP_STATE_DIR}/ohc"

# Create test files
mkdir -p "${TEMP_STATE_DIR}/.openclaw"
export STATE_DIR="${TEMP_STATE_DIR}/.openclaw"

# 1. Create a Linear file
touch "${STATE_DIR}/testLinearFile.txt"

# 2. Create a .tmp file that is old enough to be deleted (>60 min)
python3 -c "import os, time; path='${STATE_DIR}/old_temp.tmp'; open(path, 'a').close(); os.utime(path, (time.time() - 7200, time.time() - 7200))"

# 3. Create a .tmp file that is new (should not be deleted)
touch "${STATE_DIR}/new_temp.tmp"

# Execute standalone_ohc.sh stop to trigger stop_daemon -> cleanup_tmp_files
"${TEMP_STATE_DIR}/standalone_ohc.sh" stop

# Assertions
failed=0

if [[ ! -f "${STATE_DIR}/testLinearFile.txt" ]]; then
  echo "FAIL: testLinearFile.txt was deleted!"
  failed=1
fi

if [[ -f "${STATE_DIR}/old_temp.tmp" ]]; then
  echo "FAIL: old_temp.tmp was not deleted!"
  failed=1
fi

if [[ ! -f "${STATE_DIR}/new_temp.tmp" ]]; then
  echo "FAIL: new_temp.tmp was deleted!"
  failed=1
fi

if [ $failed -eq 1 ]; then
  echo "Test failed!"
  false
fi

echo "PASS: standalone_ohc.sh cleanup logic is correct."
