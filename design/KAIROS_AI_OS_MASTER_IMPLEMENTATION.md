<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AI OS Master Implementation Plan
**Role:** Principal Product Architect & KAIROS Orchestrator (L7)
**Status:** DRAFT

## 1. Abstract
The OHC Hybrid Architecture (OHC-HA) orchestrates a massive AI agent swarm capable of operating in Cloud-Native Mode, Standalone Desktop Mode, and Thin Client Mode. KAIROS ensures that every high-level user request is decomposed into executable tasks, coordinated with minimal latency, and structurally remembered for future context.

This document details the three pillars of the KAIROS architecture:
1.  **Shared Task List** (The Decomposition Engine)
2.  **Teammate Mesh** (The Real-time Transport)
3.  **autoDream** (The Memory Consolidator)

---

## 2. Shared Task List (Decomposition)
To manage complex feature requests autonomously, KAIROS leverages a durable `shared_tasks` state machine. This state machine dictates the DAG (Directed Acyclic Graph) of operations required for a larger "plan".

### 2.1 Database Schema (PostgreSQL & SQLite)
The `shared_tasks` table is designed to degrade gracefully from a multi-tenant PostgreSQL to a local SQLite instance for Standalone users.

```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING', -- PENDING, IN_PROGRESS, COMPLETED, BLOCKED
    agent_id VARCHAR, -- Nullable until an agent claims it
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    parent_plan_id TEXT,
    dependencies JSONB NOT NULL DEFAULT '[]', -- DAG edges
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### 2.2 Concurrency & Locking Strategy
- **Cloud-Native (PostgreSQL):** Uses `SELECT ... FOR UPDATE SKIP LOCKED` to allow horizontally scaled agent pods to claim work from the `shared_tasks` table without stalling each other.
- **Standalone (SQLite):** Uses application-level mutexes and SQLite `BEGIN EXCLUSIVE` transactions, ensuring safe local execution for a single human operator.

---

## 3. Orchestration: Teammate Mesh Architecture
Agents coordinate state changes and hand-offs via the Teammate Mesh, a low-latency pub/sub layer.

### 3.1 Teammate Mesh APIs & Transport
- **Cloud-Native:** Uses `CentrifugeNode` and Redis Pub/Sub (`rueidis`) for bridging WebSocket connections and streaming server-to-server events.
- **Standalone:** Degrades to an in-memory Go channel broadcast bus.

### 3.2 Key Channels
- `mesh:tasks`: Used to notify the swarm when a task state transitions (e.g., `PENDING` -> `IN_PROGRESS`).
- `mesh:coordination`: General channel for agents to advertise their presence or ask for capabilities.

---

## 4. autoDream Memory Consolidation Pipeline
Agents require a mechanism to write temporary session thoughts into a permanent index, ensuring OHC continuously evolves and doesn't repeat past mistakes. The **autoDream** pipeline fulfills this mandate (OHC-SIP).

### 4.1 Consolidation Flow
1. Agents write episodic context to the `.agent-task/memory/` directory on disk.
2. The autoDream background worker reads these YAML/MD files.
3. The content is chunked, embedded via Cohere/Minimax APIs, and inserted into `consolidated_memory`.

### 4.2 pgvector Schema
```sql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES shared_tasks(id),
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- HNSW Index for rapid approximate nearest neighbor (ANN) search
CREATE INDEX ON consolidated_memory USING hnsw (embedding vector_l2_ops);
```

---

## 5. Architectural Degradation Summary

| Component | Cloud-Native | Standalone Desktop |
| :--- | :--- | :--- |
| **Shared Task Locking** | PostgreSQL `FOR UPDATE SKIP LOCKED` | Local SQLite + Go Mutexes |
| **Teammate Mesh Comm** | Redis Pub/Sub (`rueidis`) + Centrifugo | Go Memory Channel Bus |
| **Vector DB** | pgvector / Pinecone | Local SQLite FTS/Vector |

</div>
