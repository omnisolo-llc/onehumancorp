# KAIROS AI OS Architecture

## Overview
This document details the design of the three core KAIROS features: Shared Task List, Teammate Mesh, and AutoDream Memory Consolidation. This outlines the complete architecture for the Swarm OS.

## Phase 1: Shared Task List
**Goal:** Distributed task tracking.
**Design:** Uses Postgres `FOR UPDATE SKIP LOCKED` for Cloud-Native multi-tenancy, and application-level mutexes for local SQLite standalone mode.

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

## Phase 2: Teammate Mesh APIs
**Goal:** Realtime sub-agent coordination and state broadcasting.
**Design:**
- **Cloud-Native Mode:** Uses `rueidis` Redis Pub/Sub for cross-pod coordination.
- **Standalone Mode:** Uses in-memory Go channels for efficient local coordination.

## Phase 3: AutoDream Memory Consolidation
**Goal:** Summarize episodic execution logs into long-term vector memory.
**Design:** A background daemon `AutoDreamDaemon` processes completed tasks, uses OpenAI embeddings to convert summaries to vectors, and persists to `agent_memories` using `pgvector`.
