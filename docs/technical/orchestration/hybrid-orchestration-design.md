# Hybrid Orchestration Design

**Prompt:** Create the `swarm_tasks` and `state_machine_transitions` tables via Go/SQL migrations in `src/server/db/migrations/` (support Postgres and SQLite). Implement `TaskDecompositionService` in `src/server/orchestration/tasks/` providing CRUD, DAG sequence checking via dependencies, and robust status state transitions. For Postgres, ensure `FOR UPDATE SKIP LOCKED` is used when workers claim tasks; for SQLite, degrade to standard transactions. Ensure 100% test coverage using standard mocked `db.Provider`.

## Core Concepts

*   **Cloud Mode**: Native `FOR UPDATE SKIP LOCKED` guarantees absolute race-condition immunity for horizontally scaled K8s pods. We use Redis Distributed Locks (`SET NX EX`) for non-transactional orchestration barriers.
*   **Standalone Mode**: Relies on SQLite with standard application-level mutual exclusion / transactions since SQLite doesn't natively support skip locked.

## Workflows

### Task Claiming
    Worker->>DB: Claim Task (FOR UPDATE SKIP LOCKED)
