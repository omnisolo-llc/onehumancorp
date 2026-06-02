#!/usr/bin/env bash
set -e

# Change into test directory if needed or setup paths
dir="${TEST_SRCDIR:-}/$(basename $TEST_WORKSPACE)/src/ui/next"
if [[ -z "${TEST_SRCDIR:-}" ]]; then
  dir="${BUILD_WORKSPACE_DIRECTORY}/src/ui/next"
fi

cd "$dir"

# Provide fallback files just in case sandbox misses them
if [[ ! -f "vitest.setup.ts" ]]; then
  echo "copying missing vitest.setup.ts..."
  cp ${BUILD_WORKSPACE_DIRECTORY}/src/ui/next/vitest.setup.ts .
fi
if [[ ! -f "vitest.config.ts" ]]; then
  echo "copying missing vitest.config.ts..."
  cp ${BUILD_WORKSPACE_DIRECTORY}/src/ui/next/vitest.config.ts .
fi

if [ -f "node_modules/.bin/vitest" ]; then
    NODE_OPTIONS="--experimental-vm-modules" node_modules/.bin/vitest run --passWithNoTests
else
    # if node_modules isn't linked into the sandbox properly, fallback to BUILD_WORKSPACE_DIRECTORY
    cd "${BUILD_WORKSPACE_DIRECTORY:-/app}/src/ui/next"
    npm i vitest jsdom @vitejs/plugin-react @testing-library/jest-dom @testing-library/react @testing-library/dom --no-save || true
    npx vitest run --passWithNoTests
fi
