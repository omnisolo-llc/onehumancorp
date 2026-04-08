---
status: "PENDING"
Title: "KAIROS Phase 1: Shared Task List Backend Database Design"
Priority: "P0"
Estimated Scope: "Medium"
---

# Title: KAIROS Phase 1: Shared Task List Backend Database Design

## Problem Statement
The One Human Corp (OHC) platform needs a central "KAIROS" orchestration layer to decompose complex feature requests for the Swarm. We must architect the database schema (e.g., PostgreSQL) and microservices mapping to decompose high-level feature requests for the OHC Swarm. A robust shared task list must be durable in both Cloud-Native and Standalone Desktop modes to track these tasks securely.

## Research Report
- Based on the Hybrid Architecture constraints, PostgreSQL is used for multi-tenant scalability in the cloud, while SQLite is used for standalone desktop degradation.
- Row-level locking via PostgreSQL's `FOR UPDATE SKIP LOCKED` is necessary to prevent worker collision when multiple agents attempt to claim a task.
- For SQLite Standalone mode, application-level mutexes must be used alongside standard transaction isolations.

## Design Doc
**Database Schema (`shared_tasks`):**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
```
- `dependencies` uses a JSONB array to optimize storage as per memory guidelines.
- `TIMESTAMPTZ` is compatible with Postgres and will gracefully degrade in SQLite.

## Implementation Prompt
Hello Implementer agent! Your mission is to implement the backend database designs for the "Shared Task List" feature.
1. Create the SQL migration file for `shared_tasks` in `srcs/server/db/migrations/`. Name it appropriately (e.g., `014_shared_tasks.sql`).
2. Add the migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Create the data access layer in `srcs/server/orchestration/tasks_db.go`.
4. Implement a `ClaimTask` method.
   - For PostgreSQL (`dbWrapper.Provider().IsSQLite() == false`), you MUST use `SELECT * FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED LIMIT 1`.
   - For SQLite Standalone mode, use application-level mutexes `to.mu.Lock()`.
5. Create unit tests for `tasks_db.go`. If simulating authentication claims in tests, use `context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)`.
6. Use `bazelisk test //srcs/server/orchestration/...` to verify your code.

## Visual Excellence Guidelines
Any frontend representation of the Shared Task List later created must strictly apply the OHC "Premium Feel":
```css
backdrop-filter: blur(20px) saturate(200%);
background: rgba(255, 255, 255, 0.03);
font-family: 'Outfit', 'Inter', sans-serif;
```

## Priority
P0

## Estimated Scope
Medium
