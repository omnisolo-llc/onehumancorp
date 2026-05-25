<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# KAIROS Orchestration

**1. KAIROS Shared Task List (Phase 1)**
*   **Purpose:** The central queue for distributing tasks dynamically among Agents.
*   **Database Schema:**
    ```sql
    CREATE TABLE IF NOT EXISTS shared_tasks (
        id TEXT PRIMARY KEY,
        organization_id TEXT NOT NULL,
        parent_plan_id TEXT,
        title TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'PENDING',
        assigned_agent_id TEXT,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );
    ```
*   **Sequence Diagram:**
    ```mermaid
    sequenceDiagram
        participant WorkerAgent
        participant PostgresDB
        participant CentrifugeMesh

        WorkerAgent->>PostgresDB: SELECT * FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED
        PostgresDB-->>WorkerAgent: Returns Task Data
        WorkerAgent->>PostgresDB: UPDATE shared_tasks SET status = 'IN_PROGRESS'
        WorkerAgent->>CentrifugeMesh: Broadcast MeshEvent {topic: 'task.assigned'}
    ```

**2. KAIROS Teammate Mesh (Phase 2)**
*   **Purpose:** Fast, real-time publish/subscribe communication between Agents.
*   **Protocol (Proto RPCs):**
    ```protobuf
    message MeshEvent {
        string event_id = 1;
        string topic = 2;
        bytes payload = 3;
        int64 timestamp = 4;
    }
    rpc StreamMeshEvents(EventStreamRequest) returns (stream MeshEvent);
    ```

**3. KAIROS AutoDream Pipelines (Phase 3)**
*   **Purpose:** Background daemon that converts ephemeral tasks into semantic memory.
*   **Data Pipeline Flow:**
    1.  `AutoDreamWorker` queries `shared_tasks` where `status = 'COMPLETED'`.
    2.  Invokes `MinimaxClient` LLM to generate `[]float32` embeddings.
    3.  Upserts memory vector into Postgres (`VECTOR(1536)`).


</div>
