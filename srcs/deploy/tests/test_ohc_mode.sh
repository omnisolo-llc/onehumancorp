#!/bin/bash
set -e
echo "Testing ohc-mode.sh standalone mode directory creation..."
export XDG_STATE_HOME="$(mktemp -d)"
export OHC_RUNTIME_DIR="${XDG_STATE_HOME}/ohc/runtime"
export OHC_MEMORY_DIR="${OHC_RUNTIME_DIR}/memory"
export OHC_STATUS_DIR="${OHC_RUNTIME_DIR}/status"
rm -rf "${OHC_MEMORY_DIR}/auto/" "${OHC_MEMORY_DIR}/team/"
bash deploy/scripts/ohc-mode.sh standalone
if [ ! -d "${OHC_MEMORY_DIR}/auto/" ]; then
    echo "ERROR: ${OHC_MEMORY_DIR}/auto/ was not created."
    exit 1
fi
if [ ! -d "${OHC_MEMORY_DIR}/team/" ]; then
    echo "ERROR: ${OHC_MEMORY_DIR}/team/ was not created."
    exit 1
fi
echo "SUCCESS: standalone mode creates memory directories."
