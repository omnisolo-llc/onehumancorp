---
status: DONE
agent: Principal Product Architect & KAIROS Orchestrator (L7)
---

# Title: Architect the KAIROS Hybrid Agentic OS Backbone

## Problem Statement
The OHC Agent Swarm requires a robust orchestration engine (KAIROS) capable of operating natively in both Kubernetes (Postgres/Redis) and Standalone Desktop (SQLite/Memory) environments. This engine must manage shared task DAGs, handle sub-agent distributed queuing, facilitate real-time teammate mesh communications, and securely consolidate context into long-term vector embeddings via AutoDream.

## Research Report
1. **Shared Task List**: Investigated `srcs/server/orchestration/tasks_db.go` and `srcs/server/orchestration/statemachine/machine.go`. Concurrency is managed via PostgreSQL `FOR UPDATE SKIP LOCKED` and SQLite application-level mutexes. Task dependencies (`task_dependencies`) form the DAG.
2. **Sub-Agent Queue**: `sub_agent_jobs` table supports queueing decoupled execution for worker agents (see `srcs/server/orchestration/queue/queue.go`).
3. **Teammate Mesh**: The real-time mesh leverages Centrifuge and Redis Pub/Sub for cloud modes and in-memory transport for standalone modes.
4. **AutoDream Pipeline**: Compresses session data and `.agent-task/memory` context into `autodream_memories` using `VECTOR(1536)` in `pgvector` and standard blobs in SQLite.

## Design Doc
A premium design document combining these findings has been created in `docs/features/kairos/hybrid_ai_os_implementation_guide.md`, fulfilling the Execution Playbook phases:
- **Phase 1 (UltraPlan/Decomposition)**: Shared Task DB Schema and sequence diagram.
- **Phase 2 (Orchestration)**: Teammate Mesh APIs for realtime synchronization.
- **Phase 3 (AutoDream)**: Vector DB schema integration.

## Implementation Prompt
This is an architectural task. The implementation details have been synthesized into `docs/features/kairos/hybrid_ai_os_implementation_guide.md`. Development teams can implement against these specifications by extending the database migrations in `srcs/server/db/migrations/` and the queue services in `srcs/server/orchestration/queue/`.

## Priority
P0

## Estimated Scope
Large
