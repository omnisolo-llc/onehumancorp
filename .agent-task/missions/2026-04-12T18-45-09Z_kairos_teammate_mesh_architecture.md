---
status: DONE
agent: Jules
priority: P0
scope: Large
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# 🗺️ Guide: Architect and Implement the KAIROS Teammate Mesh and Shared Task List

## Problem Statement
The OHC swarm currently lacks a centralized, distributed system for agents to securely coordinate, decompose, and track task execution across the hybrid architecture. Without a "Shared Task List" leveraging our Postgres backend and Redis Pub/Sub (Teammate Mesh), agents cannot effectively orchestrate complex, multi-step workflows.

## Research Report
- Current agents operate largely in isolation, missing a Realtime Teammate Mesh APIs layer.
- Cloud-Native Mode needs Redis Pub/Sub channels for coordination.
- AutoDream requires architectural consolidation pipelines for memory.
- There is no central distributed state machine tracking the Teammate Mesh dependencies.

## Design Doc
### 1. Database Schema (`state_machine_transitions` and `shared_tasks`)
Implement the database schema for the distributed state machine and shared task lists.
```sql
CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT,
    reason TEXT,
    occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_sm_entity ON state_machine_transitions(entity_id, entity_type);

CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    assigned_to TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

### 2. Teammate Mesh API Layer
Design Go handlers `srcs/server/orchestration/teammate_mesh.go` to handle agent inter-communication via WebSockets and Redis Pub/Sub.

### 3. AutoDream Data Pipeline
Implement `srcs/server/orchestration/autodream.go` to process episodic memory into long-term embedded truth.

## Implementation Prompt
Implementer:
1. Create the `state_machine_transitions` and `shared_tasks` database migrations.
2. Implement the Go interfaces and concrete logic for `TeammateMesh` utilizing `rueidis` for Redis operations in Cloud Mode.
3. Expose these endpoints in the `Server` or `OrchestrationHub`.
4. Implement the `AutoDream` processing logic.
5. Provide test coverage >90% for the orchestration components.
6. Remember to apply the "OHC Premium Feel" (Glassmorphism, Outfit/Inter fonts) to any UI documentation or outputs.

</div>
