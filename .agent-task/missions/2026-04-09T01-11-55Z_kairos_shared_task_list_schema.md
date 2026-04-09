---
status: PENDING
---

# Title: KAIROS Orchestrator: Shared Task List Database Schema Design
## Problem Statement
The KAIROS Orchestrator needs a robust database schema to decompose complex feature requests into a shared task list for the agent team. This schema must support the state machine tracking of tasks across the Swarm, including dependencies (DAG) and real-time status updates via the Teammate Mesh. It needs to work seamlessly in both Cloud-Native (PostgreSQL) and Standalone (SQLite) modes.

## Research Report
The Shared Task List requires a central coordination queue backed by a distributed state machine. Task claiming must prevent race conditions, using `FOR UPDATE SKIP LOCKED` in Postgres and falling back to mutexes/transactions in SQLite. We also need to define the relationships between high-level missions and decomposed sub-tasks.

## Design Doc
We will create a new table `shared_tasks` to track decomposed tasks.
- **Columns**:
  - `id`: UUID (Primary Key)
  - `mission_id`: UUID (Foreign key to `agent_missions`)
  - `title`: String
  - `description`: Text
  - `status`: Enum/String (e.g., PENDING, IN_PROGRESS, BLOCKED, COMPLETED, FAILED)
  - `assigned_agent_id`: UUID (Nullable)
  - `parent_task_id`: UUID (Nullable, for DAG dependencies)
  - `payload`: JSONB (For specific task parameters/context)
  - `created_at`: Timestamp
  - `updated_at`: Timestamp
- **Indexes**: On `mission_id`, `status`, and `assigned_agent_id` for efficient querying.

## Implementation Prompt
You are an Implementer agent. Your task is to implement the backend database design for the KAIROS Shared Task List.
1. Create a new raw SQL migration file in `srcs/server/db/migrations/` (e.g., `20260409000000_create_shared_tasks.sql`).
2. Define the `CREATE TABLE shared_tasks` statement according to the Design Doc. Use PostgreSQL syntax (the SQLite fallback will automatically handle type replacements).
3. Ensure you include the necessary indexes.
4. If there is a downward migration, include `-- +goose Down` and `DROP TABLE IF EXISTS shared_tasks;` (be mindful of SQLite drop limitations if necessary).
5. Update `srcs/server/db/BUILD.bazel` to include the new migration file in the `embedsrcs` if required.
6. Verify the migration runs successfully in both PostgreSQL and SQLite environments.

## Priority
P0

## Estimated Scope
Medium
