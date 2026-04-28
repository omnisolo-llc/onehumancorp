#!/bin/bash
set -e

# Playwright Test Runner for Bazel
# Usage: ./run_playwright.sh <test_file>

TEST_FILE=$1
PORT=3000

# The test file handles building WASM and setting up staging.
# This script just runs Playwright against the server started by the test.

echo "Running Playwright test: $TEST_FILE"
export HOME=/tmp

# Run Playwright
./node_modules/.bin/playwright test "$TEST_FILE" --config=deploy/tests/playwright.config.ts