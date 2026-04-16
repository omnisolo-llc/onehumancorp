# KAIROS Orchestrator Research and Epic

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

This document synthesizes the architectural decisions and GitHub issues generated to fulfill the "Shared Task List Decomposition", "Realtime Teammate Mesh APIs", and "AutoDream Data Pipelines" feature requirements for the KAIROS OS.

## 1. Epic Tracked
- **Epic**: [epic] Kairos KAIROS Orchestrator Architecture (#5049)

## 2. Shared Task List Decomposition

### GitHub Issue: [backend] Implement Shared Task List Decomposition for KAIROS (#5050)

**Problem Statement**
The swarm needs a central, distributed tracking system to coordinate efforts and avoid duplicate work when acting upon high-level feature requests.

**Research Report**
The current architecture relies on direct messaging which is fragile during node reboots. A durable, highly-available central shared task list backed by PostgreSQL enables KAIROS to decouple task submission from task execution (queueing).

**Database Schema**
```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    epic_id UUID REFERENCES epics(id),
    title VARCHAR(255) NOT null,
    status VARCHAR(50) NOT null CHECK (status IN ('PENDING', 'CLAIMED', 'DONE', 'FAILED')),
    assigned_agent VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## 3. Realtime Teammate Mesh APIs

### GitHub Issue: [backend] Architect Realtime Teammate Mesh APIs for KAIROS (#5051)

**Problem Statement**
Agents require sub-millisecond coordination to avoid stepping on each other's toes.

**Research Report**
Polling the database is too slow and resource-intensive for immediate state changes. Redis Pub/Sub provides an ideal lightweight transport for our Teammate Mesh, enabling push-based state propagation across the entire swarm.

**Architecture Endpoints**
- **Transport**: Redis Pub/Sub channels
- **API Endpoints**:
  - `POST /mesh/publish`
  - `GET /mesh/subscribe (WebSocket upgrade)`

## 4. AutoDream Data Pipelines for OHC VectorDB

### GitHub Issue: [backend] Implement AutoDream Data Pipelines for OHC VectorDB (#5052)

**Problem Statement**
The OS loses context over time as agent sessions cycle. We need a persistent architectural memory to inform future swarm actions.

**Research Report**
By extracting UltraPlans and closed Tasks, embedding them via LLMs, and indexing them using pgvector, we can provide a semantic search API that acts as OHC's long-term memory (AutoDream).

**Storage Configuration**
```sql
CREATE TABLE knowledge_embeddings (
    id UUID PRIMARY KEY,
    content TEXT,
    embedding VECTOR(1536)
);
CREATE INDEX ON knowledge_embeddings USING ivfflat (embedding vector_cosine_ops);
```

</div>
