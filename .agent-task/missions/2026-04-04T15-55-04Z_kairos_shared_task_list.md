---
title: "Implement Shared Task List (KAIROS Orchestration)"
problem_statement: "The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. Currently, agents lack a robust distributed state machine to track asynchronous tasks (`swarm_tasks` and `shared_tasks`) across the swarm with exact sequence and DAG dependencies."
priority: "P0"
estimated_scope: "Large"
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# Title: Implement Shared Task List (KAIROS Orchestration)

## Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. Currently, agents lack a robust distributed state machine to track asynchronous tasks (`swarm_tasks` and `shared_tasks`) across the swarm with exact sequence and DAG dependencies.

## Research Report
Based on the KAIROS Orchestration Design Doc (`docs/kairos_orchestration_design.md`):
- We must utilize `swarm_tasks` for mission-critical steps and `shared_tasks` for inter-agent delegation.
- **Cloud Mode (PostgreSQL Native)**: We need to rely on `FOR UPDATE SKIP LOCKED` inside explicit transactions (`tx, err := pool.Begin(ctx)`) for lock-free concurrency and zero TOCTOU (Time-Of-Check to Time-Of-Use) race conditions across parallel K8s agent pods. Avoid making external network calls inside this active transaction.
- **Standalone Mode (SQLite Fallback)**: We must degrade gracefully to single-node concurrency guarantees, utilizing local table locks (`UPDATE ... RETURNING` or mutexes via `pool.IsSQLite()`) to preserve transactional integrity for single-user workloads.
- **DAG Dependencies**: Ensure sequence and parallel task unblocking (e.g., frontend tasks block on backend completion) using KAIROS `task_dependencies`.

## Design Doc
1. **Schema Updates:** Ensure `swarm_tasks`, `shared_tasks`, and `task_dependencies` are created or updated correctly. Map to PostgreSQL explicitly.
2. **Go Models:** Define DAG dependency logic and task struct updates to enforce sequence blocking.
3. **Provider Implementation (`srcs/server/orchestration/tasks.go` or `task_orchestrator.go`):** Add PostgreSQL-specific logic utilizing `FOR UPDATE SKIP LOCKED` inside explicit transactions to avoid releasing row locks prematurely. For `db.Provider`, use `db.Rows` for iteration.
4. **Fallback Mechanism:** Use `pool.IsSQLite()` checks to degrade gracefully and apply SQLite single-node concurrency mechanisms via `UPDATE ... RETURNING`. Do not rely on SQLite for exact Vector NN searches.

## Implementation Prompt
1. Read `docs/kairos_orchestration_design.md` for architectural context.
2. Verify `srcs/server/db/migrations/` to ensure `swarm_tasks` and `shared_tasks` schemas exist. If adding a new `.sql` file, you **MUST** add it to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Implement `ClaimTask` or similar polling logic ensuring you open an explicit database transaction (`tx, err := pool.Begin(ctx)`) for PostgreSQL locks (`FOR UPDATE SKIP LOCKED`).
4. Ensure SQLite mode handles task locking appropriately using `UPDATE ... RETURNING` wrapped in an explicit transaction since SQLite doesn't natively support `LIMIT` with it.
5. Implement DAG blocking/unblocking logic so dependent tasks only run when parent tasks complete.
6. Enforce multi-tenant isolation by explicitly applying `organization_id` filters on all `shared_tasks` database queries.
7. Expose high-fidelity metrics via OpenTelemetry.
8. Write unit tests achieving >95% coverage, verifying both SQLite and PostgreSQL behaviors.
9. Verify changes locally by strictly using Bazelisk commands (e.g. `bazelisk test //srcs/server/orchestration:orchestration_test`).

## Priority
P0

## Estimated Scope
Large

</div>
