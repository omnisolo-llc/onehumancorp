<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AI OS: Phase 4 Premium Design Doc - Sub-Agent Orchestration Queue

## 1. Overview
The Phase 4 Sub-Agent Orchestration Queue is a critical background worker system that enables the OHC Hybrid AI OS to spawn, manage, and monitor isolated sub-agents executing background tasks. This ensures high availability, vertical/horizontal scalability, and robust state management for autonomous agent workloads.

## 2. Architecture & Hybrid Strategy
The queue utilizes a state-machine driven backend to coordinate jobs.

- **Queue Manager (`src/server/orchestration/queue/queue.rs`)**: Manages the ingestion, polling, and dispatch of sub-agent jobs.
- **Worker Nodes**: Isolated sub-agents spawned to execute specific payloads.

### Hybrid Strategy
- **Cloud-Native Mode**: Leverages Redis (e.g., BullMQ style semantics via `redis`) to manage highly concurrent distributed queues. PostgreSQL `FOR UPDATE SKIP LOCKED` ensures robust task claiming across Kubernetes pods without race conditions.
- **Standalone Mode**: Gracefully degrades to an in-memory or SQLite-backed transaction system, utilizing application-level `sync.Mutex` locks to ensure safe local task dispatch.

## 3. Database Schema
PostgreSQL/SQLite Compatible Schema for Sub-Agent Queue:

```sql
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    parent_task_id UUID NOT NULL,
    payload JSONB,
    status VARCHAR NOT NULL DEFAULT 'QUEUED', -- QUEUED, RUNNING, COMPLETED, FAILED
    worker_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## 4. Sub-Agent Task Queue Payload
When KAIROS decomposes a mission, it submits jobs to a distributed background queue.

```json
{
  "job_id": "worker-task-77",
  "queue_name": "l5-implementers",
  "data": {
    "issue_ref": "GitHub issue created from the repository task template",
    "repository_state_hash": "sha256-abc123def456",
    "execution_timeout_ms": 3600000
  }
}
```

## 5. Workflows
1. **Enqueue:** The KAIROS Orchestrator or a primary agent pushes a sub-task into `sub_agent_queue`.
2. **Poll:** The Queue Manager polls for `status='QUEUED'` tasks, locking the row.
3. **Dispatch:** A new isolated sub-agent context is spawned, and `status` becomes `RUNNING`.
4. **Completion:** Upon success/failure, the sub-agent signals the mesh, and `status` updates to `COMPLETED` or `FAILED`.

### Workflow Diagram
```mermaid
graph TD
    subgraph KAIROS Orchestrator
        A[Task Manager] -->|Enqueue| Q{Sub-Agent Queue Interface}
    end

    Q -->|Cloud| Redis[(Redis ZSETs)]
    Q -->|Standalone| DB[(SQLite Mutexed Table)]

    Redis -->|Dequeue| W1[Worker Pod]
    DB -->|Dequeue| W2[Local Worker]

    W1 -->|Transition Event| M[Teammate Mesh / Centrifuge]
    W2 -->|Transition Event| M

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,Q,Redis,DB,W1,W2,M premium;
```
</div>
