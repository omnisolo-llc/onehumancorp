# Native MCP Tool Integration & Scope Sandboxing

## Problem Statement
OHC currently lacks a standardized way to integrate external tools, APIs, and file systems seamlessly with its agents. Competitors like Claude Code rely heavily on the Model Context Protocol (MCP) to dynamically load and scope capabilities (like file read/write, terminal execution, and browser automation) natively into their Agent Harness. We need to implement a native MCP integration layer.

## Research Report
### Competitive Analysis: Claude Code (v2.1.88)
Claude Code leverages MCP as the central nervous system of its Agent Harness.
- **Dynamic Capabilities**: Tools like `FileEditTool`, `GlobTool`, `BashTool`, and `GrepTool` are all defined and exposed as standardized MCP interfaces.
- **Configuration & Scoping**: Claude Code uses `.mcp.json` to configure MCP servers. This config can restrict file access by defining scopes (e.g., preventing access to sensitive system paths or explicitly allowing specific worktrees).
- **Communication Protocol**: It supports running local MCP servers via `stdio`, `HTTP`, or `SSE` (Server-Sent Events), allowing it to integrate with any language or ecosystem.
- **Rate Limiting & Cost Tracking**: MCP tool calls are integrated with a mock rate limiting service (`mockRateLimits.ts`) and a cost tracker (`cost-tracker.ts`) that intercepts all tool usage, ensuring agent execution does not overrun budgets.

### OHC Gap Analysis
OHC-HA has basic tool integrations, but they are hardcoded and tightly coupled to the orchestrator (`src/server/orchestration/`).
1. We lack a dynamic mechanism to discover and register new tools.
2. We do not have `.mcp.json` scope validation to restrict what files an agent can read or write.
3. Our tool calls are not standardized using the open Model Context Protocol.

## Design Doc
### Architecture
1. **MCP Client Core**: Implement a native MCP client capable of communicating over `stdio` and `HTTP/SSE`.
2. **Tool Registry**: A dynamic registry that parses `.mcp.json` from the target repository/directory, instantiates the defined MCP servers, and registers their tools with the OHC Agent.
3. **Scope & Permission Enforcer**: An interceptor that intercepts file-system tools (like `FileRead` or `FileWrite`) and validates the requested paths against the scopes defined in `.mcp.json` and the OHC global policies.
4. **Telemetry & Cost Proxy**: A wrapper around tool invocations that tracks execution time, token usage, and simulated costs, sending metrics to Prometheus/OpenTelemetry.

### Implementation Protocol
1.  **MCP Protocol Implementation**: Create `src/server/orchestration/harness/mcp/client.go` to implement the core Model Context Protocol (handling initialization, tool discovery, and execution over `stdio`).
2.  **Configuration Loader**: Create `src/server/orchestration/harness/mcp/config.go` to parse `.mcp.json` files and establish connection parameters for dynamic servers.
3.  **Scope Validation**: Create `src/server/orchestration/harness/mcp/scope.go` to intercept path arguments in tools and validate them against an allowed base directory.
4.  **Integration**: Update `src/server/orchestration/task_orchestrator.go` to fetch available tools from the `MCP Client Core` and expose them to the agent's LLM context.

## Visual Context
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">
  <h3>Architecture Diagram: MCP Integration</h3>
  <pre class="mermaid">
  graph TD
    A[OHC Agent] -->|Request Tool| B[MCP Client Core]
    B -->|Check Config| C[.mcp.json Loader]
    B -->|Validate Permissions| D[Scope Enforcer]
    D -- Deny --> E[Agent Rejected]
    D -- Allow --> F[Stdio/HTTP MCP Server]
    F --> G[Execute File/Bash/DB Command]
  </pre>
</div>

## Implementation Prompt
Implement the Native MCP Integration Layer.
1. Create `src/server/orchestration/harness/mcp/client.go` to manage `stdio` sub-processes for MCP servers.
2. Create `src/server/orchestration/harness/mcp/config.go` to parse a standard `.mcp.json` configuration file.
3. Create `src/server/orchestration/harness/mcp/scope.go` that provides an `IsPathAllowed(requestedPath, basePath)` function to prevent directory traversal attacks (`../`).
4. Modify `src/server/orchestration/task_orchestrator.go` to initialize the MCP Client on startup and inject discovered tools into the LLM context.
5. Provide 100% test coverage in `src/server/orchestration/harness/mcp/*_test.go`. Ensure a test specifically tries to read `/etc/passwd` using a simulated FileRead tool and gets denied by `scope.go`.

## Priority
P1

## Estimated Scope
Medium
