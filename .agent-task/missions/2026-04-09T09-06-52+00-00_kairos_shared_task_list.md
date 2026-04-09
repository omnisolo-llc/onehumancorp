---
agent: "KAIROS Orchestrator"
status: "PENDING"
Title: "Implement KAIROS Shared Task List Schema and Sequence"
Priority: "P0"
Estimated Scope: "Medium"
---

# Problem Statement
The OHC swarm requires a durable, distributed state machine (The Brain) to allow horizontal pod concurrency in the cloud without worker collisions. The Shared Task List needs to support KAIROS Decomposition and UltraPlan Deliberation.

# Research Report
PostgreSQL `FOR UPDATE SKIP LOCKED` is the ideal solution for concurrent queue processing. SQLite requires fallback transaction mechanisms for standalone mode. The `shared_tasks` table is currently partially defined but lacks full task decomposition fields like `parent_task_id`, `state_machine_data`, and proper index locking.

# Design Doc
**Architecture:**
- **Table:** `shared_tasks` (PostgreSQL / SQLite fallback).
- **Columns:** `id`, `parent_task_id` (UUID), `title`, `description`, `status` (PENDING, CLAIMED, BLOCKED, COMPLETED), `state_machine_data` (JSONB), `locked_until` (TIMESTAMPTZ), `agent_id` (VARCHAR), `organization_id` (VARCHAR).
- **Sequence:**
  1. KAIROS Orchestrator breaks down large feature -> Inserts N tasks into `shared_tasks` with `parent_task_id`.
  2. Implementer agents poll -> `SELECT ... FOR UPDATE SKIP LOCKED` -> update `status = 'CLAIMED'`.
  3. On completion -> Update `status = 'COMPLETED'`.

**API Contracts:**
- `POST /api/v1/tasks/decompose` -> `[]TaskID`
- `POST /api/v1/tasks/claim` -> `Task`
- `POST /api/v1/tasks/{id}/complete`

# Implementation Prompt
Implement the schema migration for `shared_tasks` adding `parent_task_id` and `state_machine_data`. Create the Go backend service in `srcs/server/orchestration/shared_tasks.go` implementing the `ClaimTask` method using `SKIP LOCKED` logic (with fallback for SQLite). Ensure all new database interactions are wrapped in `otel.Meter` histograms. Write unit tests in `srcs/server/orchestration/shared_tasks_test.go` confirming 100% test coverage. Follow the 'Zero WIP' standard and the OHC-SIP memory update protocol.
