import re

with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

claim_task_search = r"""	if tm\.db\.IsSQLite\(\) \{
		// SQLite doesn't support UPDATE \.\.\. RETURNING with a LIMIT, so we use explicit two-step select-then-update within the transaction\.
		selectQuery := `
			SELECT st\.id
			FROM shared_tasks st
			WHERE st\.id = \$1 AND st\.organization_id = \$2 AND st\.status = 'PENDING' AND \(st\.locked_until IS NULL OR st\.locked_until < CURRENT_TIMESTAMP\)
			AND \(SELECT COUNT\(\*\) FROM task_dependencies td INNER JOIN shared_tasks d ON td\.depends_on_task_id = d\.id WHERE td\.task_id = st\.id AND d\.status != 'COMPLETED'\) = 0
			ORDER BY st\.priority ASC, st\.created_at ASC
			LIMIT 1
		`
		var fetchedTaskID string
		err := tx\.QueryRow\(ctx, selectQuery, taskID, claims\.OrganizationID\)\.Scan\(&fetchedTaskID\)
		if err != nil \{
			if errors\.Is\(err, sql\.ErrNoRows\) \{
				return nil, nil // No task available or blocked
			\}
			if strings\.Contains\(err\.Error\(\), "database is locked"\) \|\| strings\.Contains\(err\.Error\(\), "SQLITE_BUSY"\) \{
				return nil, fmt\.Errorf\("database is locked: %w", err\)
			\}
			return nil, fmt\.Errorf\("failed to check pending task: %w", err\)
		\}

		updateQuery := `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = \$1, updated_at = CURRENT_TIMESTAMP
			WHERE id = \$2
			RETURNING id, organization_id, COALESCE\(parent_plan_id, ''\), title, payload, status, priority, locked_until, created_at, updated_at
		`
		errQuery = tx\.QueryRow\(ctx, updateQuery, agentID, fetchedTaskID\)\.Scan\(
			&task\.ID, &task\.OrganizationID, &task\.ParentPlanID, &task\.Title, &task\.Payload, &task\.Status, &task\.Priority, &task\.LockedUntil, &task\.CreatedAt, &task\.UpdatedAt,
		\)

		if errQuery == nil \{
			auditQuery := `
				INSERT INTO state_machine_transitions \(id, entity_id, entity_type, from_state, to_state, agent_id, reason\)
				VALUES \(\$1, \$2, \$3, \$4, \$5, \$6, \$7\)
			`
			_, auditErr := tx\.Exec\(ctx, auditQuery, generateID\(\), task\.ID, "SHARED_TASK", statemachine\.StatePending, statemachine\.StateInProgress, agentID, "Task claimed by ID"\)
			if auditErr != nil \{
				return nil, fmt\.Errorf\("failed to record transition audit log: %w", auditErr\)
			\}
		\}
	\} else \{
		// PostgreSQL with FOR UPDATE SKIP LOCKED
		query := `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', agent_id = \$3, updated_at = CURRENT_TIMESTAMP
			WHERE id IN \(
				SELECT st\.id
				FROM shared_tasks st
				WHERE st\.id = \$1 AND st\.organization_id = \$2 AND st\.status = 'PENDING' AND \(st\.locked_until IS NULL OR st\.locked_until < CURRENT_TIMESTAMP\)
				AND \(SELECT COUNT\(\*\) FROM task_dependencies td INNER JOIN shared_tasks d ON td\.depends_on_task_id = d\.id WHERE td\.task_id = st\.id AND d\.status != 'COMPLETED'\) = 0
				ORDER BY st\.priority ASC, st\.created_at ASC
				LIMIT 1 FOR UPDATE SKIP LOCKED
			\)
			RETURNING id, organization_id, COALESCE\(parent_plan_id, ''\), title, payload, status, priority, locked_until, created_at, updated_at
		`
		errQuery = tx\.QueryRow\(ctx, query, taskID, claims\.OrganizationID, agentID\)\.Scan\(
			&task\.ID, &task\.OrganizationID, &task\.ParentPlanID, &task\.Title, &task\.Payload, &task\.Status, &task\.Priority, &task\.LockedUntil, &task\.CreatedAt, &task\.UpdatedAt,
		\)

		if errQuery == nil \{
			auditQuery := `
				INSERT INTO state_machine_transitions \(id, entity_id, entity_type, from_state, to_state, agent_id, reason\)
				VALUES \(\$1, \$2, \$3, \$4, \$5, \$6, \$7\)
			`
			_, auditErr := tx\.Exec\(ctx, auditQuery, generateID\(\), task\.ID, "SHARED_TASK", statemachine\.StatePending, statemachine\.StateInProgress, agentID, "Task claimed by ID"\)
			if auditErr != nil \{
				return nil, fmt\.Errorf\("failed to record transition audit log: %w", auditErr\)
			\}
		\}
	\}

	if errQuery != nil \{
		if errors\.Is\(errQuery, sql\.ErrNoRows\) \{
			return nil, nil // No task available or locked
		\}
		if strings\.Contains\(errQuery\.Error\(\), "database is locked"\) \|\| strings\.Contains\(errQuery\.Error\(\), "SQLITE_BUSY"\) \{
			return nil, fmt\.Errorf\("database is locked: %w", errQuery\)
		\}
		return nil, fmt\.Errorf\("failed to claim pending task: %w", errQuery\)
	\}"""

claim_task_replace = """	var fetchedTaskID string
	var query string
	if tm.db.IsSQLite() {
		query = `
			SELECT st.id
			FROM shared_tasks st
			WHERE st.id = $1 AND st.organization_id = $2 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY st.priority ASC, st.created_at ASC
			LIMIT 1
		`
	} else {
		query = `
			SELECT st.id
			FROM shared_tasks st
			WHERE st.id = $1 AND st.organization_id = $2 AND st.status = 'PENDING' AND (st.locked_until IS NULL OR st.locked_until < CURRENT_TIMESTAMP)
			AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0
			ORDER BY st.priority ASC, st.created_at ASC
			LIMIT 1 FOR UPDATE SKIP LOCKED
		`
	}

	errQuery = tx.QueryRow(ctx, query, taskID, claims.OrganizationID).Scan(&fetchedTaskID)
	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available or locked
		}
		if strings.Contains(errQuery.Error(), "database is locked") || strings.Contains(errQuery.Error(), "SQLITE_BUSY") {
			return nil, fmt.Errorf("database is locked: %w", errQuery)
		}
		return nil, fmt.Errorf("failed to check pending task: %w", errQuery)
	}

	errQuery = tm.stateMachine.TransitionWithTx(ctx, tx, fetchedTaskID, "SHARED_TASK", statemachine.StateInProgress, agentID, "Task claimed by ID")
	if errQuery != nil {
		return nil, fmt.Errorf("failed to transition state: %w", errQuery)
	}

	fetchQuery := `SELECT id, organization_id, COALESCE(parent_plan_id, ''), title, payload, status, priority, locked_until, created_at, updated_at FROM shared_tasks WHERE id = $1`
	errQuery = tx.QueryRow(ctx, fetchQuery, fetchedTaskID).Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Payload, &task.Status, &task.Priority, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
	)
	if errQuery != nil {
		return nil, fmt.Errorf("failed to fetch claimed task: %w", errQuery)
	}"""

content = re.sub(claim_task_search, claim_task_replace, content, count=1, flags=re.DOTALL)

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(content)
