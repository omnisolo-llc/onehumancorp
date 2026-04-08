# Title: KAIROS Shared Task List & Distributed State Machine Tracker

## Problem Statement
The OHC Swarm currently lacks a robust, distributed state machine to track tasks across the Teammate Mesh. As agents decompose complex features into smaller pieces, they need a "Shared Task List" to prevent duplicate work, manage deep-deliberation cycles (UltraPlan), and track dependencies. Without this, sub-agents operating in background queues (e.g., KAIROS sub-agents) risk race conditions, orphaned tasks, and conflicting database states, particularly in Hybrid Cloud/Standalone modes.

## Research Report
*   **Context:** KAIROS orchestration demands high-fidelity dependency tracking. Shared tasks must be tracked using a state machine that seamlessly degrades from a horizontally scaled PostgreSQL/Redis environment down to a local SQLite Standalone mode.
*   **Competitive Analysis:** Modern workflow engines (like Temporal, Celery, BullMQ) rely on a central database for durability and a queueing layer (Redis/RabbitMQ) for realtime progression. OHC requires a bespoke implementation prioritizing minimal footprint for local execution while retaining pgvector/Postgres robustness for cloud.
*   **Findings:** The `shared_tasks` table should track dependencies directly using a `dependencies` JSONB array instead of a separate relational table to optimize storage costs, as noted in the KAIROS memory constraints. Concurrent updates across the Mesh must rely on database locking (`FOR UPDATE SKIP LOCKED` in Postgres, Mutexes in SQLite) to avoid data races.
*   **References:** `docs/features/kairos/state_machine.md`, `CLAUDE_OHC.md` Hybrid architecture guidelines.

## Design Doc

### 1. Database Schema
A new table `kairos_shared_tasks` will be created.

```sql
CREATE TABLE kairos_shared_tasks (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    parent_task_id UUID REFERENCES kairos_shared_tasks(id),
    title VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, in_progress, blocked, completed, failed
    dependencies JSONB DEFAULT '[]', -- JSON array of UUIDs (Optimization for storage cost)
    agent_id UUID, -- Assigned agent
    payload JSONB, -- Task metadata/context
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
CREATE INDEX idx_kairos_tasks_tenant_status ON kairos_shared_tasks(tenant_id, status);
```

**Note:** For SQLite migrations, fallback from JSONB to TEXT and avoid `IF NOT EXISTS` syntax in `ALTER TABLE ... ADD COLUMN` per guidelines.

### 2. Teammate Mesh & Orchestration
The KAIROS orchestrator will publish events to the Teammate Mesh via Redis Pub/Sub (`mesh:tasks` channel) when a task state changes. Sub-agents will consume from these queues (e.g., `orchestration.TaskQueue` interface). We will implement a `TaskRegistry` in Go that handles `ClaimTask()`, `CompleteTask()`, and `FailTask()`, employing `FOR UPDATE SKIP LOCKED` on Postgres and standard application-level `sync.Mutex` on SQLite.

### 3. Visual Excellence & Dashboard UI
A new dashboard widget will track the swarm's activity. The UI must adhere to the OHC Premium Aesthetic:
```html
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; border-radius: 12px; padding: 16px; border: 1px solid rgba(255,255,255,0.1);">
  <h3>Swarm Activity (Shared Task List)</h3>
  <ul>
    <li>[Agent 1]: Processing Schema (In Progress)</li>
    <li>[Agent 2]: Pending Dependency (Blocked)</li>
  </ul>
</div>
```

## Implementation Prompt
**Role:** Implementer Agent
**Task:** Implement the KAIROS Shared Task List API and Database Schema.

1.  **Migrations:** Create the database migration files in `srcs/server/db/migrations/` to define the `kairos_shared_tasks` table. Implement the PostgreSQL specific migration using `JSONB` for the `dependencies` column. Implement the SQLite specific migration using `TEXT` for the dependencies column.
2.  **Go Models:** Add the Go structs in `srcs/server/orchestration/models.go` (or create `task_models.go`) for `SharedTask`.
3.  **Repository Logic:** Implement `ClaimTask`, `CompleteTask`, and `FailTask` in `srcs/server/orchestration/task_repository.go`. Ensure you use the `orchestration.TaskQueue` interface pattern.
    *   *Critical:* Implement concurrent safety. Use `FOR UPDATE SKIP LOCKED` for PostgreSQL and a global `sync.Mutex` for SQLite Standalone mode to prevent race conditions during `ClaimTask`.
4.  **Teammate Mesh Integration:** In `CompleteTask`, publish a message to the `mesh:tasks` channel using the `rueidis` Redis client.
5.  **API Endpoints:** Create `GET /api/v1/orchestration/tasks` and `POST /api/v1/orchestration/tasks` inside `srcs/server/orchestration/handlers.go`.
6.  **Tests:** Write unit tests in `srcs/server/orchestration/task_repository_test.go`. Mock the database and simulate concurrent task claims to verify lock integrity.
7.  **Visual Verification:** Ensure the new Dashboard components in `srcs/app/lib/dashboard/` apply the required glassmorphism visual tokens: `BackdropFilter` with `ImageFilter.compose` (20px blur) and `AnimatedScale` (1.02 hover).

## Priority
P0

## Estimated Scope
Large
