---
status: DONE
agent: Jules
---

# Title: Implement KAIROS Sub-Agent Orchestration & DAG Dependencies

## Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. Currently, the system lacks a robust distributed state machine to track asynchronous tasks (`swarm_tasks` and `shared_tasks`) across the swarm with exact sequence and DAG dependencies, which hinders complex sub-agent orchestration and UltraPlan deliberation cycles.

## Research Report
Based on the updated KAIROS Orchestration Design Doc (`docs/KAIROS_ORCHESTRATOR_DESIGN.md`):
- We must utilize `swarm_tasks` for mission-critical steps and sub-agent orchestration.
- In Cloud Mode (PostgreSQL Native), we need to rely on `FOR UPDATE SKIP LOCKED` for lock-free concurrency and zero TOCTOU race conditions.
- In Standalone Mode (SQLite Fallback), we degrade gracefully utilizing local table locks.
- DAG Dependencies must enforce sequence and parallel task unblocking (e.g., frontend tasks block on backend completion).
- The Teammate Mesh APIs provide real-time coordination, and AutoDream pipelines consolidate memory.

## Design Doc
See `docs/KAIROS_ORCHESTRATOR_DESIGN.md` for the complete architectural overview.
1. **Schema Updates:** Ensure `swarm_tasks` and `shared_tasks` support DAG dependency structures (e.g., `parent_plan_id`, `dependencies` array).
2. **Go Models:** Define DAG dependency logic and task struct updates to enforce sequence blocking.
3. **Provider:** Add PostgreSQL-specific logic utilizing `FOR UPDATE SKIP LOCKED` inside explicit transactions to avoid releasing row locks too early.
4. **Fallback:** Use `pool.IsSQLite()` checks to degrade gracefully and apply SQLite single-node concurrency mechanisms.

## Implementation Prompt
1. Read `docs/KAIROS_ORCHESTRATOR_DESIGN.md` for context.
2. Check `srcs/server/db/migrations/` to ensure `swarm_tasks` and `shared_tasks` fully support hierarchical sub-agent execution (`parent_plan_id`, etc.). If not, create a new migration. Remember to update `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Enhance `ClaimTask` or similar logic in `srcs/server/orchestration/tasks.go` ensuring you open a transaction `tx, err := pool.Begin(ctx)` for PostgreSQL locks, avoiding TOCTOU race conditions.
4. Ensure SQLite mode handles task locking appropriately using `UPDATE ... RETURNING`.
5. Implement DAG blocking/unblocking logic so dependent tasks only run when parent tasks complete.
6. Write unit tests achieving >90% coverage, verifying both SQLite and PostgreSQL behaviors.
7. Use Bazel for testing before PR. Ensure proper SPIFFE/SPIRE checks exist.

## Priority
P0

## Estimated Scope
Large
