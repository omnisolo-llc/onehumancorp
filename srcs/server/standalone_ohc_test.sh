#!/usr/bin/env bash
set -euo pipefail

SCRIPT_PATH="srcs/server/standalone_ohc.sh"
if [[ ! -f "$SCRIPT_PATH" ]]; then
  SCRIPT_PATH=$(find . -name standalone_ohc.sh | head -n 1)
fi

export HOME=$(mktemp -d)
trap 'rm -rf "${HOME}"' EXIT

export OHC_STANDALONE="true"
export OHC_LISTEN_ADDR="12345"
STATE_DIR="${HOME}/.openclaw"
mkdir -p "${STATE_DIR}"

touch "${STATE_DIR}/test.tmp"
touch "${STATE_DIR}/myLinearFile.txt"

if [[ "$OSTYPE" == "darwin"* ]]; then
  touch -t $(date -v-2H +%Y%m%d%H%M) "${STATE_DIR}/test.tmp"
else
  touch -d "2 hours ago" "${STATE_DIR}/test.tmp"
fi

echo "Running stop..."
bash "$SCRIPT_PATH" stop

if [[ -f "${STATE_DIR}/test.tmp" ]]; then
  echo "test.tmp was NOT deleted! Test failed."
  exit 1
fi

if [[ ! -f "${STATE_DIR}/myLinearFile.txt" ]]; then
  echo "myLinearFile.txt WAS deleted! Test failed."
  exit 1
fi

echo "Test passed."
