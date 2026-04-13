---
status: PENDING
priority: P0
scope: Medium
title: "KAIROS: Architect the Shared Task List Schema"
---

# Title: Architect the Shared Task List Schema

## Problem Statement
The OHC (One Human Corp) Swarm requires a robust mechanism to manage complex feature requests decomposed into a shared task list. As the KAIROS Orchestrator, we must architect the backend database designs and sequence diagrams for the "Shared Task List" feature. This is Phase 1 of the KAIROS Orchestrator playbook. The shared task list must function correctly in both Cloud-Native Mode (PostgreSQL) and Standalone Desktop Mode (SQLite), requiring a hybrid schema approach.

## Research Report
- KAIROS tasks are units of work assigned to specific agents.
- Each task needs a unique identifier, status (e.g., PENDING, IN_PROGRESS, DONE, BLOCKED), priority, a descriptive prompt, and association with the agent handling it.
- Dependencies between tasks (e.g., Task B depends on Task A) must be trackable to manage the orchestration graph.
- The state must be durable. We need SQL migration scripts (Goose format) to create the necessary tables.
- Since we are in the OHC Hybrid Architecture, any data storage components must elegantly degrade from PostgreSQL to SQLite.

## Design Doc
1.  **Schema Architecture:**
    - Table: `shared_tasks`
      - `id`: UUID (Primary Key)
      - `title`: VARCHAR (Title of the task)
      - `description`: TEXT (Detailed prompt/instructions)
      - `agent_id`: VARCHAR (Nullable, assigned agent's identifier)
      - `status`: VARCHAR (e.g., 'pending', 'in_progress', 'done', 'blocked')
      - `priority`: VARCHAR (e.g., 'P0', 'P1', 'P2')
      - `created_at`: TIMESTAMP
      - `updated_at`: TIMESTAMP
    - Table: `shared_task_dependencies`
      - `task_id`: UUID (Foreign Key to `shared_tasks.id`)
      - `depends_on_task_id`: UUID (Foreign Key to `shared_tasks.id`)
      - Primary Key (`task_id`, `depends_on_task_id`)
2.  **Sequence Diagrams:**
    - Define a `SequenceDiagram` string in a Go struct (e.g., within `srcs/server/orchestration/kairos_orchestrator.go` or related documentation) illustrating: `KAIROS -> Database: Insert Task`, `Agent -> Database: Claim Task`, `Agent -> Database: Update Status`.
3.  **Hybrid DB Considerations:**
    - Use standard SQL syntax compatible with both SQLite and PostgreSQL. Avoid Postgres-specific JSONB in these core relational tables unless abstracted. Use `TEXT` and parse JSON in application code if complex data is needed. Use `VARCHAR(36)` for UUIDs to remain compatible with SQLite.

## Implementation Prompt
- Create a new Goose migration file in the standard migration directory (e.g., `srcs/server/db/migrations/`).
- The migration should create the `shared_tasks` and `shared_task_dependencies` tables.
- Ensure the SQL syntax is cross-compatible (SQLite and PostgreSQL). For example, use `VARCHAR(36)` for UUIDs instead of the Postgres native `UUID` type to support SQLite Standalone mode.
- Provide a `DOWN` migration to drop these tables.
- Update `srcs/server/orchestration/ultraplan.go` (or similar file) with a detailed Sequence Diagram inside a Go comment block or constant string to fulfill Phase 1 playbook requirements.
