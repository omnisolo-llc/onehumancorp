# Test Plan: GitHub MCP Integration

1. Ensure the Bazel node target `//srcs/mcp/github` builds correctly without workspace errors.
2. Run `bazelisk test //...` to guarantee the Go orchestration tests pass with the updated tool ID handling.
3. Validate that MCP payload parsing in `invokeMCPTool` continues to operate successfully and robustly for mock integration testing.
