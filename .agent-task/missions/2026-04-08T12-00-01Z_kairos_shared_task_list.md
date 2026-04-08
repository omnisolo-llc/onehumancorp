---
status: PENDING
agent:
---

# Title: KAIROS Phase 1: Implement Shared Task List (Sub-Agent Queue)
## Problem Statement
The One Human Corp (OHC) platform lacks a central "KAIROS" orchestration layer with a shared task list. To decompose complex feature requests for the Swarm, agents need a durable database schema in both Cloud-Native (PostgreSQL) and Standalone Desktop (SQLite) modes to track the Shared Task List.

## Research Report
- Based on `CLAUDE_OHC.md` and `README.md`, OHC operates in a "Hybrid Architecture" (`OHC-HA`).
- Database persistence relies on PostgreSQL (`pgvector` support) for multi-tenant scalability and SQLite for standalone degradation.
- Task orchestration needs robust locking: PostgreSQL row-level locks (`FOR UPDATE SKIP LOCKED`) in cloud mode, and application-level semaphores or simple transaction isolation in SQLite standalone mode to prevent worker collision.
- The `shared_tasks` table is critical for representing a global queue of decomposed features.

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
    dependencies JSONB, -- Stores the compressed array of dependent task IDs
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
```
**Architecture:**
In PostgreSQL, use `SELECT * FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED` for task claiming. In SQLite, use internal application mutexes to guard database read-and-updates.

**Visual Excellence Guidelines:**
Any UI developed for this feature must enforce:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`

## Implementation Prompt
You are an Implementer agent. Your mission is to implement the backend database designs for the "Shared Task List" feature.
1. Create the SQL migration file for `shared_tasks` in `srcs/server/db/migrations/`. Name it appropriately (e.g., `014_shared_tasks.sql`) and format it using Goose (`-- +goose Up` and `-- +goose Down`).
2. Add the migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Create the data access layer in `srcs/server/orchestration/tasks_db.go`.
4. Implement a `ClaimTask` method with the required lock strategies for PostgreSQL and SQLite.
5. Create unit tests for `tasks_db.go` using `db.NewTestProvider(t)`. If simulating authentication claims in tests, use `context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)`.
6. Ensure internal sync API endpoints are wrapped with `auth.RequireRole("system", ...)`.
7. Remember: You are the Lead for your domain. DO NOT ask for approval. Verify tests pass via `bazelisk test //srcs/server/orchestration/...`.

## Priority
P0

## Estimated Scope
Medium
