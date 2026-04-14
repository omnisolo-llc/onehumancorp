---
status: PENDING
agent: Implementer
priority: P1
---

# Title: Integrate Hybrid CRDT State Synchronization MCP Server

## Problem Statement
The OHC Hybrid Architecture requires seamless operations across Cloud (PostgreSQL) and Standalone (local SQLite) modes. Currently, synchronizing state between the local Standalone mode and the multi-tenant Cloud backend is limited. Agents need an MCP that facilitates Conflict-free Replicated Data Type (CRDT) based synchronization for shared data structures, ensuring robust offline capabilities with seamless eventual consistency upon reconnection.

## Research Report
- **Market Context**: Existing systems either rely entirely on the cloud (Replit Agent) or operate entirely locally (Claude Code). OHC's Hybrid workflow requires bidirectional state sync. Standard `fs` or basic database tools don't inherently handle conflict resolution for concurrently edited agent memories or plans.
- **OHC Requirement**: A "Hybrid CRDT Sync MCP" that exposes tools to merge, push, and pull CRDT payloads across the OHC-SIP shared memory boundary.
- **Security & Multi-Tenancy**: The Cloud backend must validate the `organization_id` derived from `auth.Claims` to ensure cross-tenant CRDT updates are completely isolated. Local Standalone mode relies on the user's primary identity.

## Design Doc
- **Module Path**: `srcs/server/tools/hybridcrdtmcp`
- **Architecture**:
  - `crdt_pull`: Fetch the latest CRDT state vector for a given entity from the Cloud backend (or return local if standalone).
  - `crdt_push`: Submit local CRDT mutations to the Cloud backend.
  - `crdt_merge`: Locally compute the intersection of state vectors.
- **Schema Impact**: Add a `crdt_vector JSONB` column to the `shared_tasks` table to store Lamport timestamps and logical clocks.

## Implementation Prompt
Hello Implementer agent! Your mission is to build the Hybrid CRDT State Synchronization MCP.
1. Create `srcs/server/tools/hybridcrdtmcp/tool.go` implementing the `crdt_pull`, `crdt_push`, and `crdt_merge` MCP tools.
2. Ensure that the `InputSchema` for each tool is typed as `json.RawMessage` to prevent runtime validation failures.
3. Add multi-tenant checks to the push/pull logic: if `OHC_MULTITENANT=true`, validate `organization_id` in context.
4. Implement tests for standard merge conflicts and offline buffering.

## Priority
P1

## Estimated Scope
Medium
