import re

with open("srcs/server/agents/kairos/task_decomposer.go", "r") as f:
    content = f.read()

# Replace the whole logic inside AcquirePendingTask with a proper SKIP LOCKED implementation
new_acquire = """func (td *TaskDecomposer) AcquirePendingTask(ctx context.Context, organizationID, agentID string) (*Task, error) {
	tx, err := td.provider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var lockedTask Task
	var depsJSON string
	var payloadStr *string

	if td.provider.IsSQLite() {
		// SQLite: We use a CTE or nested select to find a task whose dependencies are satisfied
		// Since JSON functions in SQLite might be limited depending on version,
		// we simplify by doing a clean SELECT and UPDATE.
		// Wait, a better approach for SQLite that is safe:

		// 1. We must select a pending task whose dependencies are completed.
		// Since we cannot easily do a full JSON array join in SQLite universally,
		// we can do the filtering in Go, BUT within the transaction to maintain safety.

		query := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies FROM shared_tasks_decomposition WHERE organization_id = $1 AND status != 'COMPLETED'`
		rows, err := tx.Query(ctx, query, organizationID)
		if err != nil {
			return nil, fmt.Errorf("failed to query tasks: %w", err)
		}
		defer rows.Close()

		var allTasks []*Task
		taskMap := make(map[string]*Task)
		for rows.Next() {
			var t Task
			var dJSON string
			var pStr *string
			if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &t.AssignedAgentID, &t.Priority, &pStr, &t.ParentPlanID, &dJSON); err != nil {
				return nil, fmt.Errorf("failed to scan task: %w", err)
			}
			if err := json.Unmarshal([]byte(dJSON), &t.Dependencies); err != nil {
				return nil, fmt.Errorf("failed to unmarshal dependencies: %w", err)
			}
			allTasks = append(allTasks, &t)
			taskMap[t.ID] = &t
		}

		// This close is needed because sqlite tx cannot execute while rows are open
		rows.Close()

		var targetTaskID string
		for _, t := range allTasks {
			if t.Status == "PENDING" && (t.AssignedAgentID == "" || t.AssignedAgentID == agentID) {
				depsSatisfied := true
				for _, depID := range t.Dependencies {
					if dep, ok := taskMap[depID]; !ok || dep.Status != "COMPLETED" {
						// Note: If a dependency is missing from the DB entirely, it's either deleted or cross-org (invalid).
						// We treat it as unsatisfied or we'd need to query it. We assume tasks in same org.
						depsSatisfied = false
						break
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

		selectQuery := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies FROM shared_tasks_decomposition WHERE id = $1`
		err = tx.QueryRow(ctx, selectQuery, targetTaskID).Scan(
			&lockedTask.ID, &lockedTask.OrganizationID, &lockedTask.Title, &lockedTask.Description,
			&lockedTask.Status, &lockedTask.AssignedAgentID, &lockedTask.Priority, &payloadStr,
			&lockedTask.ParentPlanID, &depsJSON,
		)
		if err != nil {
			return nil, fmt.Errorf("failed to read back locked task: %w", err)
		}

	} else {
		// PostgreSQL: We can use a CTE to filter tasks whose dependencies are completed,
		// and then FOR UPDATE SKIP LOCKED on them.

		// To avoid complex JSON array joins in SQL for this basic implementation,
		// we fetch a batch of PENDING tasks, check dependencies, and attempt to lock ONE.
		// This uses SKIP LOCKED correctly.

		selectQuery := `
			SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies
			FROM shared_tasks_decomposition
			WHERE organization_id = $1 AND status = 'PENDING' AND (assigned_agent_id IS NULL OR assigned_agent_id = '' OR assigned_agent_id = $2)
			FOR UPDATE SKIP LOCKED LIMIT 10
		`

		rows, err := tx.Query(ctx, selectQuery, organizationID, agentID)
		if err != nil {
			return nil, fmt.Errorf("failed to query pg pending tasks: %w", err)
		}

		var pendingTasks []*Task
		for rows.Next() {
			var t Task
			var dJSON string
			var pStr *string
			if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &t.AssignedAgentID, &t.Priority, &pStr, &t.ParentPlanID, &dJSON); err != nil {
				rows.Close()
				return nil, fmt.Errorf("failed to scan pg task: %w", err)
			}
			if pStr != nil {
				t.Payload = []byte(*pStr)
			}
			if err := json.Unmarshal([]byte(dJSON), &t.Dependencies); err != nil {
				rows.Close()
				return nil, fmt.Errorf("failed to unmarshal pg dependencies: %w", err)
			}
			pendingTasks = append(pendingTasks, &t)
		}
		rows.Close()

		var targetTaskID string
		for _, t := range pendingTasks {
			if len(t.Dependencies) == 0 {
				targetTaskID = t.ID
				break
			}

			// Verify dependencies are completed
			depsSatisfied := true
			for _, depID := range t.Dependencies {
				var depStatus string
				err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", depID).Scan(&depStatus)
				if err != nil || depStatus != "COMPLETED" {
					depsSatisfied = false
					break
				}
			}
			if depsSatisfied {
				targetTaskID = t.ID
				break
			}
		}

		if targetTaskID == "" {
			// None of the locked pending tasks had satisfied dependencies.
			// The transaction will rollback and release the locks.
			return nil, nil
		}

		// Update the specific task we chose
		updateQuery := `UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, targetTaskID)
		if err != nil {
			return nil, fmt.Errorf("failed to update pg lock: %w", err)
		}

		// Retrieve it to return
		for _, t := range pendingTasks {
			if t.ID == targetTaskID {
				lockedTask = *t
				lockedTask.Status = "IN_PROGRESS"
				lockedTask.AssignedAgentID = agentID

				depsBytes, _ := json.Marshal(t.Dependencies)
				depsJSON = string(depsBytes)

				if t.Payload != nil {
					pStr := string(t.Payload)
					payloadStr = &pStr
				}
				break
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit lock tx: %w", err)
	}

	if payloadStr != nil {
		lockedTask.Payload = []byte(*payloadStr)
	}
	if err := json.Unmarshal([]byte(depsJSON), &lockedTask.Dependencies); err != nil {
		return nil, fmt.Errorf("failed to unmarshal deps on locked task: %w", err)
	}

	return &lockedTask, nil
}"""

# Extract everything up to AcquirePendingTask
match = re.search(r'// AcquirePendingTask', content)
start_idx = match.start()

match2 = re.search(r'// checkCircularDependencies', content)
end_idx = match2.start()

new_content = content[:start_idx] + new_acquire + "\n\n\n" + content[end_idx:]

with open("srcs/server/agents/kairos/task_decomposer.go", "w") as f:
    f.write(new_content)
