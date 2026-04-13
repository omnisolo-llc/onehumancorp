---
status: PENDING
agent: Implementer
priority: P0
---

# Title: KAIROS: Shared Task List Database Schema (Phase 1)

## Problem Statement
The OHC Swarm needs a central "Shared Task List" backend database design to track complex sub-agent orchestration and UltraPlan DAG dependencies robustly in both Cloud-Native (PostgreSQL) and Standalone (SQLite) modes.

## Research Report
- Based on CLAUDE_OHC.md, Postgres provides horizontal scaling (`FOR UPDATE SKIP LOCKED`).
- SQLite gracefully handles standalone constraints via single-connection Mutex.
- Deep deliberation cycles need a DAG representation in the DB (dependencies).

## Design Doc
**Architecture & Schema:**
1. Create `srcs/server/db/migrations/035_kairos_shared_tasks.sql`
```sql
CREATE TABLE IF NOT EXISTS shared_tasks_v3 (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

## Implementation Prompt
Create `srcs/server/db/migrations/035_kairos_shared_tasks.sql` with the `shared_tasks_v3` table. Add it to `srcs/server/db/BUILD.bazel`. Update tasks logic with `ClaimTask` and DAG dependency resolution. Include robust test coverage (>90%).

## Priority
P0

## Estimated Scope
Large
