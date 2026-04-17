#!/usr/bin/env bash
set -euo pipefail

# --- begin runfiles.bash initialization v3 ---
f=bazel_tools/tools/bash/runfiles/runfiles.bash
source "${RUNFILES_DIR:-/dev/null}/$f" 2>/dev/null || \
  source "$(grep -sm1 "^$f " "${RUNFILES_MANIFEST_FILE:-/dev/null}" | cut -f2- -d' ')" 2>/dev/null || \
  source "$0.runfiles/$f" 2>/dev/null || \
  source "$(grep -sm1 "^$f " "$0.runfiles_manifest" | cut -f2- -d' ')" 2>/dev/null || \
  source "$(grep -sm1 "^$f " "$0.exe.runfiles_manifest" | cut -f2- -d' ')" 2>/dev/null || \
  { echo>&2 "ERROR: cannot find $f"; exit 1; }
# --- end runfiles.bash initialization v3 ---

SCRIPT_PATH="$(rlocation _main/srcs/server/standalone_ohc.sh)"
if [[ -z "${SCRIPT_PATH}" ]]; then
  echo "ERROR: could not locate standalone_ohc.sh" >&2
  exit 1
fi

TEMP_DIR="$(mktemp -d)"
export HOME="${TEMP_DIR}"
export OHC_STANDALONE="true"

SCRIPT_DIR="${TEMP_DIR}/srcs/server"
mkdir -p "${SCRIPT_DIR}"
cp "${SCRIPT_PATH}" "${SCRIPT_DIR}/standalone_ohc.sh"
chmod +x "${SCRIPT_DIR}/standalone_ohc.sh"

export SERVER_BIN="${SCRIPT_DIR}/ohc"
touch "${SERVER_BIN}"
chmod +x "${SERVER_BIN}"

STATE_DIR="${HOME}/.openclaw"
mkdir -p "${STATE_DIR}"

touch "${STATE_DIR}/test.tmp"
touch "${STATE_DIR}/should_keep.txt"
touch "${STATE_DIR}/testLinearFile.txt"

# Set modification time using python for cross-platform compatibility
python3 -c "import os, time; t=time.time()-7200; os.utime('${STATE_DIR}/test.tmp', (t, t))"

export OHC_STANDALONE="true"
"${SCRIPT_DIR}/standalone_ohc.sh" stop || true

FAIL=0

if [[ -f "${STATE_DIR}/test.tmp" ]]; then
    echo "test.tmp was not deleted"
    FAIL=1
fi

if [[ ! -f "${STATE_DIR}/testLinearFile.txt" ]]; then
    echo "testLinearFile.txt was incorrectly deleted. Linear state files should be preserved per requirements."
    FAIL=1
fi

if [[ ! -f "${STATE_DIR}/should_keep.txt" ]]; then
    echo "should_keep.txt was incorrectly deleted"
    FAIL=1
fi

if [ $FAIL -ne 0 ]; then
  exit 1
fi

echo "All tests passed"
