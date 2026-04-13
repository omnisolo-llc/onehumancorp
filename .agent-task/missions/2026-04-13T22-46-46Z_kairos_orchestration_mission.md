<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# Shared Task List and Teammate Mesh

## Problem Statement
The KAIROS Orchestration logic requires formal database structures to track tasks decomposed from complex features and an event mesh to coordinate agent execution across Cloud and Standalone modes.

## Research Report
Based on `CLAUDE_OHC.md` and `KAIROS_AI_OS_MASTER_PLAN.md` (and existing duplicate missions), the Swarm Intelligence Protocol dictates that KAIROS must orchestrate a shared task list mapping high-level instructions to granular tasks. Coordination occurs over a Realtime Teammate Mesh utilizing Redis Pub/Sub channels (`mesh:tasks` and `mesh:coordination`) in Cloud mode and Local Go channels in Standalone Mode. Long term memory requires a `consolidated_memory` table utilizing pgvector for the "AutoDream" functionality.

## Design Doc

### 1. Database Schema
A new shared task list schema in `srcs/server/db/migrations/`:
```sql
CREATE TABLE IF NOT EXISTS shared_tasks_v3 (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

### 2. Teammate Mesh Orchestration (`srcs/server/orchestration/mesh/pubsub.go`)
Provide an interface to abstract the transport components for the Realtime Teammate Mesh APIs.
```go
type MeshClient interface {
    Publish(channel string, payload []byte) error
    Subscribe(channel string, handler func([]byte)) error
}
```
Channels: `mesh:tasks`, `mesh:coordination`.

### 3. State Machine & Queues
A Distributed State Machine tracking the agent's mesh responses and a scalable Sub-Agent Orchestration queue to trigger background workers.

### 4. AutoDream Vector Pipeline
Background worker system storing vectors in `consolidated_memory`:
```sql
CREATE TABLE consolidated_memory (
    id UUID PRIMARY KEY,
    content TEXT,
    embedding vector(1536)
);
```

## Implementation Prompt
Implement the schema migration file for `shared_tasks_v3` and `consolidated_memory` to fulfill the UltraPlan Deliberation. Build the generic `MeshClient` interface to handle the Sub-Agent Queue Pub/Sub over `mesh:tasks` and `mesh:coordination`. Architect the distributed state machine that updates `shared_tasks_v3` status based on mesh events. Ensure the backend database designs degrade gracefully between PostgreSQL and SQLite.

## Priority
P0

## Estimated Scope
Large

</div>
