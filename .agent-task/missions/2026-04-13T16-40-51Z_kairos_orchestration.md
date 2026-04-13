---
title: "KAIROS: Shared Task List Database Schema and Orchestration APIs"
status: DONE
agent: Link
agent: Implementer
priority: P0
estimated_scope: Large
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS: Shared Task List Database Schema and Orchestration APIs

## Problem Statement
The OHC swarm requires a shared task list schema to synchronize tasks between different AI agents. The current implementation is scattered and lacks a reliable storage mechanism for state machine tracking and Teammate Mesh coordination in both Cloud-Native and Standalone modes.

## Research Report
- **Market Reality:** Competitor platforms often use purely local (unscalable) or purely cloud (low privacy/no offline) task queues.
- **OHC's Unfair Advantage:** Our Hybrid Architecture (OHC-HA) uses PostgreSQL (cloud) and SQLite (standalone).
- **KAIROS Orchestrator Needs:**
  1. **Shared Task List:** Database schema using `VARCHAR PRIMARY KEY` to support both UUIDs generated in application code (avoiding Postgres-specific `gen_random_uuid()` for SQLite compatibility).
  2. **Teammate Mesh:** Redis/in-memory PubSub for coordination.
  3. **AutoDream:** Vector pipeline for memory consolidation.

## Design Doc

### 1. Shared Task List (Decomposition & State Machine)
**Schema (PostgreSQL & SQLite Compatible):**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### 2. Teammate Mesh APIs (Orchestration)
Realtime communication layer for agent coordination using `mesh:tasks` and `mesh:coordination`.

### 3. AutoDream Data Pipelines (Memory Consolidation)
Asynchronous background pipeline to summarize task outcomes and store in `ohc_memory_embeddings`.

## Implementation Prompt
Hello Implementer agent!
1. Please add the `shared_tasks_v4` migration (using `VARCHAR PRIMARY KEY` and explicit UUID generation in application code, to ensure SQLite compatibility).
2. Implement the Go struct `SharedTaskOrchestrator` to interface with this database.

## Priority
P0

## Estimated Scope
Large

</div>
