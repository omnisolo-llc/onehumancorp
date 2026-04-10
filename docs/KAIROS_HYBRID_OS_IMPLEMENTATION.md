<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# KAIROS Orchestration: Unified Hybrid AI OS Architecture

**Phase 1: Shared Task List (Decomposition)**
The Swarm uses a distributed state machine backed by PostgreSQL (`FOR UPDATE SKIP LOCKED`) in cloud mode, falling back to SQLite transactions for standalone environments. This ensures task execution prevents agent collisions.

```mermaid
sequenceDiagram
    participant Planner
    participant DB as Shared Task List
    participant Worker
    Planner->>DB: Decompose Goal & INSERT tasks
    Worker->>DB: SELECT PENDING task FOR UPDATE SKIP LOCKED
    DB-->>Worker: Acquire Task Lock
    Worker->>DB: UPDATE status to IN_PROGRESS
```

**Phase 2: Teammate Mesh APIs (Orchestration)**
Realtime coordination relies on Redis Pub/Sub channels (e.g., `mesh:tasks`) multiplexed through Centrifugo WebSockets, bringing latency under 1ms.

**Phase 3: AutoDream Pipeline (Memory)**
Ephemeral context logs (`agent_session_data`) are processed by the AutoDream worker, which leverages Minimax LLMs to compress and embed the findings into a durable `pgvector` store (`autodream_memories`), granting true Swarm Intelligence over time.

</div>
