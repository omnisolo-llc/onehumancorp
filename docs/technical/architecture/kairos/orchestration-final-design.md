<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Design Doc: KAIROS AI OS Orchestration Implementation
**Author:** Principal Product Architect & KAIROS Orchestrator (L7)

## Overview
This document defines the structural and aesthetic vision for the OHC "Hybrid Agentic OS". KAIROS orchestrates the agent team by decomposing complex feature requests into a shared task list.

## 1. Phase 1: Shared Task List (Decomposition)
The Shared Task List tracks complex feature decomposition into actionable, sequenced tasks.

**Database Schema (PostgreSQL):**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    parent_plan_id TEXT,
    dependencies JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

**Sequence Diagram:**
```mermaid
sequenceDiagram
    participant KAIROS
    participant TaskDB as PostgreSQL (TaskDB)
    participant Implementer
    KAIROS->>TaskDB: INSERT INTO shared_tasks_v4 (status='PENDING')
    Implementer->>TaskDB: SELECT id FROM shared_tasks_v4 WHERE status='PENDING' FOR UPDATE SKIP LOCKED
    TaskDB-->>Implementer: Return task row
    Implementer->>TaskDB: UPDATE shared_tasks_v4 SET status='IN_PROGRESS' WHERE id=?
```

## 2. Phase 2: Orchestration (Teammate Mesh Architecture)
Realtime communication via Centrifuge node integration and transport components like `LocalTeammateMesh`.

- **Cloud-Native Mode:** Uses Redis Pub/Sub to manage highly concurrent distributed queues via `mesh:tasks` and `mesh:coordination` channels.
- **Standalone Mode:** Degrades gracefully to an in-memory channel broadcast to ensure low-latency IPC.

## 3. Phase 3: autoDream (Memory Consolidation Pipeline)
Background workers consolidate `agent_session_data` and optional `OHC_MEMORY_DIR/*.yml` runtime memory files to embeddings stored in PostgreSQL with pgvector, in the `consolidated_memory` table, granting the swarm exact semantic search capabilities.

## 4. Phase 4: Sub-Agent Orchestration Queue
Background worker system (`srcs/server/orchestration/queue/queue.go`) with Redis or SQLite implementations for spawning isolated sub-agents.

**Database Schema (PostgreSQL):**
```sql
CREATE TABLE IF NOT EXISTS sub_agent_queue (
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

## 5. Visual Excellence Mandate
All associated UI components must represent the OHC "Premium Feel".
- Backdrop Filter: `blur(20px) saturate(200%)`
- Background: `rgba(255, 255, 255, 0.03)`
- Typography: `'Outfit', 'Inter', sans-serif`
</div>
