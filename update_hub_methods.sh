cat >> srcs/server/orchestration/postgres_hub.go << 'EOL'

func (r *PgHubRepository) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	var claimed bool
	err := pgWithRetry(ctx, func() error {
		tx, err := r.pool.Begin(ctx)
		if err != nil {
			return fmt.Errorf("pg: begin claim task: %w", err)
		}
		defer func() { _ = tx.Rollback(ctx) }()

		var status string
		var lockedUntil *time.Time
		err = tx.QueryRow(ctx, "SELECT status, locked_until FROM swarm_tasks WHERE id = $1 FOR UPDATE SKIP LOCKED", taskID).Scan(&status, &lockedUntil)
		if err != nil {
			if err.Error() == "sql: no rows in result set" {
				claimed = false
				return nil
			}
			return fmt.Errorf("pg: query task: %w", err)
		}

		if status != "PENDING" && status != "FAILED" && (lockedUntil == nil || lockedUntil.After(time.Now())) {
			claimed = false
			return nil
		}

		newLock := time.Now().Add(30 * time.Second)
		_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = $2, updated_at = NOW() WHERE id = $3", agentID, newLock, taskID)
		if err != nil {
			return fmt.Errorf("pg: update task: %w", err)
		}

		if err := tx.Commit(ctx); err != nil {
			return fmt.Errorf("pg: commit task claim: %w", err)
		}

		claimed = true
		return nil
	})
	return claimed, err
}
EOL

cat >> srcs/server/orchestration/sqlite_hub.go << 'EOL'

func (r *SqliteHubRepository) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return false, fmt.Errorf("sqlite: begin claim task: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	var status string
	var lockedUntil *time.Time
	err = tx.QueryRow(ctx, "SELECT status, locked_until FROM swarm_tasks WHERE id = ? LIMIT 1", taskID).Scan(&status, &lockedUntil)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			return false, nil
		}
		return false, fmt.Errorf("sqlite: query task: %w", err)
	}

	if status != "PENDING" && status != "FAILED" && (lockedUntil == nil || lockedUntil.After(time.Now())) {
		return false, nil
	}

	newLock := time.Now().Add(30 * time.Second)
	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = ?, locked_until = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?", agentID, newLock, taskID)
	if err != nil {
		return false, fmt.Errorf("sqlite: update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return false, fmt.Errorf("sqlite: commit task claim: %w", err)
	}

	return true, nil
}
EOL
