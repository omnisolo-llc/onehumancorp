# OHC KAIROS: Hybrid AI OS Vision

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px;">

## 1. Overview
The OHC Hybrid Agentic OS empowers a single human to orchestrate a vast swarm of AI agents. The **KAIROS Orchestration** layer acts as the autonomous backbone, dynamically decomposing high-level tasks, tracking state dependencies, coordinating via real-time meshes, and consolidating long-term memories.

## 2. The KAIROS Triad
KAIROS guarantees "Absolute Autonomy" and "Swarm Intelligence" through three pillars:
1. **Shared Task List (The Brain):** A durable state machine living in PostgreSQL (`FOR UPDATE SKIP LOCKED`) or SQLite.
2. **Teammate Mesh (The Nerves):** Low-latency WebSockets / Redis Pub/Sub channels for peer-to-peer coordination.
3. **AutoDream (The Memory):** A background pipeline that vectorizes session logs into a `pgvector` store for semantic search.

## 3. Architecture Visualization
```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh (Redis/Centrifugo)
        M[Mesh Hub]
    end

    subgraph KAIROS Orchestrator
        T[(Shared Task List)]
        AD[AutoDream Pipeline]
        V[(pgvector Memories)]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    A1 -->|Claim Task| T
    A2 -->|Claim Task| T

    T -.->|Completions| AD
    AD -->|Embed| V
    A1 -->|Semantic Search| V
```

## 4. Phase 1: Shared Task List Database Schema
**PostgreSQL Mode:**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    parent_plan_id TEXT,
    locked_until TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);
```

## 5. Phase 2: Teammate Mesh API Event Bus
Agents broadcast via channels like `mesh:tasks` to notify siblings:
```json
{
  "event_type": "TASK_CLAIMED",
  "agent_id": "Implementer-1",
  "payload": {
    "task_id": "123e4567-e89b-12d3-a456-426614174000",
    "timestamp": "2026-04-12T16:00:00Z"
  }
}
```

## 6. Phase 3: AutoDream Pipeline
Ephemeral `agent_session_data` and `.agent-task/memory/*.yml` logs are chunked, summarized using a Minimax LLM call, and indexed into `pgvector` to ensure zero context loss.

```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    agent_id VARCHAR,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX ON autodream_memories USING hnsw (embedding vector_l2_ops);
```

## Visual Excellence Mandate
This architectural view strictly enforces the OHC Aesthetic Core. Any downstream implementation MUST inject:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
</div>