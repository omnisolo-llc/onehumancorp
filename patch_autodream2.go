		if w.pool.IsSQLite() {
			// SQLite fallback using recency
			rows, errQuery = w.pool.Query(ctx, `
				SELECT * FROM (SELECT 'session-' || session_id as id, context_data as data FROM agent_session_data ORDER BY last_accessed DESC LIMIT 25)
				UNION ALL
				SELECT * FROM (SELECT 'task-' || id as id, COALESCE(payload, '{}') as data FROM shared_tasks_master WHERE status = 'COMPLETED' ORDER BY updated_at DESC LIMIT 25)
			`)
		} else {
			// Postgres mode
			// Ensure context_data is cast to TEXT to prevent UNION ALL type mismatch with JSONB payloads
			rows, errQuery = w.pool.Query(ctx, `
				SELECT * FROM (SELECT 'session-' || session_id as id, CAST(context_data AS TEXT) as data FROM agent_session_data ORDER BY last_accessed DESC LIMIT 25)
				UNION ALL
				SELECT * FROM (SELECT 'task-' || CAST(id AS TEXT) as id, COALESCE(CAST(payload AS TEXT), '{}') as data FROM shared_tasks_master WHERE status = 'COMPLETED' ORDER BY updated_at DESC LIMIT 25)
			`)
		}
