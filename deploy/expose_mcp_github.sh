#!/bin/bash
set -e

# Build the binary
bazelisk build //srcs/agents/mcp/github:github

# Expose it in the MCP bundle dir
mkdir -p "${MCP_BUNDLE_DIR:-/tmp/mcp_bundles}/github"
cp bazel-bin/srcs/agents/mcp/github/github_/github "${MCP_BUNDLE_DIR:-/tmp/mcp_bundles}/github/github"

echo "Exposed github MCP bundle in ${MCP_BUNDLE_DIR:-/tmp/mcp_bundles}/github"
