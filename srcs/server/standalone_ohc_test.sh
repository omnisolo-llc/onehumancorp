#!/usr/bin/env bash
set -euo pipefail

SCRIPT_SRC="srcs/server/standalone_ohc.sh"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

cp "$SCRIPT_SRC" "$TMP_DIR/"
touch "$TMP_DIR/ohc-server"
chmod +x "$TMP_DIR/ohc-server" "$TMP_DIR/standalone_ohc.sh"

export HOME="$TMP_DIR/home"
mkdir -p "$HOME/.openclaw"
STATE_DIR="$HOME/.openclaw"

# Create a file and artificially backdate it to be older than 60 mins.
# Using touch -t to be compatible with both GNU and BSD touch (macOS)
touch -t "200001010000" "${STATE_DIR}/old.tmp"
touch "${STATE_DIR}/new.tmp"

export OHC_STANDALONE="true"

# the stop command invokes cleanup_tmp_files
"$TMP_DIR/standalone_ohc.sh" stop >/dev/null

if [ -f "${STATE_DIR}/old.tmp" ]; then
    echo "old.tmp was not deleted!"
    exit 1
fi

if [ ! -f "${STATE_DIR}/new.tmp" ]; then
    echo "new.tmp was deleted!"
    exit 1
fi

echo "All tests passed."
exit 0