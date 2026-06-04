# KAIROS Orchestration Design

## Overview
The KAIROS Orchestrator provides absolute autonomy for the OHC Swarm. It requires a distributed state machine mapped to both Cloud-Native and Standalone Desktop environments.

## Phase 1: Shared Task List (The Brain)
A durable distributed state machine for swarm tasks (`swarm_tasks` and `shared_tasks`).
- **Cloud Mode (PostgreSQL):** Uses `FOR UPDATE SKIP LOCKED` inside explicit transactions (`tx.Begin()`) to ensure horizontal pod concurrency.
- **Standalone Mode (SQLite):** Degrades gracefully utilizing explicit `UPDATE ... RETURNING` (or select-then-update) combined with application-level semaphores via `pool.IsSQLite()` checks.
- **Dependencies:** DAG dependencies enforced via `task_dependencies` and `swarm_task_dependencies` join tables to prevent sequence race conditions.

## Phase 2: Teammate Mesh (The Nerves)
A high-throughput realtime event bus for intent broadcast and memory coordination.
- **Cloud Mode:** Agents publish to production Redis Pub/Sub channels (`mesh:tasks`, `mesh:coordination`). Centrifuge Node handles downstream WebSockets.
- **Standalone Mode:** Fallbacks to in-memory Go channels for offline functionality.
- **Security:** Zero Secrets architecture relying entirely on SPIFFE/SPIRE mTLS identities.

## Phase 3: AutoDream (The Memory)
The long-term persistence layer embedding ephemeral context.
- Uses `pgvector` in PostgreSQL for exact Nearest Neighbor search of `autodream_memories`.
- Consolidates episodic worker history via Minimax LLMs.
- SQLite fallback maps vectors to local blobs or uses recency-based search.

## Aesthetic Core
Any UI built around this architecture must enforce the Visual Excellence Mandate:
<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>

## Sequence Diagram (Shared Task Claiming)
```mermaid
sequenceDiagram
    participant Agent as Worker Agent
    participant DB as Postgres (shared_tasks)
    participant Hub as Teammate Mesh Hub

    Agent->>DB: BEGIN
    Agent->>DB: SELECT id FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED LIMIT 1
    alt Task Found
        DB-->>Agent: Returns Task 123
        Agent->>DB: UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = 'worker-1' WHERE id = 123
        Agent->>DB: COMMIT
        Agent->>Hub: Publish MeshEvent {topic: 'task.assigned', payload: Task 123}
    else No Task Found
        DB-->>Agent: Returns 0 rows
        Agent->>DB: ROLLBACK
    end
```
