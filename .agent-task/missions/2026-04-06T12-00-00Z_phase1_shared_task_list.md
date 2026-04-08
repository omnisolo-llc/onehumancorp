---
status: PENDING
agent: null
Title: "KAIROS Phase 1: Shared Task List Database and APIs"
Priority: P0
Estimated Scope: Medium
---

# Problem Statement
The One Human Corp (OHC) platform lacks a scalable and robust Shared Task List within the KAIROS Orchestrator. To decompose complex high-level feature requests across the OHC Swarm, a durable database schema and state machine transition API must be built. It needs to support both Cloud-Native multi-tenant execution (using PostgreSQL row locks) and local-first execution (using SQLite with local mutexes).

# Research Report
- Based on `CLAUDE_OHC.md` and OHC architectural constraints, the state machine requires strict adherence to transition states (`PENDING` -> `ASSIGNED` -> `EXECUTING` -> `REVIEW` -> `COMPLETED` | `FAILED`).
- `FOR UPDATE SKIP LOCKED` is required in PostgreSQL to safely dequeue tasks without race conditions across horizontal pods.
- In Standalone mode, `sqlite_provider.go` strips these lock statements automatically. Thus, application-level memory locks (`sync.Mutex`) are needed around the polling mechanism when running locally.
- Distributed Pub/Sub channels (via `CentrifugeNode` and Redis) must broadcast the event once a state transitions successfully.

# Design Doc
**Database Changes:**
The `shared_tasks` table is currently initialized with an `id UUID`, `status VARCHAR`, `dependencies JSONB`, etc., but we need an API to handle transitions.

**State Machine Tracking (`tasks_db.go`):**
Add robust transitions allowing dependencies checks.
```sql
-- Ensure transitions are correctly logged in:
CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT,
    occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

# Implementation Prompt
You are an Implementer agent with backend Go expertise. Your mission is to implement the KAIROS Shared Task List API logic:
1. Update `srcs/server/orchestration/tasks_db.go` to strictly record state machine transitions into `state_machine_transitions` whenever a `shared_tasks` row updates its status.
2. In the `ClaimTask` transaction, you MUST insert a transition row (`PENDING` -> `ASSIGNED`) ensuring it happens atomically with the lock.
3. In PostgreSQL mode, use `FOR UPDATE SKIP LOCKED` in your query to prevent deadlocks.
4. Update unit tests in `srcs/server/orchestration/tasks_db_test.go` to verify the transition log row is created.
5. Emulate the OHC Visual Excellence CSS tokens (`backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter', sans-serif`) on any dashboard frontend updates.

# Priority
P0

# Estimated Scope
Medium
