#!/bin/bash
# Vitest test runner - hermetic via Bazel data dependencies
set -e

# Script location in Bazel runfiles: _main/src/cli/test_runner.sh
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Set NODE_PATH to help module resolution in deep sandbox paths
export NODE_PATH="$SCRIPT_DIR/node_modules"

# Run vitest without coverage (rolldown has issues in sandbox)
exec ./node_modules/.bin/vitest run
