import re

with open("srcs/server/agents/kairos/task_decomposer.go", "r") as f:
    content = f.read()

# Fix the compile error caused by missing assignID variable in Postgres section
new_pg_acquire = """} else {
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
			var assignID *string
			if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &assignID, &t.Priority, &pStr, &t.ParentPlanID, &dJSON); err != nil {
				rows.Close()
				return nil, fmt.Errorf("failed to scan pg task: %w", err)
			}
			if assignID != nil {
				t.AssignedAgentID = *assignID
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
	}"""

content = re.sub(r'\} else \{\n\t\t// PostgreSQL: We can use a CTE to filter.*?^\t\}', new_pg_acquire, content, flags=re.DOTALL|re.MULTILINE)

with open("srcs/server/agents/kairos/task_decomposer.go", "w") as f:
    f.write(content)
