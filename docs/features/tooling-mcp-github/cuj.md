# CUJ: AI Agent using GitHub MCP

1. The AI Agent discovers the GitHub MCP tool via `mcp://tools/github`.
2. The agent uses the `github` MCP tool to fetch repo contents or pull request info.
3. The underlying process invokes the Bazel-built GitHub MCP binary over stdio.
