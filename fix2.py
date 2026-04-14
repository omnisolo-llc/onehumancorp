import sys

with open("srcs/server/lib/orchestration/tasks/tasks.go", "r") as f:
    content = f.read()

# Fix the RowsAffected issue. Exec returns (int64, error) directly, not sql.Result
fix_exec = """	rowsAffected, err := tx.Exec(ctx, updateQuery, task.Status, task.AssignedAgentID, task.LockedUntil, task.UpdatedAt, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if rowsAffected == 0 {
		// Task was likely claimed by another worker concurrently
		return nil, nil
	}"""

content = content.replace("""	res, err := tx.Exec(ctx, updateQuery, task.Status, task.AssignedAgentID, task.LockedUntil, task.UpdatedAt, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil {
		return nil, fmt.Errorf("failed to get rows affected: %w", err)
	}
	if rowsAffected == 0 {
		// Task was likely claimed by another worker concurrently
		return nil, nil
	}""", fix_exec)

with open("srcs/server/lib/orchestration/tasks/tasks.go", "w") as f:
    f.write(content)
