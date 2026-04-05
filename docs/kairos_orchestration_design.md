<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Doc: KAIROS Orchestration
**Author:** Antigravity, Principal Product Architect & Visionary (L7)
**Status:** Approved
**Last Updated:** 2026-04-05

## 1. Overview
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. **KAIROS Orchestration** is the core blueprint that unites our "Shared Task List", "Teammate Mesh APIs", and "AutoDream Vector Data Pipelines" into a seamless Hybrid Agentic OS.

## 2. Core Components

### 2.1 Shared Task List
A distributed state machine for tracking and orchestrating asynchronous tasks across the swarm. KAIROS utilizes `swarm_tasks` for mission-critical steps and `shared_tasks` for inter-agent delegation.
*   **PostgreSQL Native (Cloud)**: Relies on `FOR UPDATE SKIP LOCKED` for lock-free concurrency and zero TOCTOU (Time-Of-Check to Time-Of-Use) race conditions across parallel K8s agent pods.
*   **SQLite Fallback (Standalone)**: Degrades gracefully to single-node concurrency utilizing a two-step select-then-update approach within explicit transactions (`tx.Begin()`) because SQLite lacks `SKIP LOCKED`.
*   **DAG Dependencies**: Enforces sequence and parallel task unblocking (e.g., frontend tasks block on backend completion) by utilizing a `task_dependencies` join table.
*   **Isolation**: Ensures multi-tenant isolation by enforcing `organization_id` on all task claims and updates.

### 2.2 Teammate Mesh APIs
A high-throughput realtime event bus that allows agents to broadcast intent, coordinate memory, and perform lock arbitration without polling the database continuously.
*   **Cloud Architecture**: Agents publish to production Redis Pub/Sub channels (e.g., `mesh:tasks`, `mesh:coordination`). Centrifuge handles downstream WebSocket propagation to the human CEO dashboard.
*   **Standalone Architecture**: Fallbacks to in-memory Go channels (e.g., `LocalTeammateMesh`), guaranteeing the OS never fails simply because a heavy dependency (Redis) is offline.
*   **Zero Secrets**: Relies entirely on SPIFFE/SPIRE Workload APIs to establish mTLS mesh identities. System endpoints wrapped in `auth.RequireRole("system", ...)`.
*   **Message Schema**: Adheres to the OHC-SIP Protocol (JSON payloads with `agent_id`, `action`, `status` at the root).

### 2.3 AutoDream Vector Data Pipelines
A semantic memory consolidation pipeline running passively to translate ephemeral session contexts into durable, vectorized truth.
*   **Pipeline Logic**: Background workers monitor `agent_session_data` and trigger Minimax/LLM summarization jobs (`AutoDreamWorker`), transforming short-term token buffers into high-dimensional `pgvector` records in `autodream_memories` and `swarm_truth_embeddings`.
*   **Cloud Mode**: Uses `pgvector` for exact Nearest Neighbor search (`ORDER BY embedding <-> $1`).
*   **Local Degradation**: In SQLite, falls back to recency-based full-text extraction (`ORDER BY created_at DESC`).
*   **AutoDreamWorker lock handling**: Uses `FOR UPDATE SKIP LOCKED` for PostgreSQL to handle row locks safely, but conditionally omits it when running in SQLite mode (`w.pool.IsSQLite()`).

## 3. Sequence Flow (Master Loop)

```mermaid
sequenceDiagram
    participant CEO as CEO Dashboard
    participant API as OHC Server
    participant DB as Shared Task List (PG/SQLite)
    participant TM as Teammate Mesh (Redis/Mem)
    participant Agent as Autonomous Agent
    participant Dream as AutoDream Worker

    CEO->>API: Decompose Mission "Build Feature X"
    API->>DB: Insert DAG Tasks into swarm_tasks
    API->>TM: Broadcast "TASK_SPAWNED"
    TM-->>Agent: Mesh Event Received
    Agent->>DB: Attempt `FOR UPDATE SKIP LOCKED` Claim
    DB-->>Agent: Task Assigned
    Agent->>API: Execute & Report Success
    API->>TM: Broadcast "TASK_COMPLETED"
    Agent->>Dream: Ephemeral Context Shift
    Dream->>DB: Compress Context via LLM into pgvector
```

## 4. DB Schema References

**Task Tracking (`shared_tasks` and `task_dependencies`)**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    locked_until TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id),
    FOREIGN KEY (task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (depends_on_task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_task_deps_task ON task_dependencies(task_id);
CREATE INDEX IF NOT EXISTS idx_task_deps_depends ON task_dependencies(depends_on_task_id);
```

**Memory Consolidation (`autodream_memories`)**
```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_mission_id TEXT
);
```

## 5. Visual Excellence
This architecture must not be exposed as dry infrastructure. The frontend dashboard tracking KAIROS metrics will deploy the OHC "Premium Feel":
*   `backdrop-filter: blur(20px) saturate(200%)`
*   Fluid, ghostly data layer representations for Redis pub/sub streams.

</div>
