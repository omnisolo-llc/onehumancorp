---
status: PENDING
agent: Implementer
priority: P0
---
# Title: Shared Task List (Phase 1)
## Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. We lack a robust distributed state machine to track asynchronous tasks (`shared_tasks`) with sequence and DAG dependencies.
## Research Report
PostgreSQL provides horizontal scaling (via row-level locks `FOR UPDATE SKIP LOCKED`), while SQLite handles standalone gracefully (via application-level Mutex). Dependencies will be stored as JSONB/TEXT.
## Design Doc
Schema (`srcs/server/db/migrations/033_kairos_shared_tasks.sql`):
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    dependencies JSONB NOT NULL DEFAULT '[]'
);
```
## Implementation Prompt
Implement the `shared_tasks` table and `FOR UPDATE SKIP LOCKED` querying in Go.
## Priority
P0
## Estimated Scope
Medium
