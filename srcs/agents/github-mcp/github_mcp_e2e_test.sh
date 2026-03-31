#!/usr/bin/env bash
set -e

# Setup mock bundle dir
export MCP_BUNDLE_DIR="${TEST_TMPDIR}/mcp_bundles"
mkdir -p "${MCP_BUNDLE_DIR}"

# Run the bundle script to expose it
BIN_PATH=$1
if [ -z "$BIN_PATH" ]; then
    BIN_PATH="srcs/agents/github-mcp/github-mcp-bin"
fi

./srcs/agents/github-mcp/github-mcp "$BIN_PATH"

# Verify the bundle exists
if [ ! -f "${MCP_BUNDLE_DIR}/github-mcp/run" ]; then
  echo "Error: github-mcp bundle was not exposed properly."
  exit 1
fi

echo "Success: Bundle script exists and is executable."

# Execute the MCP server with a mock initialization payload to verify it starts and responds
export GITHUB_PERSONAL_ACCESS_TOKEN="mock_token"

# Create a mock initialize request matching MCP specification
# The MCP server reads line-by-line JSON-RPC
cat << 'EOF' > init_req.json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}
EOF

# Run the MCP server, feeding it the initialize request
OUTPUT=$(cat init_req.json | "${MCP_BUNDLE_DIR}/github-mcp/run" || true)

echo "Output from MCP Server:"
echo "$OUTPUT"

# Verify the output contains a successful JSON-RPC response with capabilities
if [[ "$OUTPUT" == *"\"jsonrpc\":\"2.0\""* ]] && [[ "$OUTPUT" == *"\"id\":1"* ]] && [[ "$OUTPUT" == *"capabilities"* ]]; then
    echo "Success: MCP server initialized correctly."
else
    echo "Error: MCP server failed to initialize properly."
    exit 1
fi
