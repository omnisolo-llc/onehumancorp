<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Master Design Doc: KAIROS AI OS Orchestration

This document serves as the final premium design doc synthesizing the OHC Hybrid AI OS Orchestration layer.

## 1. The Shared Task List (The Brain)
The Shared Task List handles task decomposition into a DAG (Directed Acyclic Graph) and avoids worker collisions.

### Database Schema
**Table:** `shared_tasks`
- `id` (UUID, Primary Key)
- `organization_id` (VARCHAR)
- `parent_plan_id` (TEXT)
- `title` (VARCHAR)
- `description` (TEXT)
- `status` (VARCHAR): 'PENDING', 'IN_PROGRESS', 'COMPLETED'
- `agent_id` (VARCHAR, Nullable)
- `payload` (JSONB)
- `dependencies` (JSONB)
- `locked_until` (TIMESTAMP)

**Degradation Strategy:**
- **Cloud-Native (PostgreSQL):** Uses `SELECT ... FOR UPDATE SKIP LOCKED` to allow highly concurrent, pod-level orchestration.
- **Standalone (SQLite):** Degrades to application-level `sync.Mutex` and basic `UPDATE` transactions.

### Sequence Diagram
```mermaid
sequenceDiagram
    participant ArchitectAgent as KAIROS Orchestrator (L7)
    participant DB as Postgres/SQLite (Shared Task List)
    participant WorkerAgent as Worker Agent
    participant Hub as Teammate Mesh Gateway

    ArchitectAgent->>DB: Breakdown Feature X into Tasks (State: PENDING)
    ArchitectAgent->>DB: INSERT shared_tasks

    loop Worker Polling Cycle
        WorkerAgent->>DB: BEGIN TRANSACTION
        WorkerAgent->>DB: SELECT id FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED LIMIT 1
        alt Task Acquired
            DB-->>WorkerAgent: Lock granted (Task A)
            WorkerAgent->>DB: UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = 'worker-uuid'
            WorkerAgent->>DB: COMMIT
            WorkerAgent->>Hub: Publish MeshEvent (TaskTransition -> IN_PROGRESS)
            WorkerAgent->>WorkerAgent: Execute work
            WorkerAgent->>DB: UPDATE shared_tasks SET status = 'COMPLETED'
            WorkerAgent->>Hub: Publish MeshEvent (TaskTransition -> COMPLETED)
        else No Task Available / Locked
            DB-->>WorkerAgent: Returns 0 rows
            WorkerAgent->>DB: ROLLBACK
        end
    end
```

## 2. Teammate Mesh (The Nerves)
The Teammate Mesh facilitates real-time coordination without delays.

### API Contracts
**Channels:** `mesh:tasks`, `mesh:coordination`, `mesh:capabilities`

**Degradation Strategy:**
- **Cloud-Native:** Uses `rueidis` (Redis Pub/Sub) and `CentrifugeNode` for broad network event distribution.
- **Standalone:** Uses Go channels for fast, local IPC.

## 3. autoDream (The Memory)
The autoDream pipeline consolidates ephemeral task data into long-term vector embeddings.

### Pipeline Architecture
1. **Extract:** Sweep `COMPLETED` tasks from `shared_tasks`.
2. **Synthesize:** Compress task logs using Minimax/LLM.
3. **Embed:** Upsert to `autodream_memories`.

### Database Schema
**Table:** `autodream_memories`
- `id` (TEXT, Primary Key)
- `organization_id` (VARCHAR)
- `task_id` (TEXT)
- `content` (TEXT)
- `embedding` (VECTOR(1536))
- `source_type` (TEXT)
- `created_at` (TIMESTAMPTZ)

**Degradation Strategy:**
- **Cloud-Native:** `pgvector` with HNSW index for `vector_cosine_ops`.
- **Standalone:** Embeddings serialized as BLOBs; cosine similarity performed in-memory via application logic.

</div>
