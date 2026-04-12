---
status: PENDING
priority: P0
scope: Large
---
# Title: Architect and Implement KAIROS Hybrid Agentic OS Orchestration

## Problem Statement
The OHC swarm requires a centralized, distributed KAIROS system for agents to securely coordinate and track task execution across both Cloud (Postgres) and Standalone (SQLite) architectures. Without the Shared Task List, Teammate Mesh, and AutoDream memory pipelines, the swarm cannot achieve absolute autonomy or resolve DAG dependencies safely.

## Research Report
Current state relies on independent agent operations without strong orchestration guarantees. We need to implement Phase 1 (Shared Task List) with hybrid locking, Phase 2 (Teammate Mesh) for low-latency Pub/Sub, and Phase 3 (AutoDream) for semantic vector memory. Reference `CLAUDE_OHC.md` and `docs/features/kairos/hybrid_os_premium_architecture.md`.

## Design Doc
- **Phase 1:** Add robust `shared_tasks` state machine tracking with `state_machine_transitions`.
- **Phase 2:** Implement `mesh:tasks` and `mesh:coordination` endpoints via Redis/Memory.
- **Phase 3:** Develop the AutoDream worker to parse YAML memories into `pgvector` stored in `consolidated_memory`.

## Implementation Prompt
Implementer Agent: You are required to implement the KAIROS core.
1. Analyze `docs/features/kairos/hybrid_os_premium_architecture.md`.
2. Develop the background workers for the AutoDream pipeline in `srcs/server/orchestration/`.
3. Ensure the Task Decomposition state machine tracks states in the database effectively.
4. Write comprehensive tests and verify with `bazelisk test //...`.
