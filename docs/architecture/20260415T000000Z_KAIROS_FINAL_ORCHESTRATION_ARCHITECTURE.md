<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# Master Design Doc: KAIROS Hybrid Agentic OS
**Author:** Principal Product Architect & KAIROS Orchestrator (L7)

## 1. Phase 1: Shared Task List (Decomposition)
The Shared Task List tracks complex feature decomposition into actionable, sequenced `shared_tasks_decomposition`.

**Database Schema (PostgreSQL):**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    locked_until TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

**Sequence Diagram:**
```mermaid
sequenceDiagram
    participant KAIROS
    participant TaskDB as PostgreSQL (shared_tasks_decomposition)
    participant Implementer
    KAIROS->>TaskDB: INSERT INTO shared_tasks_decomposition (status='PENDING')
    Implementer->>TaskDB: SELECT id FROM shared_tasks_decomposition WHERE status='PENDING' FOR UPDATE SKIP LOCKED
    TaskDB-->>Implementer: Return task row
    Implementer->>TaskDB: UPDATE shared_tasks_decomposition SET status='IN_PROGRESS' WHERE id=?
```

## 2. Phase 2: Orchestration (Teammate Mesh Architecture)
Realtime communication via transport components utilizing the Teammate Mesh APIs. Realtime API endpoint `/api/mesh/broadcast`.

## 3. Phase 3: autoDream (Memory Consolidation Pipeline)
Background workers consolidate runtime memory to embeddings stored in PostgreSQL with pgvector, in the `autodream_memories` table.

## 4. Phase 4: Sub-Agent Orchestration Queue
Tasks spawn background sub-agents via integration with the `sub_agent_jobs` queue in SQLite/Postgres.

</div>
