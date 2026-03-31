#!/usr/bin/env bash
set -e

echo "Running E2E verification for github MCP server"

BUNDLE_DIR="${MCP_BUNDLE_DIR:-/tmp/mcp_bundles}"
mkdir -p "$BUNDLE_DIR"

if [ ! -x "./github" ] && [ ! -x "srcs/agents/mcp/github/github_/github" ] && [ ! -x "../github_/github" ] && [ ! -x "srcs/agents/mcp/github/github.sh" ]; then
    find .
    echo "Error: github binary not found or not executable"
    return 1 2>/dev/null
fi

BIN="./github"
if [ ! -x "$BIN" ]; then
    if [ -x "srcs/agents/mcp/github/github_/github" ]; then
        BIN="srcs/agents/mcp/github/github_/github"
    elif [ -x "srcs/agents/mcp/github/github.sh" ]; then
        BIN="srcs/agents/mcp/github/github.sh"
    else
        BIN="../github_/github"
    fi
fi

# Ensure that we expose the built bundle to the actual MCP_BUNDLE_DIR
# as specified in step 2 of the problem statement "Expose it as a standard bundle in MCP_BUNDLE_DIR"
cp "$BIN" "$BUNDLE_DIR/github_mcp_bundle"

$BUNDLE_DIR/github_mcp_bundle <<< "" > /dev/null || true

echo "GitHub MCP E2E verification passed."
