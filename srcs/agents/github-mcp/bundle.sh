#!/usr/bin/env bash
# Exposes the github-mcp as a bundle in MCP_BUNDLE_DIR

BUNDLE_DIR="${MCP_BUNDLE_DIR:-/tmp/mcp_bundles}"
mkdir -p "$BUNDLE_DIR/github-mcp"
cp -aL "$1" "$BUNDLE_DIR/github-mcp/run"
chmod +x "$BUNDLE_DIR/github-mcp/run"

if [ -d "${1}.runfiles" ]; then
    cp -aL "${1}.runfiles" "$BUNDLE_DIR/github-mcp/run.runfiles"
fi
