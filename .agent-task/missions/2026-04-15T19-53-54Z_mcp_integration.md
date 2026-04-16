**Title**: [backend] Standardize Agent Tools to use Model Context Protocol (MCP)

**Problem Statement:**
OHC agents currently use custom, bespoke tool definitions. This creates friction when trying to integrate new tools or when trying to share tools with other agent ecosystems. Market leaders like Claude Code have entirely pivoted their harness entrypoints and tool structures to use the Model Context Protocol (MCP).

**Research Report:**
Analysis of Claude Code shows:
- The entire agent harness is exposed as an MCP server (`src/entrypoints/mcp.ts`).
- Tools are internally defined and then mapped seamlessly to `ListToolsRequestSchema` and `CallToolRequestSchema`.
- External MCP servers (stdio, HTTP, SSE) are dynamically loaded via `.mcp.json` config scopes.
- Adopting MCP allows the harness to instantly consume hundreds of existing open-source MCP tools without writing custom wrappers.

**Design Doc:**
1.  **Module:** `srcs/server/harness/mcp/`
2.  **Architecture:**
    - Build an MCP Client manager capable of connecting to stdio and SSE MCP servers.
    - Refactor the OHC internal Tool interface to map 1:1 with the MCP Tool specification (`name`, `description`, `inputSchema`).
    - Expose the OHC Agent Harness itself as an MCP Server so it can be controlled by thin clients.
3.  **Database:** Add an `mcp_servers` table to PostgreSQL (Cloud-Native) / SQLite (Standalone) to persist authorized external tool servers per user.

**Implementation Prompt:**
"As an Implementer, build an MCP client manager in `srcs/server/harness/mcp/`. Define a `ServerConfig` struct. Implement a connection manager that can spawn and communicate with a stdio-based MCP server. Write a function `ConvertToMCPTool(t InternalTool) mcp.Tool` to map our tools. Include comprehensive tests mocking the stdio streams. Add Prometheus metrics `ohc_mcp_tool_calls_total`."

**Priority:** P2
**Estimated Scope:** Medium
