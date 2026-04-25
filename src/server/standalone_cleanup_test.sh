#!/usr/bin/env bash
set -euo pipefail

# Find the runfiles directory
if [[ -z "${RUNFILES_DIR:-}" ]]; then
  if [[ -n "${TEST_SRCDIR:-}" ]]; then
    export RUNFILES_DIR="${TEST_SRCDIR}"
  else
    echo "ERROR: RUNFILES_DIR and TEST_SRCDIR are not set." >&2
    exit 1
  fi
fi

# Locate the standalone_ohc.sh script within the runfiles
script_path="${RUNFILES_DIR}/_main/src/server/standalone_ohc.sh"
if [[ ! -f "${script_path}" ]]; then
    script_path="${RUNFILES_DIR}/${TEST_WORKSPACE:-mono}/src/server/standalone_ohc.sh"
    if [[ ! -f "${script_path}" ]]; then
        echo "ERROR: standalone_ohc.sh not found in runfiles" >&2
        exit 1
    fi
fi

source <(sed -n "/^cleanup_tmp_files() {/,/^}/p" "${script_path}")

TEST_DIR=$(mktemp -d)
export STATE_DIR="$TEST_DIR"
export OHC_STANDALONE="true"

touch "${STATE_DIR}/normal.tmp"
touch "${STATE_DIR}/Linear-state.tmp"
touch "${STATE_DIR}/some_linear_junk"

# Artificial backdate to simulate old files using touch -t
touch -t 200001010000 "${STATE_DIR}/normal.tmp"
touch -t 200001010000 "${STATE_DIR}/Linear-state.tmp"

cleanup_tmp_files

if [[ -f "${STATE_DIR}/normal.tmp" ]]; then
  echo "Error: normal.tmp was not deleted"
  exit 1
fi

if [[ -f "${STATE_DIR}/Linear-state.tmp" ]]; then
  echo "Error: Linear-state.tmp was not deleted"
  exit 1
fi

if [[ -f "${STATE_DIR}/some_linear_junk" ]]; then
  echo "Error: some_linear_junk was not deleted"
  exit 1
fi

rm -rf "${TEST_DIR}"
echo "PASS: cleanup logic works"
