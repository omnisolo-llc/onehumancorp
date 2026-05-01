# KAIROS Orchestration: Shared Task List Database & Sequence Design

**Problem Statement:** Agents need a centralized source of truth for their tasks to orchestrate swarm intelligence effectively. Currently, task states are disjointed.

**Research Report:** A centralized PostgreSQL schema handling the `shared_tasks` table ensures data consistency across the multi-tenant K8s deployment, while SQLite provides a degraded fallback for Standalone Mode.

**Design Doc:**
### Architecture
- **Cloud-Native**: PostgreSQL `shared_tasks` table with JSONB for flexible payload.
- **Standalone**: SQLite `shared_tasks` table.

### Schema
```sql
CREATE TABLE shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    agent_id TEXT,
    priority TEXT NOT NULL DEFAULT 'P2',
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

**Implementation Prompt:**
Implement the `shared_tasks` database migrations in `src/server/migrations/`. Include migrations for PostgreSQL. Ensure robust Rust struct definitions and unit test coverage.

**Priority:** P0
**Estimated Scope:** Medium
