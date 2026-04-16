status: DONE
agent: Implementer

Parent: #4296

## Title
Implement Hybrid MCP Local-to-Cloud State Synchronization for External Tools

## Problem Statement
The OHC Hybrid OS requires external integrations (like Jira, Notion, Airtable) to function across both Standalone (SQLite) and Cloud-Native (Postgres/Redis) modes. Currently, tool execution states and credentials risk fragmentation when agents transition between offline desktop modes and cloud swarms, creating a disjointed integration experience compared to Claude Code.

## Research Report
Competitor analysis shows OpenClaw and Replit Agent rely purely on cloud credential stores, blocking local offline workflow preparations. OHC's "Blue Ocean" advantage requires building a Hybrid Model Context Protocol (MCP) Synchronization layer. This layer will buffer tool execution metadata locally in SQLite during Standalone mode and safely replicate it via SPIFFE/SPIRE to the OHC Central Database (OHC-SIP) when online.

## Design Doc
- **Schema Updates**: Design a robust `hybrid_mcp_sync_queue` table compatible with both SQLite and Postgres to buffer integration state changes.
- **MCP Mesh Proxy**: Implement an interceptor pattern to cache MCP API requests/responses locally and replay/sync them to the Cloud MCP Gateway.
- **SPIFFE/SPIRE SVID**: Ensure mTLS authentication verifies the local daemon before merging tool state into the cloud Postgres instances.

## Implementation Prompt
Hello Implementer agent!
1. Create a new database migration file in `srcs/server/db/migrations/` to define the `hybrid_mcp_sync_queue` table.
2. Implement a new `McpSyncProxy` module within the server source tree (e.g. `srcs/server/integrations/`). It should buffer integration metadata into `db.Provider` and periodically sync to the cloud gateway using SPIFFE mTLS.
3. Create appropriate Bazel targets for the new package and verify with `bazelisk test` on the newly created targets, ensuring >90% coverage.

## Priority
P1

## Estimated Scope
Medium
