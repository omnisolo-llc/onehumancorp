# KAIROS AI OS Features Implementation Design

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

## Overview
This document synthesizes the architectural decisions and design requirements for the OHC Hybrid Agentic OS, specifically fulfilling the mandates for Task Decomposition, UltraPlan Deliberation, State Machine Tracking, Sub-Agent Orchestration, and Teammate Mesh Architecture.

The architecture supports both Cloud-Native (Multi-tenant, PostgreSQL/Redis) and Standalone (Local single-user, SQLite) modes gracefully.

## 1. Task Decomposition & Shared Task List
The central hub for coordinating the swarm is the Shared Task List.

### Database Schema (`shared_tasks`)
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
    dependencies JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
Task claiming utilizes `FOR UPDATE SKIP LOCKED` on PostgreSQL and a local lock simulation (`sync.Mutex` with local transactions) on SQLite to prevent race conditions.

## 2. UltraPlan Deliberation & State Machine Tracking
A dedicated state machine tracks deep-deliberation cycles and complex epic-level feature dependencies.

### Database Schema (`ultraplan_state`)
```sql
CREATE TABLE IF NOT EXISTS ultraplan_state (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    epic_id UUID REFERENCES shared_tasks(id),
    deliberation_status VARCHAR NOT NULL DEFAULT 'PENDING',
    deliberation_history JSONB NOT NULL DEFAULT '[]',
    locked_by VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
The state transition flow (PENDING -> DELIBERATING -> CLAIMED -> DONE -> FAILED) safely orchestrates multi-step processes across the network.

## 3. Sub-Agent Orchestration Queue
A background queuing logic system isolates sub-agents during complex workflows.

### Database Schema (`sub_agent_queue`)
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
- **Quotas**: Limits on VRAM/Tokens are enforced before dequeueing.
- **Retries**: Implements exponential backoff (`2^attempt * 1s`) for failures.
- **Storage**: Uses Redis ZSETs for Cloud-Native and SQLite tables for Standalone environments.

## 4. Teammate Mesh Architecture
Agents require sub-millisecond coordination. The Teammate Mesh is a realtime Pub/Sub framework.

**Transport Methods:**
- **Cloud-Native**: Redis Pub/Sub channels (with WebSocket hubs for clients).
- **Standalone**: In-memory Go channel broker.

**Primary Endpoints:**
- `POST /api/mesh/v2/broadcast`
- `GET /api/mesh/v2/subscribe` (WebSocket Upgrade)

## 5. AutoDream Data Pipelines (Long-Term Context)
The Swarm Intelligence Protocol (OHC-SIP) stores architectural memory over time via vector representations.

### Database Schema (`consolidated_memory`)
```sql
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES shared_tasks(id),
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX ON consolidated_memory USING ivfflat (embedding vector_cosine_ops);
```

</div>
