<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: Integration Blueprint: Hybrid Dynamic Tool Discovery MCP

## Problem Statement
Current implementations for Dynamic Tool Discovery via Switchboard, while robust in the Cloud-native environment (K8s/LangGraph, Redis, SPIRE), fail to scale down for local standalone execution. The standalone SQLite agents are unable to resolve or load new tools Just-In-Time because they do not have access to a remote vector search registry or SPIRE infrastructure, leading to static agent tool restrictions out-of-the-box. We lack a "Hybrid Dynamic Tool Discovery MCP" proxy that can act as a lightweight, local-first search mechanism when running in standalone mode, while effortlessly routing to the enterprise Switchboard when running in the Cloud.

## Research Report
The existing `design-hook-dynamic-tool-discovery.md` establishes the framework for discovering tools dynamically via the Switchboard `/v1/tools/search` and authenticating through SPIRE. However, Replit and Claude Code have demonstrated success relying purely on the file-system and SQLite for local tool indexing. OHC's "Unfair Advantage" requires an application-level Hybrid MCP proxy that bridges these concepts. By utilizing the SQLite native FTS (Full-Text Search) capabilities for standalone environments, we can offer local tool indexing that mirrors the vector capabilities of the Cloud environment without heavy infrastructure dependencies.

## Design Doc
**Architecture:**
- Create a new package `src/server/lib/integrations/hybrid_discovery/`.
- The `DiscoveryProxy` will implement the MCP Tool interface.
- Must dynamically choose the backend:
  - If `db != nil && fmt.Sprintf("%T", db.Driver()) == "*sqlite.Driver"`, it initializes the local SQLite FTS strategy.
  - If it is Postgres/Cloud, it routes the request to the `Switchboard` microservice via gRPC.

**API Contracts:**
- `SearchTools(ctx async context, intent string) ([]ToolSpec, error)`
- `RequestToolSVID(ctx async context, toolName string) (SVID, error)` (Gracefully mocks/bypasses in SQLite).

**DB Schema Changes:**
- For Postgres: None.
- For SQLite: Ensure a generic FTS virtual table exists or dynamically populate `sqlite_mcp_tools`.

**Security:**
- Validate `organization_id` strictly when routing to Switchboard in Postgres mode.
- Ensure the Mock SVID system in SQLite Mode restricts tools to the current filesystem boundary to prevent broad privilege escalation.

## Implementation Prompt
"Implement the Hybrid Dynamic Tool Discovery MCP tool in `src/server/lib/integrations/hybrid_discovery/`.
1. Create `discovery.rs` defining the `DiscoveryProxy` and its MCP capabilities (`SearchTools` and `RequestToolSVID`).
2. Implement driver-agnostic logic. To determine if the connection is SQLite, use: `db != nil && fmt.Sprintf("%T", db.Driver()) == "*sqlite.Driver"`.
3. In SQLite mode, implement a rudimentary Full-Text Search against the local registry, returning valid `ToolSpec` definitions. Bypass SPIRE and generate a local pseudo-SVID.
4. In Postgres mode, route requests to the remote Switchboard and authenticate using proper SPIRE calls.
5. Create tests in `discovery_test.rs` verifying routing mechanisms.
6. Create an E2E test starting from UI interaction to verify the fallback logic functions as intended.
7. Update `BUILD.bazel` to include new dependencies."

## Priority
P1

## Estimated Scope
Medium
</div>
