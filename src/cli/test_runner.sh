#!/bin/bash
# Vitest test runner - hermetic via Bazel data dependencies
set -e

# Script location in Bazel runfiles: _main/src/cli/test_runner.sh
# The data dependency //src/cli:node_modules is symlinked to the source tree
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Run vitest from Bazel-managed node_modules (hermetic, no system deps needed)
exec ./node_modules/.bin/vitest run
