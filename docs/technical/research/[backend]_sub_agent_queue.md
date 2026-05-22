# [Architect] Implement Sub-Agent Orchestration Queue for KAIROS

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

## Problem Statement
The KAIROS Orchestration Backend requires a robust mechanism to manage and orchestrate isolated sub-agents spawned from shared tasks. Currently, without a dedicated Sub-Agent Orchestration Queue, the system risks resource exhaustion, lacks mechanisms for exponential backoff or retry logic on failures, and cannot enforce strict quotas (e.g., VRAM, token limits) per sub-agent, which is critical for cloud-native scalability and cost control.

## Research Report
Based on the `docs/architecture/KAIROS_ORCHESTRATION_IMPLEMENTATION_BLUEPRINT.md`, a background queue manager (similar to BullMQ or Celery) must integrate with the shared task list (`shared_tasks_decomposition` table). It must provide an isolation strategy where each sub-agent executes in a task-specific context, while enforcing strict compute and token quotas. For hybrid architecture compliance, this queue degrades from Redis ZSETs (in Cloud-Native mode) to a local `sub_agent_queue` table with locking (in Standalone mode).

## Design Doc

**Sub-Agent Orchestration Queue Architecture**

To support hybrid deployments, the Sub-Agent Queue uses a database-backed state machine.

**Database Schema (`sub_agent_queue`)**
```sql
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_task_id TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    worker_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

**Resource Management & Isolation**
1.  **Quotas**: The worker daemon polling the queue must validate the available tenant quotas (VRAM/Tokens) against limits before transitioning a job from `QUEUED` to `IN_PROGRESS`.
2.  **Backoff & Retries**: Failed sub-agent tasks will utilize an exponential backoff formula (e.g., `2^attempt * 1s`) before being re-queued.
3.  **Isolation Context**: Sub-agents receive a narrowed context scoped only to their `parent_task_id`, avoiding full repository context bleed.

</div>

## Implementation Prompt
As an Implementer agent, you are tasked with implementing the KAIROS Sub-Agent Orchestration Queue Manager.
1. Expand the existing `src/server/orchestration/queue/queue_manager.rs` to strictly enforce VRAM and token quotas per sub-agent before dequeuing.
2. Implement exponential backoff retry logic for failed jobs.
3. Ensure the queue manager gracefully falls back to the `sub_agent_queue` SQLite schema for standalone environments and uses Redis (ZSET-backed scheduling) for cloud environments.
4. Unit test coverage MUST be 100%, verifying the correct execution of quota limit enforcement and exponential backoff.

## Priority
P0

## Estimated Scope
Large
