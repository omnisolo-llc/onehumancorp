<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# KAIROS Orchestration: Unified Hybrid AI OS Architecture

## Phase 1: Shared Task List (Task Decomposition)
The Shared Task List serves as the brain of the OHC Swarm.
- **Database:** PostgreSQL (`FOR UPDATE SKIP LOCKED`) for Cloud, SQLite (Mutex) for Standalone.
- **Schema:** Tracks state via `shared_tasks` and `state_machine_transitions`.

```mermaid
sequenceDiagram
    participant UI as Dashboard
    participant KAIROS as Orchestrator
    participant DB as Shared Task List (Pg/SQLite)
    participant Agent as Swarm Agent

    UI->>KAIROS: Feature Request
    KAIROS->>DB: INSERT INTO shared_tasks (PENDING)
    Agent->>DB: Claim Task (Lock)
    Agent->>DB: UPDATE shared_tasks (IN_PROGRESS)
```

## Phase 2: Teammate Mesh APIs (Orchestration)
The Teammate Mesh provides real-time event broadcasting and zero-friction coordination across pods.
- **Channels:** `mesh:tasks`, `mesh:coordination`.
- **Transport:** Redis Pub/Sub (Cloud), Memory Transport (Standalone).

## Phase 3: AutoDream Data Pipeline (Memory)
Continuous long-term memory vectorization.
- **Mechanism:** Parses ephemeral YAML files in `.agent-task/memory/` to chunked embeddings via Minimax.
- **Storage:** Upserts to `consolidated_memory` using `pgvector`.

</div>
