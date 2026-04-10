# KAIROS Design Doc

## 1. Shared Task List Database Schema (PostgreSQL)

```sql
CREATE TABLE IF NOT EXISTS swarm_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mission_id TEXT NOT NULL,
    parent_plan_id TEXT, -- Facilitates Sub-Agent Orchestration
    dependencies JSONB NOT NULL DEFAULT '[]', -- DAG Sequence enforcement
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    payload JSONB,
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

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
```

## 2. Teammate Mesh APIs

The Teammate Mesh provides realtime Pub/Sub capabilities backed by Redis for cloud and Memory for standalone.

Transport Interface:
- `MeshTransport` interface implemented by `RedisMeshTransport` and `MemoryMeshTransport`.

gRPC Contracts:
- `AdvertiseCapabilities(AgentCapabilities)`
- `DiscoverAgents(Query)`
- `StreamMeshEvents(EventStreamRequest)`

## 3. AutoDream Data Pipeline Architecture

Data Sources: `agent_session_data` and `.agent-task/memory/{timestamp}.yml`.
Pipeline: `AutoDreamPipeline` orchestrator orchestrates chunking and compression via LLM.

Vector Storage Schema (pgvector):
The vector data is persisted to the `swarm_memory` table (defined in `005_sip.sql`).
