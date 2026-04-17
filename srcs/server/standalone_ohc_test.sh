#!/usr/bin/env bash

set -euo pipefail

export HOME=$(mktemp -d)
STATE_DIR="${HOME}/.openclaw"
mkdir -p "${STATE_DIR}"

touch -t 200001010000 "${STATE_DIR}/old_file.tmp"
touch -t 200001010000 "${STATE_DIR}/old_Linear_state.tmp"
touch "${STATE_DIR}/new_file.tmp"
touch "${STATE_DIR}/new_Linear_state.tmp"

export OHC_STANDALONE="true"

# the data dependency ensures standalone_ohc.sh is here
SCRIPT_DIR=$(dirname "$0")

# mock server bin so the script doesn't fail early
touch "${HOME}/ohc-server"
chmod +x "${HOME}/ohc-server"

# In the bazel test runfiles, we might need to find standalone_ohc.sh
if [[ -f "${SCRIPT_DIR}/standalone_ohc.sh" ]]; then
  WRAPPER_SCRIPT="${SCRIPT_DIR}/standalone_ohc.sh"
elif [[ -f "srcs/server/standalone_ohc.sh" ]]; then
  WRAPPER_SCRIPT="srcs/server/standalone_ohc.sh"
else
  # fallback for test wrapper
  WRAPPER_SCRIPT=$(find . -name standalone_ohc.sh | head -n 1)
fi

cp "${WRAPPER_SCRIPT}" "${HOME}/standalone_ohc.sh"
chmod +x "${HOME}/standalone_ohc.sh"

# Run stop which will invoke cleanup_tmp_files
"${HOME}/standalone_ohc.sh" stop

# Verify old_file.tmp is deleted
if [[ -f "${STATE_DIR}/old_file.tmp" ]]; then
  echo "FAIL: old_file.tmp was not deleted"
  exit 1
fi

# Verify old_Linear_state.tmp is preserved
if [[ ! -f "${STATE_DIR}/old_Linear_state.tmp" ]]; then
  echo "FAIL: old_Linear_state.tmp was deleted"
  exit 1
fi

# Verify new_file.tmp is preserved
if [[ ! -f "${STATE_DIR}/new_file.tmp" ]]; then
  echo "FAIL: new_file.tmp was deleted"
  exit 1
fi

# Verify new_Linear_state.tmp is preserved
if [[ ! -f "${STATE_DIR}/new_Linear_state.tmp" ]]; then
  echo "FAIL: new_Linear_state.tmp was deleted"
  exit 1
fi

echo "PASS"
exit 0
