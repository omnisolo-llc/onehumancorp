---
status: PENDING
agent: Jules
priority: P1
---

# Title: Integrate Hybrid Feature Flag & Configuration MCP Server

## Problem Statement
The OHC Hybrid Architecture requires agents to toggle feature flags or read configurations autonomously. In Cloud mode, this relies on a scalable database or multi-tenant configuration store, while in Standalone mode, it must fallback to a local SQLite configuration table. Without a unified MCP tool, agents cannot inspect or modify system behavior (like enabling experimental workflows or adjusting limits) across both modes, limiting their operational autonomy.

## Research Report
- **Market Context**: Evaluated standard MCP configuration servers. Existing configuration MCP tools are tailored for specific platforms and do not support a hybrid multi-tenant (Cloud) vs. single-user (Standalone) architecture.
- **OHC Requirement**: A custom OHC MCP adapter is required. We need a "Hybrid Feature Flag MCP Server" that dynamically binds to either `pgx` (Postgres) or `sqlite3` using the internal `db.Provider` interface.
- **Security & Multi-Tenancy**: In Cloud mode, access must be strictly scoped to the tenant's `organization_id` via `auth.Claims`. An agent in Cloud mode must never be able to toggle a feature flag for another tenant. In Standalone mode, the local database applies.

## Design Doc
- **Module Path**: `srcs/server/tools/configmcp`
- **Architecture**:
  - Implements the Model Context Protocol (MCP) for `list_tools` and `call_tool`.
  - Exposes tools: `get_feature_flag`, `set_feature_flag`, and `list_configs`.
  - Dynamically uses the system's `db.Provider` to determine the underlying engine and execute queries.
  - For Cloud (Postgres): Enforces tenant scoping by requiring `auth.Claims.OrganizationID` in all query filters.
  - For Standalone (SQLite): Uses the local database instance and handles concurrency appropriately.
- **Security**: The `set_feature_flag` tool must require explicit context elevation or be restricted to certain agent roles.

## Implementation Prompt
Hello Implementer agent!
1. Create a new directory `srcs/server/tools/configmcp`.
2. Implement the MCP server conforming to the project's internal tool registry interfaces.
3. Implement `ListTools` returning definitions for `get_feature_flag`, `set_feature_flag`, and `list_configs`.
4. Implement `CallTool`:
   - Inject `auth.Claims` from the context.
   - Detect DB mode using the `db.Provider`.
   - Ensure Cloud queries enforce tenant scoping using `auth.Claims`.
   - Return clear errors if configurations are not found or access is denied.
5. Add unit tests for both Postgres and SQLite modes proving cross-mode functionality.
6. Achieve >90% test coverage for the `configmcp` package.

## Priority
P1

## Estimated Scope
Medium
