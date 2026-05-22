<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Design Doc: Sub-Agent Orchestration Queue
**Author:** Principal Product Architect & KAIROS Orchestrator (L7)

## 1. Overview
The **Sub-Agent Orchestration Queue** is a critical component of the OHC Hybrid AI OS, enabling the system to spawn, manage, and monitor isolated sub-agents executing background tasks. This system ensures high availability, scalability, and robust state management for autonomous agent workloads.

## 2. Architecture
The queue utilizes a state-machine driven backend to coordinate jobs.

### Component Design
- **Queue Manager (`src/server/queue.rs` and `src/server/orchestration/queue/`)**: Manages the ingestion, polling, and dispatch of sub-agent jobs.
- **Worker Nodes**: Isolated sub-agents spawned to execute specific payloads.

### Hybrid Strategy
- **Cloud-Native Mode**: Leverages Redis (e.g., BullMQ style semantics via `redis`) to manage highly concurrent distributed queues. PostgreSQL `FOR UPDATE SKIP LOCKED` ensures robust task claiming across Kubernetes pods without race conditions.
- **Standalone Mode**: Gracefully degrades to an in-memory or SQLite-backed transaction system, using Rust synchronization primitives to ensure safe local task dispatch.

## 3. Database Schema (PostgreSQL/SQLite Compatible)
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

## 4. Workflows
1. **Enqueue:** The KAIROS Orchestrator or a primary agent pushes a sub-task into `sub_agent_queue`.
2. **Poll:** The Queue Manager polls for `status='QUEUED'` tasks, locking the row.
3. **Dispatch:** A new isolated sub-agent context is spawned, and `status` becomes `RUNNING`.
4. **Completion:** Upon success/failure, the sub-agent signals the mesh, and `status` updates to `COMPLETED` or `FAILED`.

</div>
