---
title: "KAIROS Orchestrator: Core Architecture Implementation"
status: PENDING
assignee: ""
priority: P0
estimated_scope: Large
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# 🚀 Mission: KAIROS Orchestrator

## 1. Problem Statement
The OHC swarm requires a highly resilient, observable, and autonomous coordination engine to scale. Currently, task delegation can be brittle, lacking a unified distributed state machine for dependency tracking. Moreover, long-term memory (AutoDream) needs robust, batch-oriented pipelines for Hybrid Architecture (Cloud pgvector + Standalone SQLite). We need an Implementer agent to bridge the gap between our high-level architecture designs and the concrete database schemas, queue interfaces, and API contracts.

## 2. Research Report
Our analysis of the KAIROS engine reveals three critical pillars needing immediate implementation:
- **Shared Task List & State Machine:** Agents currently coordinate ad-hoc. A robust Distributed State Machine (`docs/features/kairos/state_machine.md`) backed by `shared_tasks` and `state_machine_transitions` is required.
- **Sub-Agent Orchestration Queue:** High-concurrency task delegation needs a resilient queue. We must implement `sub_agent_jobs` (`docs/features/kairos/sub_agent_queue.md`).
- **AutoDream Pipeline:** Ephemeral session data must be consolidated into `autodream_memories` for long-term RAG search (`docs/features/kairos/autodream_pipeline.md`).

### Competitive Analysis
| Feature | Legacy Systems | OHC KAIROS |
| :--- | :--- | :--- |
| **State Tracking** | Local Only | Distributed (Redis + Postgres) / SQLite Fallback |
| **Memory** | Ephemeral | Persistent Vector DB (pgvector) |
| **Coordination** | HTTP Polling | Teammate Mesh (Pub/Sub) |

## 3. Design Doc

### 3.1 Architecture Sequence Diagrams

#### Shared Task List & Distributed State Machine
```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant DB as OHC-SIP Database
    participant M as Teammate Mesh
    participant A as Agent Worker

    O->>DB: INSERT shared_tasks (PENDING)
    DB-->>O: Task ID
    O->>M: Publish Task Created Event
    A->>M: Listen for Task Events
    A->>DB: Claim Task (UPDATE status = ASSIGNED, FOR UPDATE)
    DB->>DB: INSERT state_machine_transitions (PENDING -> ASSIGNED)
    DB-->>A: Claim Success
    A->>M: Publish Task Claimed Event
```

#### Teammate Mesh APIs (Centrifuge Integration)
```mermaid
sequenceDiagram
    participant A1 as Agent 1
    participant C as Centrifuge Hub
    participant A2 as Agent 2

    A1->>C: Publish Message (channel: mission_123)
    C->>A2: Route Message via WebSocket/gRPC
    A2->>C: Acknowledge Receipt
```

#### AutoDream Data Pipeline
```mermaid
sequenceDiagram
    participant W as AutoDream Worker
    participant E as Minimax/LLM API
    participant DB as Vector DB (pgvector/SQLite)

    W->>W: Periodic Sweep (LIMIT 500)
    W->>E: Generate Embeddings
    E-->>W: []float32 Vector
    W->>DB: UPSERT autodream_memories
```

## 4. Implementation Prompt
**Attention Implementer Agent:** You are tasked with bringing the KAIROS Orchestrator schemas to life.

1. **Database Schema:** Create a new migration file `srcs/server/db/migrations/032_kairos_orchestrator_schema.sql` (verify sequence number). Include the schemas for:
   - `shared_tasks`
   - `task_dependencies`
   - `state_machine_transitions`
   - `sub_agent_jobs`
   - `autodream_memories`
   Ensure strict adherence to Hybrid Compatibility (Postgres + SQLite). Do not use `IF NOT EXISTS` on `ALTER TABLE ADD COLUMN`. Use `goose` migration formatting (`-- +goose Up` / `-- +goose Down`).
2. **Bazel Wiring:** Update `srcs/server/db/BUILD.bazel` to include your new migration file in the `embedsrcs` list.
3. **Verification:** Write a Go test (e.g., `srcs/server/orchestration/kairos_schema_test.go`) utilizing `db.NewSqliteProvider(sqliteDB)` to verify the schema initializes correctly without syntax errors.

**Priority:** P0
**Estimated Scope:** Large

</div>
