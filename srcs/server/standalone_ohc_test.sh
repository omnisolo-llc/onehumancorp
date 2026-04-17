#!/usr/bin/env bash
set -euo pipefail

export HOME="$(mktemp -d)"
# trap for cleanup
trap 'rm -rf "${HOME}"' EXIT

export STATE_DIR="${HOME}/.openclaw"
mkdir -p "${STATE_DIR}"

touch -t 200001010000 "${STATE_DIR}/old_normal.tmp"
touch -t 200001010000 "${STATE_DIR}/old_Linear_state.tmp"
touch "${STATE_DIR}/recent_normal.tmp"

export OHC_STANDALONE="true"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

(
  export SERVER_BIN="${HOME}/ohc"
  cat << 'MOCK' > "${HOME}/ohc"
#!/bin/bash
echo "mock ohc"
MOCK
  chmod +x "${HOME}/ohc"

  cp "${SCRIPT_DIR}/standalone_ohc.sh" "${HOME}/standalone_ohc.sh"
  sed -i "s/SERVER_BIN=\"\$(find_server_bin \"\${SCRIPT_DIR}\" || true)\"/SERVER_BIN=\"${HOME//\//\\/}\/ohc\"/" "${HOME}/standalone_ohc.sh"

  "${HOME}/standalone_ohc.sh" stop
)

failed=0

if [ -f "${STATE_DIR}/old_normal.tmp" ]; then
  echo "old_normal.tmp was not deleted!"
  failed=1
fi

if [ ! -f "${STATE_DIR}/old_Linear_state.tmp" ]; then
  echo "old_Linear_state.tmp was deleted!"
  failed=1
fi

if [ ! -f "${STATE_DIR}/recent_normal.tmp" ]; then
  echo "recent_normal.tmp was deleted!"
  failed=1
fi

if [ "$failed" -eq 1 ]; then
  exit 1
fi

echo "All tests passed!"
