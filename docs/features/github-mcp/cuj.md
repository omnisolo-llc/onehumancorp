# CUJ: GitHub MCP Integration

1. A human provides a task requiring a PR review.
2. The orchestrator delegates the task to a Worker Agent.
3. The Worker Agent uses `github-mcp` to fetch PR content.
4. The MCP Server executes securely within the sandbox via Stdio.
