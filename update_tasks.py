import re

with open('srcs/server/orchestration/tasks.go', 'r') as f:
    content = f.read()

# Update PollTasks to handle dependencies
old_sqlite_query_poll = """		query = `
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
			LIMIT $1
		`"""

new_sqlite_query_poll = """		// Check if task has uncompleted dependencies
		query = `
			SELECT st.id, st.mission_id, st.parent_plan_id, st.dependencies, st.title, st.payload, st.status, st.locked_until, st.created_at, st.updated_at
			FROM swarm_tasks st
			WHERE st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			  AND NOT EXISTS (
			      SELECT 1 FROM task_dependencies td
			      JOIN swarm_tasks dep ON dep.id = td.depends_on_task_id
			      WHERE td.task_id = st.id AND dep.status != 'COMPLETED'
			  )
			ORDER BY json_extract(st.payload, '$.priority') ASC, st.created_at ASC
			LIMIT $1
		`"""

old_pg_query_poll = """		query = `
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY payload->>'priority' ASC, created_at ASC
			LIMIT $1
			FOR UPDATE SKIP LOCKED
		`"""

new_pg_query_poll = """		query = `
			SELECT st.id, st.mission_id, st.parent_plan_id, st.dependencies, st.title, st.payload, st.status, st.locked_until, st.created_at, st.updated_at
			FROM swarm_tasks st
			WHERE st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			  AND NOT EXISTS (
			      SELECT 1 FROM task_dependencies td
			      JOIN swarm_tasks dep ON dep.id = td.depends_on_task_id
			      WHERE td.task_id = st.id AND dep.status != 'COMPLETED'
			  )
			ORDER BY st.payload->>'priority' ASC, st.created_at ASC
			LIMIT $1
			FOR UPDATE SKIP LOCKED
		`"""

content = content.replace(old_sqlite_query_poll, new_sqlite_query_poll)
content = content.replace(old_pg_query_poll, new_pg_query_poll)

# Update ClaimTask to handle dependencies
old_sqlite_query_claim = """		query := `
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
			LIMIT 1
		`"""

new_sqlite_query_claim = """		query := `
			SELECT st.id, st.mission_id, st.parent_plan_id, st.dependencies, st.title, st.payload, st.status, st.locked_until, st.created_at, st.updated_at
			FROM swarm_tasks st
			WHERE st.id = $1 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			  AND NOT EXISTS (
			      SELECT 1 FROM task_dependencies td
			      JOIN swarm_tasks dep ON dep.id = td.depends_on_task_id
			      WHERE td.task_id = st.id AND dep.status != 'COMPLETED'
			  )
			ORDER BY json_extract(st.payload, '$.priority') ASC, st.created_at ASC
			LIMIT 1
		`"""

old_pg_query_claim = """		query := `
			SELECT id, mission_id, parent_plan_id, dependencies, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY payload->>'priority' ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`"""

new_pg_query_claim = """		query := `
			SELECT st.id, st.mission_id, st.parent_plan_id, st.dependencies, st.title, st.payload, st.status, st.locked_until, st.created_at, st.updated_at
			FROM swarm_tasks st
			WHERE st.id = $1 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			  AND NOT EXISTS (
			      SELECT 1 FROM task_dependencies td
			      JOIN swarm_tasks dep ON dep.id = td.depends_on_task_id
			      WHERE td.task_id = st.id AND dep.status != 'COMPLETED'
			  )
			ORDER BY st.payload->>'priority' ASC, st.created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`"""

content = content.replace(old_sqlite_query_claim, new_sqlite_query_claim)
content = content.replace(old_pg_query_claim, new_pg_query_claim)


with open('srcs/server/orchestration/tasks.go', 'w') as f:
    f.write(content)
