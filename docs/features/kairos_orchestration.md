<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>

# Master Design Doc - KAIROS AI OS Orchestration

## Overview
This document consolidates the architecture for the KAIROS Triad: Shared Task List, Teammate Mesh, and AutoDream. It serves as the single source of truth for the OHC Hybrid Agentic OS task and memory orchestration.

**1. KAIROS Shared Task List (Phase 1)**
*   **Purpose:** The central queue for distributing tasks dynamically among Agents. Ensures reliable execution and handles task dependencies using database locks.
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
*   **Purpose:** Fast, real-time publish/subscribe communication between Agents for events and coordination.
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
*   **Architecture Diagram:**
    ```mermaid
    graph TD
        AgentA[Agent A] -->|Publish| TeammateMesh[Teammate Mesh / Redis / Centrifuge]
        AgentB[Agent B] -->|Subscribe| TeammateMesh
        TeammateMesh -->|Broadcast| AgentB
    ```

**3. KAIROS AutoDream Pipelines (Phase 3)**
*   **Purpose:** Background daemon that converts ephemeral tasks into semantic memory, allowing the Swarm to continuously learn.
*   **Data Pipeline Flow:**
    1.  `AutoDreamWorker` queries `shared_tasks` where `status = 'COMPLETED'`.
    2.  Invokes `MinimaxClient` LLM to generate `[]float32` embeddings from task payload and deliberation logs.
    3.  Upserts memory vector into Postgres (`VECTOR(1536)`) to `autodream_memories`.

## Follow-up Tasks (Phase 4)
*   [ ] **Telemetry & Tracing:** Integrate OpenTelemetry spans to trace task lifecycles from creation in `shared_tasks` to embedding in `autodream_memories`.
*   [ ] **Hybrid Deployment Sync:** Implement state synchronization mechanisms to reliably replicate `shared_tasks` between cloud (Postgres) and standalone desktop (SQLite) environments seamlessly.
*   [ ] **Batch AutoDream Processing:** Update `AutoDreamWorker` to process completed tasks in configurable batches rather than querying individually, reducing database load and rate-limiting LLM APIs.
*   [ ] **Dependency Graph Visualization:** Add an administrative UI that pulls `parent_plan_id` connections to visualize agentic workflows and debug stuck tasks.
