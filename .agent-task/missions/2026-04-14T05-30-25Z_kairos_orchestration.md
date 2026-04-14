---
agent: Implementer
status: FAILED
priority: P0
scope: Large
---

# Title: Implement KAIROS Hybrid OS Orchestration: Shared Task List, Teammate Mesh, and autoDream

## Problem Statement
The OHC swarm requires a unified backend orchestration system to manage task decomposition, real-time agent coordination, and long-term memory consolidation across both Cloud-Native (PostgreSQL/Redis) and Standalone (SQLite/In-Memory) modes.

## Research Report
- **PostgreSQL**: Optimal for horizontally scaled pod concurrency utilizing `FOR UPDATE SKIP LOCKED`.
- **Redis**: Ideal for the Teammate Mesh transport via Pub/Sub (`mesh:tasks`).
- **pgvector**: Perfect for autoDream's semantic memory consolidation.
- **Graceful Degradation**: Standalone mode must fall back to SQLite transactions and in-memory channel broadcasts to preserve resources.

## Design Doc
Reference the master design document for complete architectural specifications, schemas, and API contracts:
`docs/architecture/KAIROS_HYBRID_OS_IMPLEMENTATION_PLAN.md`

## Implementation Prompt
You are an Implementer agent. Your mission is to implement the KAIROS Hybrid OS Orchestration components exactly as specified in the Design Doc.

1. Create a new database migration file in `srcs/server/db/migrations/` named `060_kairos_hybrid_orchestration.sql` containing the schemas for `shared_tasks_kairos` and `sub_agent_queue_kairos`. Ensure the migration uses Goose annotations (`-- +goose Up` and `-- +goose Down`).
2. Create the data access layer in a new file `srcs/server/orchestration/kairos_tasks_db.go`. Implement the task claiming logic utilizing `FOR UPDATE SKIP LOCKED` for PostgreSQL, with an explicit fallback for SQLite.
3. Create the Teammate Mesh API handlers in a new file `srcs/server/orchestration/kairos_mesh_api.go` providing the structures for `mesh:tasks` and `mesh:coordination` channels.
4. Ensure all files belong to the `orchestration` package.
5. Provide comprehensive unit tests in `srcs/server/orchestration/kairos_tasks_test.go`.
6. Run `~/go/bin/bazelisk test //srcs/server/orchestration:...` to verify your implementation.

## Priority
P0

## Estimated Scope
Large
