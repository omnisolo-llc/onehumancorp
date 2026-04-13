<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS: Phase 1 - Architect Shared Task List

## Problem Statement
The KAIROS orchestrator requires a robust backend datastore for the "Shared Task List" feature. Without a consistent PostgreSQL/SQLite schema and data access layer, we cannot implement task decomposition and coordination.

## Research Report
*   **Database**: OHC's hybrid structure requires SQL migrations that work on both Postgres (Cloud Mode) and SQLite (Standalone Mode).
*   **Schema**: Needs to track task hierarchy (parent/child), state (`PENDING`, `IN_PROGRESS`, etc.), and assignments.
*   **Tenant Isolation**: Strict isolation using `organization_id` via `OrganizationIDFromContext`.

## Design Doc
### Shared Task List Database Schema
**Table `tasks`**:
*   `id` (UUID, Primary Key)
*   `organization_id` (UUID, Foreign Key, Indexed)
*   `parent_task_id` (UUID, nullable, Foreign Key to `tasks.id`)
*   `title` (VARCHAR(255), Not Null)
*   `description` (TEXT)
*   `status` (VARCHAR(50), Not Null, Default 'PENDING')
*   `assigned_agent_role` (VARCHAR(100))
*   `created_at` (TIMESTAMP)
*   `updated_at` (TIMESTAMP)

**Table `task_dependencies`**:
*   `task_id` (UUID, Foreign Key)
*   `depends_on_task_id` (UUID, Foreign Key)
*   `Primary Key` (task_id, depends_on_task_id)

### Sequence Diagram
1. User Request -> API Handler.
2. KAIROS Decomposes Request.
3. KAIROS Inserts Parent Task + Child Tasks to `tasks` table.
4. KAIROS inserts dependencies to `task_dependencies`.

## Implementation Prompt
**Role**: Implementer Agent
**Task**: Create the DB migrations and Go repository for the Shared Task List.
**Instructions**:
1. Add a migration file `srcs/server/db/migrations/{timestamp}_create_shared_tasks.sql` defining the `tasks` and `task_dependencies` tables. Ensure it's compatible with both Postgres and SQLite.
2. Implement the Go struct models in `srcs/server/repository/models.go`.
3. Create `srcs/server/repository/task_repo.go` with methods `CreateTask`, `GetTasksByOrg`, and `UpdateTaskStatus`.
4. Enforce tenant isolation in queries using `OrganizationIDFromContext(ctx)`.
**Acceptance Criteria**:
*   Migrations run without errors.
*   Repository tests pass.
*   Test coverage for `task_repo.go` > 90%.

## Priority
P0

## Estimated Scope
Medium

</div>
