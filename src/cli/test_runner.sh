#!/bin/bash
# Vitest test runner - hermetic via Bazel data dependencies
set -e

# Script location in Bazel runfiles: _main/src/cli/test_runner.sh
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Follow node_modules symlink to get to real external path
NODE_MODULES_REAL="$(readlink -f "$SCRIPT_DIR/node_modules" 2>/dev/null || echo "")"
if [ -n "$NODE_MODULES_REAL" ]; then
    cd "$(dirname "$NODE_MODULES_REAL")"
else
    cd "$SCRIPT_DIR"
fi

# Find node binary from runfiles or PATH
NODE_BIN=""
for candidate in \
    "$SCRIPT_DIR/../external/rules_nodejs++node+nodejs/bin/nodejs/node" \
    "$SCRIPT_DIR/../external/nodejs/bin/node" \
    "$SCRIPT_DIR/../../../external/nodejs/bin/node"; do
    if [ -x "$candidate" ]; then
        NODE_BIN="$candidate"
        break
    fi
done

# Fall back to PATH if no runfiles node found
if [ -z "$NODE_BIN" ]; then
    NODE_BIN="$(command -v node || echo "")"
fi

if [ -z "$NODE_BIN" ]; then
    echo "Error: node not found in PATH or runfiles" >&2
    exit 127
fi

VITEST_BIN="./node_modules/.bin/vitest"
exec "$NODE_BIN" "$VITEST_BIN" run