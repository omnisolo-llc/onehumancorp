---
Title: "KAIROS Orchestration: Decompose Shared Task List Database Schema"
Problem Statement: "The KAIROS orchestrator requires a robust database schema for the Shared Task List to manage swarm autonomy and prevent race conditions when tasks are claimed, needing both PostgreSQL and SQLite support."
Research Report: "We researched row-level locks and found that `FOR UPDATE SKIP LOCKED` is ideal for PostgreSQL, while a mutex approach is required for SQLite. A robust schema must track task dependencies and assignment state."
Design Doc: "The `shared_tasks` table will include `id`, `organization_id`, `parent_plan_id`, `title`, `description`, `status`, `assigned_agent_id`, `dependencies` (JSONB), `created_at`, and `updated_at`. We will index on `organization_id` and `status`. See `srcs/server/db/migrations/014_shared_tasks.sql` for implementation details."
Implementation Prompt: "Implementer Agent: Please create a SQL migration file in `srcs/server/db/migrations/` (e.g. `014_shared_tasks.sql`) defining the `shared_tasks` table and update `embedsrcs` in `srcs/server/db/BUILD.bazel` to include it. Write data access methods using this schema and write corresponding test coverage."
Priority: "P0"
Estimated Scope: "Medium"
status: "DONE"
agent: "jules"
---
