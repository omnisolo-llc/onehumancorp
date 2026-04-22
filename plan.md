Wait! The task actually reads:
"Implement Shared Task List (KAIROS Orchestration)
We lack a robust distributed state machine to track asynchronous tasks (swarm_tasks and shared_tasks) across the swarm with exact sequence and DAG dependencies.
1. Schema Updates: Ensure swarm_tasks and shared_tasks are created or updated correctly.
2. Go Models: Define DAG dependency logic and task struct updates to enforce sequence blocking.
3. Provider: Add PostgreSQL-specific logic utilizing FOR UPDATE SKIP LOCKED inside explicit transactions to avoid releasing row locks too early.
4. Fallback: Use pool.IsSQLite() checks to degrade gracefully and apply SQLite single-node concurrency mechanisms.
Implement DAG blocking/unblocking logic so dependent tasks only run when parent tasks complete."

In `srcs/server/orchestration/shared_tasks.go`, we have `ClaimTask` for `shared_tasks_decomposition`.
Wait, look at `srcs/server/orchestration/tasks_db.go`: `ClaimTaskV4` has:
```go
    query := `
        SELECT id FROM shared_tasks_v4 t
        WHERE t.status = 'PENDING' AND t.organization_id = $1
        AND NOT EXISTS (
            SELECT 1
            FROM jsonb_array_elements_text(t.dependencies::jsonb) d
            JOIN shared_tasks_v4 dep ON dep.id = d
            WHERE dep.status != 'COMPLETED'
        )
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
```

So I need to apply similar logic to:
1. `srcs/server/orchestration/shared_tasks.go`: `ClaimTask` (updates `shared_tasks_decomposition`)
2. `srcs/server/orchestration/state/cloud_state_manager.go`: `ClaimTask` (updates `swarm_tasks`)
3. `srcs/server/orchestration/state/standalone_state_manager.go`: `ClaimTask` (updates `swarm_tasks`)
4. Ensure `060_shared_task_list.sql` is in `srcs/server/db/BUILD.bazel`.

Let's modify `srcs/server/orchestration/shared_tasks.go`:
```go
<<<<<<< SEARCH
		query = `SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at FROM shared_tasks_decomposition WHERE status = 'PENDING' AND organization_id = $1 LIMIT 1`
=======
		query = `
			SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
			FROM shared_tasks_decomposition t
			WHERE t.status = 'PENDING' AND t.organization_id = $1
			AND NOT EXISTS (
				SELECT 1 FROM json_each(t.dependencies) d
				JOIN shared_tasks_decomposition dep ON dep.id = d.value
				WHERE dep.status != 'COMPLETED' AND dep.status != 'DONE'
			)
			LIMIT 1
		`
>>>>>>> REPLACE
```

```go
<<<<<<< SEARCH
		query = `
		UPDATE shared_tasks_decomposition
		SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks_decomposition
			WHERE status = 'PENDING' AND organization_id = $2
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
	    `
=======
		query = `
		UPDATE shared_tasks_decomposition
		SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT t.id FROM shared_tasks_decomposition t
			WHERE t.status = 'PENDING' AND t.organization_id = $2
			AND NOT EXISTS (
				SELECT 1 FROM jsonb_array_elements_text(COALESCE(t.dependencies, '[]'::jsonb)) d
				JOIN shared_tasks_decomposition dep ON dep.id::text = d
				WHERE dep.status != 'COMPLETED' AND dep.status != 'DONE'
			)
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
	    `
>>>>>>> REPLACE
```

Let's check `shared_tasks_decomposition` schema:
```sql
CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    agent_id TEXT,
    priority TEXT NOT NULL DEFAULT 'P2',
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    locked_until DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

Let's do this for `cloud_state_manager.go` and `standalone_state_manager.go`.
