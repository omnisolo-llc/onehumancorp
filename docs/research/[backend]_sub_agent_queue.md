# [Architect] Implement KAIROS Sub-Agent Orchestration Queue

## Problem Statement
The KAIROS orchestrator requires the ability to spawn, manage, and monitor isolated sub-agents executing background tasks. Currently, complex UltraPlans cannot be safely distributed without a robust queueing mechanism. Without this, agents risk runaway compute costs, concurrency collisions, and state corruption.

## Research Report
Tasks often spawn background sub-agents. Integrating the `shared_tasks_decomposition` table with a background Queue is vital. In Cloud-Native mode, utilizing Redis (e.g., BullMQ semantics) and PostgreSQL `FOR UPDATE SKIP LOCKED` handles highly concurrent distributed queues safely. In Standalone Desktop mode, utilizing an internal SQLite table with `sync.Mutex` ensures safe local task dispatch without external dependencies.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

**Sub-Agent Orchestration Queue**

This system ensures high availability, scalability, and robust state management for autonomous agent workloads across both deployment modes.

**Database Schema (PostgreSQL / SQLite Compatible)**
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

**Key Mechanisms:**
- **Isolation:** Each sub-agent is spawned in an isolated environment with a narrow, task-specific context.
- **Queuing:** Supports `attempts`, `max_attempts`, and `backoff` logic to handle intermittent failures.
- **Resource Management:** Strictly enforced VRAM and token quotas per sub-agent to prevent runaway compute costs.
- **Degradation:** Falls back from Redis ZSETs/PostgreSQL distributed locking to SQLite/sync.Mutex in standalone mode.

</div>

## Implementation Prompt
Implement the Sub-Agent Orchestration Queue manager in `srcs/server/orchestration/queue/queue.go`.
1. Scaffold the `sub_agent_queue` schema in PostgreSQL and SQLite.
2. Implement the Queue Manager to poll for `QUEUED` tasks, utilizing `FOR UPDATE SKIP LOCKED` in PostgreSQL and local `sync.Mutex` transactions in SQLite.
3. Build the worker dispatch logic that spawns an isolated sub-agent context, strictly enforcing token and VRAM quotas.
4. Implement retry capabilities including `attempts`, `max_attempts`, and `backoff` logic.
5. Integrate with the Teammate Mesh to broadcast task `status` transitions (e.g., RUNNING -> COMPLETED).
Ensure 100% unit test coverage for the queueing logic and hybrid locking strategies.

## Priority
P0

## Estimated Scope
Large
