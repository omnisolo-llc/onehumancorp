---
status: PENDING
agent: Researcher
priority: P0
---

# Title: Integrate Hybrid Local-to-Cloud State Sync MCP Server

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) operates across multi-tenant Cloud (PostgreSQL) and single-user Standalone (SQLite) modes. A critical gap identified is the inability for agents operating locally in Standalone Desktop Mode to synchronize their offline/local state (e.g., KAIROS Shared Task List, `agent_missions`, `agent_session_data`) with the cloud backends when connectivity is restored or when agents decide a task requires cloud-native compute delegation. Without a standard local-to-cloud sync tool, agents are isolated to the single-machine boundary and cannot hand off tasks transparently.

## Research Report
- **Market Context**: Most sync solutions (like PowerSync or ElectricSQL) are application-level data sync utilities intended for client-to-server UX state. For an *Agentic* architecture, the sync mechanism must be exposed to the Agent itself via the Model Context Protocol (MCP) so the agent can autonomously decide *what* and *when* to sync to prevent unnecessary global token/data burn.
- **OHC Requirement**: We need a "Hybrid Local-to-Cloud State Sync MCP Proxy". The agent should be able to trigger a `sync_context_to_cloud` or `fetch_cloud_delegation_status` tool.
- **Tooling Discovery**: A dedicated MCP adapter wrapping OHC's internal `database.SyncProvider` that handles SQLite-to-PostgreSQL conflict resolution. The MCP server ensures agents can push their SQLite KAIROS state to the K8s Postgres tenant partition using their `auth.Claims`.
- **Security & Multi-Tenancy**: The agent must provide its valid `JWT` representing its tenant (`organization_id`). The sync tool must only push/pull data scoped to that exact tenant to prevent multi-tenant bleed.

## Design Doc
- **Module Path**: `srcs/server/tools/statesyncmcp`
- **Architecture**:
  - Implements the Model Context Protocol (MCP) tools: `list_tools`, `call_tool`.
  - Exposes tools: `sync_local_to_cloud`, `sync_cloud_to_local`, `get_sync_status`.
  - **Dependencies**: Uses `hub.DB().Provider()` to extract local state, and a Go HTTP client or gRPC client to send payloads to the Cloud API.
  - **Conflict Resolution**: Last-Write-Wins (LWW) based on `updated_at` timestamps for state machine transitions.

## Implementation Prompt
Hello Implementer agent!
1. Create a new directory `srcs/server/tools/statesyncmcp`.
2. Abstract the sync logic behind an interface `mcp.StateSyncProvider` with methods `SyncUp`, `SyncDown`, `GetStatus`.
3. Implement `ListTools` to expose `sync_local_to_cloud`, `sync_cloud_to_local`, and `get_sync_status`.
4. Implement `CallTool`:
   - Inject `auth.Claims` from the context.
   - For `sync_local_to_cloud`, query the local SQLite DB for unsynced state transitions, serialize them, and push them to the configured `OHC_CORE_URL` or Cloud API endpoint.
   - For `sync_cloud_to_local`, fetch completed tasks from the cloud and update the local SQLite database.
5. Provide a fallback mock or no-op if running natively in the Cloud without a local SQLite counterpart.
6. Achieve >90% test coverage for the `statesyncmcp` package.

## Priority
P0

## Estimated Scope
Large
