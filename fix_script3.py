import re

with open("srcs/server/agents/kairos/task_decomposer.go", "r") as f:
    content = f.read()

# Fix the bug in SQLite AcquirePendingTask where it fails to pick task if assigned_agent_id is NULL instead of ""
# And also in finding deps satisfying
new_sqlite_acquire = """if td.provider.IsSQLite() {
		query := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies FROM shared_tasks_decomposition WHERE organization_id = $1 AND status != 'COMPLETED'`
		rows, err := tx.Query(ctx, query, organizationID)
		if err != nil {
			return nil, fmt.Errorf("failed to query tasks: %w", err)
		}

		var allTasks []*Task
		taskMap := make(map[string]*Task)
		for rows.Next() {
			var t Task
			var dJSON string
			var pStr *string
			var assignID *string
			if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &assignID, &t.Priority, &pStr, &t.ParentPlanID, &dJSON); err != nil {
			    rows.Close()
				return nil, fmt.Errorf("failed to scan task: %w", err)
			}
			if assignID != nil {
				t.AssignedAgentID = *assignID
			}
			if err := json.Unmarshal([]byte(dJSON), &t.Dependencies); err != nil {
			    rows.Close()
				return nil, fmt.Errorf("failed to unmarshal dependencies: %w", err)
			}
			allTasks = append(allTasks, &t)
			taskMap[t.ID] = &t
		}
		rows.Close()

		var targetTaskID string
		for _, t := range allTasks {
			if t.Status == "PENDING" && (t.AssignedAgentID == "" || t.AssignedAgentID == agentID) {
				depsSatisfied := true
				for _, depID := range t.Dependencies {
					if dep, ok := taskMap[depID]; !ok || dep.Status != "COMPLETED" {
					    // If missing from map it could be completed
					    if !ok {
					        var depStatus string
                            err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", depID).Scan(&depStatus)
                            if err != nil || depStatus != "COMPLETED" {
                                depsSatisfied = false
                                break
                            }
					    } else {
						    depsSatisfied = false
						    break
						}
					}
				}
				if depsSatisfied {
					targetTaskID = t.ID
					break
				}
			}
		}

		if targetTaskID == "" {
			return nil, nil // No task available
		}

		updateQuery := `
			UPDATE shared_tasks_decomposition
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = datetime('now', '+5 minutes'), updated_at = CURRENT_TIMESTAMP
			WHERE id = $2 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < datetime('now'))
		`
		res, err := tx.Exec(ctx, updateQuery, agentID, targetTaskID)
		if err != nil {
			return nil, fmt.Errorf("failed to update sqlite lock: %w", err)
		}
		if res == 0 {
			return nil, ErrLockFailed
		}

		var assignID *string
		selectQuery := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies FROM shared_tasks_decomposition WHERE id = $1`
		err = tx.QueryRow(ctx, selectQuery, targetTaskID).Scan(
			&lockedTask.ID, &lockedTask.OrganizationID, &lockedTask.Title, &lockedTask.Description,
			&lockedTask.Status, &assignID, &lockedTask.Priority, &payloadStr,
			&lockedTask.ParentPlanID, &depsJSON,
		)
		if err != nil {
			return nil, fmt.Errorf("failed to read back locked task: %w", err)
		}
		if assignID != nil {
			lockedTask.AssignedAgentID = *assignID
		}

	}"""

# Use regex to replace the SQLite block
content = re.sub(r'if td\.provider\.IsSQLite\(\) \{.*?\n\t\} else \{', new_sqlite_acquire + ' else {', content, flags=re.DOTALL)

with open("srcs/server/agents/kairos/task_decomposer.go", "w") as f:
    f.write(content)
