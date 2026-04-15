import re

with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

# Replace CompleteTaskWithResult to check currentStatus before transitioning
pattern = re.compile(r"""	var createdAt time\.Time
	var currentStatus string
	err := tm\.db\.QueryRow\(ctx, "SELECT created_at, status FROM shared_tasks WHERE id = \$1 AND agent_id = \$2 AND organization_id = \$3", taskID, agentID, claims\.OrganizationID\)\.Scan\(&createdAt, &currentStatus\)
	if err != nil {
		if errors\.Is\(err, sql\.ErrNoRows\) {
			return errors\.New\("task not found or not assigned to agent"\)
		}
		return fmt\.Errorf\("failed to verify task ownership: %w", err\)
	}""")

new_content = pattern.sub("""	var createdAt time.Time
	var currentStatus string
	err := tm.db.QueryRow(ctx, "SELECT created_at, status FROM shared_tasks WHERE id = $1 AND agent_id = $2 AND organization_id = $3", taskID, agentID, claims.OrganizationID).Scan(&createdAt, &currentStatus)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("task not found or not assigned to agent")
		}
		return fmt.Errorf("failed to verify task ownership: %w", err)
	}

	if currentStatus == statemachine.StateCompleted {
		return errors.New("task is already completed")
	}""", content)

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(new_content)
