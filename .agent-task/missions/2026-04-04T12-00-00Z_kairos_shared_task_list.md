---
status: IN_PROGRESS
agent: Implementer
---

# Title: Implement Shared Task List (KAIROS Orchestration)

## Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. We lack a robust distributed state machine to track asynchronous tasks (`swarm_tasks` and `shared_tasks`) across the swarm with exact sequence and DAG dependencies.

## Research Report
Based on the KAIROS Orchestration Design Doc (`docs/kairos_orchestration_design.md`):
- We must utilize `swarm_tasks` for mission-critical steps and `shared_tasks` for inter-agent delegation.
- In Cloud Mode (PostgreSQL Native), we need to rely on `FOR UPDATE SKIP LOCKED` for lock-free concurrency and zero TOCTOU race conditions.
- In Standalone Mode (SQLite Fallback), we degrade gracefully utilizing local table locks.
- DAG Dependencies must enforce sequence and parallel task unblocking.

## Design Doc
1. **Schema Updates:** Ensure `swarm_tasks` and `shared_tasks` are created or updated correctly.
2. **Go Models:** Define DAG dependency logic and task struct updates to enforce sequence blocking.
3. **Provider:** Add PostgreSQL-specific logic utilizing `FOR UPDATE SKIP LOCKED` inside explicit transactions to avoid releasing row locks too early.
4. **Fallback:** Use `pool.IsSQLite()` checks to degrade gracefully and apply SQLite single-node concurrency mechanisms.

## Implementation Prompt
1. Read `docs/kairos_orchestration_design.md` for context.
2. Check `srcs/server/db/migrations/` to ensure `swarm_tasks` and `shared_tasks` exist. If not, create them. Remember to update `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Implement `ClaimTask` or similar logic in `srcs/server/scheduler/` or `srcs/server/orchestration/` ensuring you open a transaction `tx, err := pool.Begin(ctx)` for PostgreSQL locks.
4. Ensure SQLite mode handles task locking appropriately using `UPDATE ... RETURNING`.
5. Implement DAG blocking/unblocking logic so dependent tasks only run when parent tasks complete.
6. Write unit tests achieving >95% coverage, verifying both SQLite and PostgreSQL behaviors.
7. Use Bazel for testing before PR.

## Priority
P0

## Estimated Scope
Large
