<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03) !important; font-family: 'Outfit', 'Inter', sans-serif !important; padding: 24px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); color: #ffffff;">

# Phase 4: KAIROS Orchestration Design

## Sub-Agent Orchestration Queue

Robust background queueing logic to spawn isolated sub-agents, as an extension of the existing KAIROS Orchestrator.

### Architecture
*   Spawns isolated sub-agents to process decomposed tasks asynchronously.
*   Enforces isolation between the parent task execution and the sub-agent's operations.

### Database Schema Definition
```sql
CREATE TABLE IF NOT EXISTS sub_agent_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    parent_task_id UUID NOT NULL,
    payload JSONB,
    status VARCHAR NOT NULL DEFAULT 'QUEUED',
    worker_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### Integration with Existing Triad
*   **Shared Task List**: `sub_agent_jobs.parent_task_id` links directly to a decomposed task in `shared_tasks_decomposition`.
*   **Teammate Mesh**: Job status updates (e.g., QUEUED -> IN_PROGRESS) are broadcast over the `mesh:events:task_updates` channels.
*   **AutoDream**: Sub-agent memory artifacts will eventually be consolidated via AutoDream pipelines.

</div>
