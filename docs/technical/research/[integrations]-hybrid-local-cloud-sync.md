# Title: Integration Blueprint: SQLite-to-Postgres Hybrid MCP Sync

## Problem Statement
OHC supports both Cloud-native (Postgres) and Standalone (SQLite) modes. However, local agents lack a seamless data synchronization tool to push local SQLite insights/events to the multi-tenant Postgres cloud, and vice versa. There is a gap in the hybrid architecture for an out-of-the-box MCP tool that securely syncs state between the local footprint and the remote footprint.

## Research Report
Market research indicates that most Model Context Protocol (MCP) implementations are designed strictly for cloud or strictly for local execution. Tools like standard replication (e.g., Litestream) are too low-level and don't respect multi-tenant application boundaries. By introducing an application-level Hybrid MCP Sync tool, we can allow agents to intelligently choose what context to push to the cloud (for long-term vector truth) and what to pull to the local device.
- **Competitors:** Claude Code focuses on local execution, Replit Agent on cloud. OHC's "Unfair Advantage" is the hybrid nature.
- **Reference:** OHC Market Strategy.

## Design Doc
**Architecture:**
- Add a new package `src/server/lib/integrations/hybrid_sync/`.
- Introduce a `SyncManager` that implements the MCP Tool interface.
- Must support both `*sql.DB` driver variants (SQLite and Postgres), dynamically inspecting the driver using `db != nil && fmt.Sprintf("%T", db.Driver()) == "*sqlite.Driver"`.

**API Contracts:**
- `PushState(ctx async context, payload SyncPayload) error`
- `PullState(ctx async context, filter SyncFilter) (*SyncPayload, error)`

**DB Schema Changes:**
- None required; relies on existing data structures (e.g., orchestration events).

**Security:**
- Must validate `organization_id` in cloud mode.

## Implementation Prompt
"Implement the Hybrid MCP Sync tool in `src/server/lib/integrations/hybrid_sync/`.
1. Create `sync.rs` defining the `SyncManager` and its MCP capabilities (`PushState` and `PullState`).
2. Implement driver-agnostic logic. To determine if the connection is SQLite, use: `db != nil && fmt.Sprintf("%T", db.Driver()) == "*sqlite.Driver"`. For SQLite, strip Postgres-specific prefixes, replace `NOW()` with `CURRENT_TIMESTAMP`, and remove `FOR UPDATE SKIP LOCKED`.
3. Create tests in `sync_test.rs` using `tempfile::tempdir()` for isolated testing. Never hardcode workspace directories.
4. Update or create the adjacent `BUILD.bazel` file, ensuring `srcs` array accurately reflects the new files."

## Priority
P1

## Estimated Scope
Medium
