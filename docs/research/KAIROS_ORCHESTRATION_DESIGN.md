# 🧠 AutoDream & KAIROS Orchestration Design

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border-radius: 12px; padding: 24px; font-family: 'Outfit', 'Inter', sans-serif;">
## Architectural Vision
This document outlines the consolidation findings for the AutoDream data pipelines, Shared Task List, and Teammate Mesh, enabling autonomous agent workflows within the Hybrid OS.

### 1. Shared Task List (State Machine Tracking)
*   **Cloud Mode**: PostgreSQL with `SKIP LOCKED` for horizontal scalability.
*   **Standalone Mode**: Graceful degradation to local SQLite SIPDB.

### 2. Teammate Mesh (Agent Coordination)
*   Real-time coordination via Redis Pub/Sub on `mesh:tasks`.
*   Enables instant dependency unblocking for sub-agents.

### 3. AutoDream (Memory Consolidation)
*   Consolidates executed missions into Vector DB embeddings.
*   Synchronizes with `autodream_memories` to build persistent agent knowledge.
</div>

### Sequence Diagram: Shared Task List execution
```mermaid
sequenceDiagram
    participant KAIROS
    participant DB as Postgres/SQLite
    participant Mesh as Teammate Mesh
    participant Worker

    KAIROS->>DB: INSERT Task (Status: PENDING)
    KAIROS->>Mesh: PUBLISH "mesh:tasks" "Task Created"
    Worker-->>Mesh: SUBSCRIBE "mesh:tasks"
    Worker->>DB: SELECT Task (SKIP LOCKED)
    Worker->>DB: UPDATE Task (Status: IN_PROGRESS)
    Worker->>Mesh: PUBLISH "mesh:tasks" "Task In Progress"
    Worker->>Worker: Execute
    Worker->>DB: UPDATE Task (Status: DONE)
    Worker->>Mesh: PUBLISH "mesh:tasks" "Task Complete"
```
