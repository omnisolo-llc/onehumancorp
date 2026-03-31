# GitHub MCP Integration

## Objective
Package the standard `@modelcontextprotocol/server-github` via Bazel and integrate it into the OHC Swarm OS so that AI agents can seamlessly interact with GitHub repositories, pull requests, and issues.

## Architecture
1. Add the Node dependency using pnpm.
2. Expose the server using `aspect_rules_js` as `js_binary`.
3. The resulting binary can then be launched as a bundle via stdio in the `MCP_BUNDLE_DIR`.

## Steps
1. Package it in Bazel (rules_nodejs)
2. E2E Verification
