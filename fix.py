import sys

with open("srcs/server/lib/orchestration/tasks/tasks.go", "r") as f:
    content = f.read()

# Fix 1: Make Description nullable
content = content.replace("var payload sql.NullString\n\tvar parentPlanID sql.NullString", "var payload sql.NullString\n\tvar parentPlanID sql.NullString\n\tvar description sql.NullString")
content = content.replace("&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.Priority, &payload, &parentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,", "&task.ID, &task.OrganizationID, &task.Title, &description, &task.Status, &task.Priority, &payload, &parentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,")

# Add back string mapping
string_mapping = """	if payload.Valid {
		task.Payload = payload.String
	}
	if parentPlanID.Valid {
		task.ParentPlanID = parentPlanID.String
	}
	if description.Valid {
		task.Description = description.String
	}"""
content = content.replace("	if payload.Valid {\n\t\ttask.Payload = payload.String\n\t}\n\tif parentPlanID.Valid {\n\t\ttask.ParentPlanID = parentPlanID.String\n\t}", string_mapping)


# Fix 2: Add AND status = 'PENDING' to UPDATE queries and check RowsAffected
update_sqlite = """	updateQuery := `
		UPDATE shared_tasks_v2
		SET status = ?, assigned_agent_id = ?, locked_until = ?, updated_at = ?
		WHERE id = ? AND status = 'PENDING'
	`"""
content = content.replace("	updateQuery := `\n\t\tUPDATE shared_tasks_v2\n\t\tSET status = ?, assigned_agent_id = ?, locked_until = ?, updated_at = ?\n\t\tWHERE id = ?\n\t`", update_sqlite)

update_pg = """	if !s.db.IsSQLite() {
		updateQuery = `
			UPDATE shared_tasks_v2
			SET status = $1, assigned_agent_id = $2, locked_until = $3, updated_at = $4
			WHERE id = $5 AND status = 'PENDING'
		`
	}"""
content = content.replace("	if !s.db.IsSQLite() {\n\t\tupdateQuery = `\n\t\t\tUPDATE shared_tasks_v2\n\t\t\tSET status = $1, assigned_agent_id = $2, locked_until = $3, updated_at = $4\n\t\t\tWHERE id = $5\n\t\t`\n\t}", update_pg)

# Fix 3: Handle result and RowsAffected
exec_call = """	res, err := tx.Exec(ctx, updateQuery, task.Status, task.AssignedAgentID, task.LockedUntil, task.UpdatedAt, task.ID)
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
	}"""
content = content.replace("	_, err = tx.Exec(ctx, updateQuery, task.Status, task.AssignedAgentID, task.LockedUntil, task.UpdatedAt, task.ID)\n\tif err != nil {\n\t\treturn nil, fmt.Errorf(\"failed to update task: %w\", err)\n\t}", exec_call)


with open("srcs/server/lib/orchestration/tasks/tasks.go", "w") as f:
    f.write(content)
