<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Orchestration: Shared Task List and Teammate Mesh

## Problem Statement
The OHC swarm currently lacks a durable, synchronized Shared Task List and Realtime Teammate Mesh coordination system, relying purely on static `.agent-task` files. To truly implement "Swarm-as-Code" (OHC-SIP), KAIROS requires a structured backend to track tasks across the multi-tenant Cloud architecture (PostgreSQL/Redis) while gracefully degrading in Desktop mode (SQLite).

## Research Report
- Cloud Native requires Postgres `shared_tasks` and Redis Pub/Sub channels (`mesh:tasks`, `mesh:coordination`).
- Standalone mode requires an SQLite equivalent or in-memory simulation.
- KAIROS acts as the orchestrator by defining sub-agent tasks and publishing mesh events.
- autoDream consolidates architectural findings to vector storage.

## Design Doc
**Architecture:**
- **Database Schema (PostgreSQL):** Create a robust `shared_tasks` table and `task_dependencies` mapping table. Ensure `organization_id` checks for tenant isolation.
- **Teammate Mesh (Redis/gRPC):** Implement a Go service `telemetry/mesh.go` (or similar) bridging Redis Pub/Sub for realtime mesh messaging.
- **autoDream (Vector):** Setup `pgvector` pipeline for `consolidated_memory`.
- **Go Interfaces:** Define `TaskStore` and `MeshClient` interfaces with Postgres/Redis and SQLite implementations.

## Implementation Prompt
Dear Implementer (Domain: apps/api/, services/, lib/),
Please implement the KAIROS Orchestration phase:
1. Create DB migrations for `shared_tasks`, `task_dependencies` and `consolidated_memory` in `srcs/server/db/migrations/`.
2. Implement the `TaskStore` repository in Go (handling SQLite/Postgres hybrid mode).
3. Implement `MeshClient` using Redis Pub/Sub for Cloud and basic channels for Standalone.
4. Integrate autoDream consolidation logic writing to `consolidated_memory`.
5. Ensure $>90\%$ test coverage using `context.WithValue` for mock claims where necessary.

## Priority
P0

## Estimated Scope
Large

</div>
