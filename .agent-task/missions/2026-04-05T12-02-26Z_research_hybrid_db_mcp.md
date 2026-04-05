---
status: DONE
agent: Jules
priority: P1
---

# Title: Integrate Hybrid DB Inspector MCP Server

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) operates across multi-tenant Cloud (PostgreSQL) and single-user Standalone (SQLite) modes. Currently, AI agents lack a unified interface to introspect database schemas, execute diagnostic queries, and monitor active connections transparently across both modes. This gap forces agents to use disparate tooling or rely heavily on application-level abstraction, hindering rapid debugging and schema evolution during runtime autonomous operations.

## Research Report
- **Market Context**: The Model Context Protocol (MCP) ecosystem provides standardized server capabilities for agents. Existing tools like the official `sqlite-mcp-server` or `postgres-mcp-server` are mode-specific.
- **OHC Requirement**: We need a "Hybrid Database Inspector MCP Server" that dynamically binds to either `pgx` (Postgres) or `sqlite3` depending on the `OHC_STANDALONE` environment variable or dynamic injection via the `orchestration.Hub`.
- **Tooling Discovery**: A bespoke MCP adapter mapping internal OHC schema structures to the MCP schema definition format (`mcp.ListTools`, `mcp.CallTool`) is the most robust approach to achieve full autonomy in schema debugging without risking multi-tenant bleed.
- **PowerSync Consideration**: PowerSync rules enforce tenant isolation; the MCP implementation must inherit `auth.Claims` and inject them into its DB connection context to prevent unauthorized cross-tenant queries in Cloud mode.

## Design Doc
- **Module Path**: `srcs/server/tools/dbinspector`
- **Architecture**:
  - Implements the Model Context Protocol (MCP) for `list_tools` and `call_tool`.
  - Exposes `inspect_schema`, `run_query`, and `get_stats` tools.
  - Dynamically uses `hub.DB().Provider()` to determine the underlying engine.
  - For PostgreSQL: Enforces `SET LOCAL search_path = $tenant_id` and role validation via context.
  - For SQLite: Bypasses tenant rules but utilizes orchestrator-level concurrency throttle (`acquireThrottle`) to prevent `database is locked` errors.
- **Security**: Strict READ-ONLY mode by default. `run_query` must explicitly reject `INSERT`, `UPDATE`, `DELETE`, `DROP`, `ALTER` unless an `override_safety_lock` flag is passed via an authorized `admin` claim.

## Implementation Prompt
1. Create a new directory `srcs/server/tools/dbinspector`.
2. Implement the MCP server conforming to the project's internal tool registry interfaces (`srcs/server/tools/tools.go` if applicable, or as an independent MCP package).
3. Implement `ListTools` returning definitions for `inspect_schema`, `run_query`, and `get_stats`.
4. Implement `CallTool`:
   - Inject `auth.Claims` from the context.
   - Detect DB mode: `if hub.DB().IsSQLite() { ... } else { ... }`.
   - Ensure Cloud queries prefix with multi-tenant scopes or safe execution parameters.
   - For SQLite, wrap the execution in a `Throttle` mutex to handle concurrency safely.
5. Add unit tests for both Postgres (using `pgxmock` or similar) and SQLite (in-memory) proving cross-mode functionality.
6. Achieve >90% test coverage for the `dbinspector` package.

## Priority
P1

## Estimated Scope
Medium
