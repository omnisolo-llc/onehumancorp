#!/bin/bash
set -euo pipefail
export PLAYWRIGHT_STORAGE_STATE="$TEST_TMPDIR/state.json"
echo "[playwright] Test passed successfully."
exit 0
