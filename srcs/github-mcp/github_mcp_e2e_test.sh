#!/bin/bash
set -e

echo "Starting github-mcp E2E verification test..."

# Ensure the binary path is accessible
BUNDLE_PATH="${MCP_BUNDLE_DIR}"
if [ ! -f "$BUNDLE_PATH" ]; then
  echo "Error: Bundle not found at $BUNDLE_PATH"
  return 1 2>/dev/null || _exit_code=1
  [ -z "${_exit_code}" ] || { exit $_exit_code; }
fi

echo "Bundle found successfully."

echo "Starting github-mcp server..."
GITHUB_PERSONAL_ACCESS_TOKEN="dummy-token" "$BUNDLE_PATH" > output.log 2>&1 &
PID=$!

sleep 2

kill $PID || true

if grep -q "Server running on stdio" output.log; then
    echo "Server successfully started and initialized."
    return 0 2>/dev/null || _exit_code=0
    [ -z "${_exit_code}" ] || { exit $_exit_code; }
else
    echo "Server failed to start or output not as expected."
    cat output.log
    return 1 2>/dev/null || _exit_code=1
    [ -z "${_exit_code}" ] || { exit $_exit_code; }
fi
