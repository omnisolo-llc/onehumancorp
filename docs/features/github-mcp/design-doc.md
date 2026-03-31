# Design Doc: GitHub MCP Integration

## What
Integrates the `@modelcontextprotocol/server-github` module via an `aspect_rules_js` Bazel target and connects it to the Go Orchestration Backend using `stdio`.

## Why
Enables Autonomous Agents to directly interface with GitHub to read issues, pull requests, and commit changes using the standard Model Context Protocol.

## Architecture
- **Node.js**: Wrapped inside `srcs/mcp/github/index.js` and packaged using `aspect_rules_js`.
- **Backend Wiring**: We removed the "hacky alias" to `git-mcp` and introduced proper stdio execution for `github-mcp` if the binary is accessible via `MCP_BUNDLE_DIR`, otherwise it falls back to the native implementation for mock/dev workflows.
- **Observability**: Updated `status` and `missions` following SIPDB conventions.
