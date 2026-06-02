issue_title: "Implement Missing sub_agent_jobs Database Schema"
issue_description: |
  **Problem Statement**:
  The codebase references `sub_agent_jobs` in multiple files, notably `src/server/orchestration/queue/pg_queue.rs` and `sqlite_queue.rs`, to perform background task orchestration and manage Sub-Agent queues via SQL commands. However, the database schema migration files in `src/server/migrations/` were missing the creation of the `sub_agent_jobs` table. This causes failures when the application boots and tries to interact with the queue.

  **Research Report**:
  - Investigating the `docs/technical/architecture/kairos/sub-agent-queue-design.md`, it details the `sub_agent_queue` schema, which in recent versions of code appears as `sub_agent_jobs`.
  - Searched through all migration scripts using `grep -rn "sub_agent_jobs" src/server/migrations/` and `grep -rn "CREATE TABLE.*queue"` but found no existing table definition for `sub_agent_jobs`.
  - The missing table is critical for handling asynchronous Sub-Agent jobs correctly and preserving multi-tenant isolation.

  **Design Doc**:
  The table schema needs to match the usage in the `PgTaskQueue` and `SqliteTaskQueue` modules, capturing all columns needed (`id`, `tenant_id`, `parent_task_id`, `agent_role`, `payload`, `status`, `attempts`, `max_attempts`, `run_after`, `locked_until`, `created_at`, `updated_at`).
  It must incorporate the PostgreSQL Row-Level Security (RLS) policy using `tenant_id` to maintain strict tenant isolation.

  **Implementation Prompt**:
  "Create a new database migration file `src/server/migrations/058_sub_agent_jobs.sql` that defines the `sub_agent_jobs` table schema exactly as requested in the design document, making sure to apply PostgreSQL Row Level Security (RLS) on the `tenant_id`."

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
