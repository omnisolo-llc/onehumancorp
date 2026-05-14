package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"
)

type SharedTask struct {
	ID             string           `json:"id"`
	OrganizationID string           `json:"organization_id"`
	Title          string           `json:"title"`
	Description    *string          `json:"description,omitempty"`
	Status         string           `json:"status"`
	AgentID        *string          `json:"agent_id,omitempty"`
	Priority       string           `json:"priority"`
	Payload        *json.RawMessage `json:"payload,omitempty"`
	ParentPlanID   *string          `json:"parent_plan_id,omitempty"`
	Dependencies   json.RawMessage  `json:"dependencies,omitempty"`
	CreatedAt      time.Time        `json:"created_at"`
	UpdatedAt      time.Time        `json:"updated_at"`
	Action         string           `json:"action,omitempty"`
}

type TaskStore interface {
	ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error)
	CreateTask(ctx context.Context, task *SharedTask) error
	GetTask(ctx context.Context, id string) (*SharedTask, error)
	UpdateTaskStatus(ctx context.Context, id string, status string) error
	GetTasksByOrganization(ctx context.Context, organizationID string) ([]*SharedTask, error)
	PollDelegatedTasks(ctx context.Context, limit int) ([]*SharedTask, error)
	ReportMissionHandover(ctx context.Context, missionID string, blockers string) error
	SanitizeBacklog(ctx context.Context) error
}

// PostgresTaskStore implementation
type PostgresTaskStore struct {
	db *sql.DB
}

func NewPostgresTaskStore(db *sql.DB) *PostgresTaskStore {
	return &PostgresTaskStore{db: db}
}

// ClaimTask atomically retrieves and assigns an available PENDING task to the given agent ID.
// It ensures concurrency safety, utilizing database locks for Postgres or a mutex for SQLite.
// Tasks with unresolved dependencies (not COMPLETED) are excluded from claiming.
func (s *PostgresTaskStore) ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error) {

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	_, err = tx.ExecContext(ctx, "SET LOCAL app.current_tenant = $1", organizationID)
	if err != nil {
		return nil, err
	}

	query := `
		SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND organization_id = $1 AND NOT EXISTS (
            SELECT 1 FROM task_dependencies td
            JOIN shared_tasks dep ON dep.id = td.depends_on_task_id
            WHERE td.task_id = shared_tasks.id AND dep.status != 'COMPLETED'
        )
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	row := tx.QueryRowContext(ctx, query, organizationID)

	task := &SharedTask{}
	var payloadBytes, depsBytes []byte
	err = row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
		&task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
		&task.CreatedAt, &task.UpdatedAt,
	)

	if err == sql.ErrNoRows {
		return nil, nil // No tasks available
	} else if err != nil {
		return nil, err
	}

	if len(payloadBytes) > 0 {
		raw := json.RawMessage(payloadBytes)
		task.Payload = &raw
	}
	if len(depsBytes) > 0 {
		task.Dependencies = json.RawMessage(depsBytes)
	}

	// Update status to ASSIGNED and set agent_id
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
		RETURNING id
	`
	var returnedID string
	err = tx.QueryRowContext(ctx, updateQuery, agentID, task.ID).Scan(&returnedID)
	if err != nil {
		if err == sql.ErrNoRows {
			// Lost the race or something went wrong
			return nil, errors.New("failed to claim task")
		}
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	task.Status = "ASSIGNED"
	task.AgentID = &agentID

	return task, nil
}

func (s *PostgresTaskStore) CreateTask(ctx context.Context, task *SharedTask) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()
	_, err = tx.ExecContext(ctx, "SELECT set_config('app.current_tenant', $1, true)", task.OrganizationID)
	if err != nil {
		return err
	}

	query := `
		INSERT INTO shared_tasks (organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
		RETURNING id, created_at, updated_at
	`

	var payloadBytes, depsBytes []byte
	if task.Payload != nil {
		payloadBytes = []byte(*task.Payload)
	}
	if len(task.Dependencies) > 0 {
		depsBytes = []byte(task.Dependencies)
	} else {
		depsBytes = []byte("[]")
	}

	err = tx.QueryRowContext(ctx, query,
		task.OrganizationID, task.Title, task.Description, task.Status,
		task.AgentID, task.Priority, payloadBytes, task.ParentPlanID, depsBytes,
	).Scan(&task.ID, &task.CreatedAt, &task.UpdatedAt)

	if err != nil {
		return err
	}

	if len(task.Dependencies) > 0 {
		var deps []string
		if err := json.Unmarshal(task.Dependencies, &deps); err == nil {
			for _, depID := range deps {
				if _, err := tx.ExecContext(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2) ON CONFLICT DO NOTHING", task.ID, depID); err != nil {
					return err
				}
			}
		}
	}
	return tx.Commit()
}

func (s *PostgresTaskStore) GetTask(ctx context.Context, id string) (*SharedTask, error) {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

    query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks
        WHERE id = $1
    `
    row := tx.QueryRowContext(ctx, query, id)

    task := &SharedTask{}
    var payloadBytes, depsBytes []byte
    err = row.Scan(
        &task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
        &task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
        &task.CreatedAt, &task.UpdatedAt,
    )

    if err == sql.ErrNoRows {
        return nil, errors.New("task not found")
    } else if err != nil {
        return nil, err
    }

    if len(payloadBytes) > 0 {
        raw := json.RawMessage(payloadBytes)
        task.Payload = &raw
    }
    if len(depsBytes) > 0 {
        task.Dependencies = json.RawMessage(depsBytes)
    }

	if err := tx.Commit(); err != nil {
		return nil, err
	}
    return task, nil
}

func (s *PostgresTaskStore) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	_, err = tx.ExecContext(ctx, query, status, id)
	if err != nil {
		return err
	}
	return tx.Commit()
}


func (s *PostgresTaskStore) PollDelegatedTasks(ctx context.Context, limit int) ([]*SharedTask, error) {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND priority = 'DELEGATED'
		FOR UPDATE SKIP LOCKED
		LIMIT $1
	`
	rows, err := tx.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*SharedTask
	var claimedIDs []string
	for rows.Next() {
		task := &SharedTask{}
		var payloadBytes, depsBytes []byte
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
			&task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
			&task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		tasks = append(tasks, task)
		claimedIDs = append(claimedIDs, task.ID)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}
	rows.Close()

	if len(claimedIDs) > 0 {
		for _, id := range claimedIDs {
			updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
			_, err = tx.ExecContext(ctx, updateQuery, id)
			if err != nil {
				return nil, err
			}
		}
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	for _, task := range tasks {
		task.Status = "ASSIGNED"
	}

	return tasks, nil
}

func (s *PostgresTaskStore) SanitizeBacklog(ctx context.Context) error {
	_, err := s.db.ExecContext(ctx, "DELETE FROM agent_missions WHERE status = 'blocked'")
	return err
}

func (s *PostgresTaskStore) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	_, err := s.db.ExecContext(ctx, `
		UPDATE agent_missions
		SET status = 'blocked',
		    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '
' END || $1
		WHERE id = $2`, blockers, missionID)
	return err
}

func (s *PostgresTaskStore) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*SharedTask, error) {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()
	_, err = tx.ExecContext(ctx, "SELECT set_config('app.current_tenant', $1, true)", organizationID)
	if err != nil {
		return nil, err
	}

    query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks
        WHERE organization_id = $1
    `
    rows, err := tx.QueryContext(ctx, query, organizationID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*SharedTask
	for rows.Next() {
		task := &SharedTask{}
		var payloadBytes, depsBytes []byte
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
			&task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
			&task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		tasks = append(tasks, task)
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}
	return tasks, nil
}

// SqliteTaskStore implementation
type SqliteTaskStore struct {
	db    *sql.DB
	mutex sync.Mutex
}

func NewSqliteTaskStore(db *sql.DB) *SqliteTaskStore {
	return &SqliteTaskStore{db: db}
}

func (s *SqliteTaskStore) ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error) {
	s.mutex.Lock()
	defer s.mutex.Unlock()

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	// Find a pending task
	query := `
		SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND organization_id = ? AND NOT EXISTS (
            SELECT 1 FROM task_dependencies td
            JOIN shared_tasks dep ON dep.id = td.depends_on_task_id
            WHERE td.task_id = shared_tasks.id AND dep.status != 'COMPLETED'
        )
		LIMIT 1
	`
	row := tx.QueryRowContext(ctx, query, organizationID)

	task := &SharedTask{}
	var payloadBytes, depsBytes []byte
    var createdAtStr, updatedAtStr string
	err = row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
		&task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
		&createdAtStr, &updatedAtStr,
	)

	if err == sql.ErrNoRows {
		return nil, nil // No tasks available
	} else if err != nil {
		return nil, err
	}

    // Simplistic time parsing for SQLite timestamp strings
    if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
        task.CreatedAt = t
    }
    if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
        task.UpdatedAt = t
    }

	if len(payloadBytes) > 0 {
		raw := json.RawMessage(payloadBytes)
		task.Payload = &raw
	}
	if len(depsBytes) > 0 {
		task.Dependencies = json.RawMessage(depsBytes)
	}

	// Update status to ASSIGNED and set agent_id
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', agent_id = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ? AND status = 'PENDING'
		RETURNING id
	`
	var returnedID string
	err = tx.QueryRowContext(ctx, updateQuery, agentID, task.ID).Scan(&returnedID)
	if err != nil {
		if err == sql.ErrNoRows {
			// Lost the race or something went wrong
			return nil, errors.New("failed to claim task")
		}
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	task.Status = "ASSIGNED"
	task.AgentID = &agentID

	return task, nil
}

func (s *SqliteTaskStore) CreateTask(ctx context.Context, task *SharedTask) error {
	s.mutex.Lock()
	defer s.mutex.Unlock()

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

    // Generate UUID in Go for SQLite if it doesn't have gen_random_uuid()
	query := `
		INSERT INTO shared_tasks (id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	var payloadBytes, depsBytes []byte
	if task.Payload != nil {
		payloadBytes = []byte(*task.Payload)
	}
	if len(task.Dependencies) > 0 {
		depsBytes = []byte(task.Dependencies)
	} else {
		depsBytes = []byte("[]")
	}

    if task.ID == "" {
        task.ID = "id-" + time.Now().Format("20060102150405.000000") // Mock UUID for SQLite test simplicity if not provided
    }

    if task.Status == "" {
        task.Status = "PENDING"
    }

    if task.Priority == "" {
        task.Priority = "P2"
    }

	_, err = tx.ExecContext(ctx, query,
        task.ID, task.OrganizationID, task.Title, task.Description, task.Status,
		task.AgentID, task.Priority, payloadBytes, task.ParentPlanID, depsBytes,
	)

    if err == nil {
        task.CreatedAt = time.Now()
        task.UpdatedAt = time.Now()
    }
	if err == nil && len(task.Dependencies) > 0 {
		var deps []string
		if errUnpack := json.Unmarshal(task.Dependencies, &deps); errUnpack == nil {
			for _, depID := range deps {
				if _, errDep := tx.ExecContext(ctx, "INSERT OR IGNORE INTO task_dependencies (task_id, depends_on_task_id) VALUES (?, ?)", task.ID, depID); errDep != nil {
					return errDep
				}
			}
		}
	}

	if err != nil {
		return err
	}
	return tx.Commit()
}

func (s *SqliteTaskStore) PollDelegatedTasks(ctx context.Context, limit int) ([]*SharedTask, error) {
	s.mutex.Lock()
	defer s.mutex.Unlock()

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND priority = 'DELEGATED'
		LIMIT ?
	`
	rows, err := tx.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*SharedTask
	var claimedIDs []string

	for rows.Next() {
		task := &SharedTask{}
		var payloadBytes, depsBytes []byte
		var createdAtStr, updatedAtStr string
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
			&task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
			&createdAtStr, &updatedAtStr,
		)
		if err != nil {
			return nil, err
		}
		if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
			task.CreatedAt = t
		}
		if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
			task.UpdatedAt = t
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		tasks = append(tasks, task)
		claimedIDs = append(claimedIDs, task.ID)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}
	rows.Close()

	if len(claimedIDs) > 0 {
		for _, id := range claimedIDs {
			updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'PENDING'`
			_, err = tx.ExecContext(ctx, updateQuery, id)
			if err != nil {
				return nil, err
			}
		}
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	for _, task := range tasks {
		task.Status = "ASSIGNED"
	}

	return tasks, nil
}

func (s *SqliteTaskStore) GetTask(ctx context.Context, id string) (*SharedTask, error) {
    query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks
        WHERE id = ?
    `
    row := s.db.QueryRowContext(ctx, query, id)

    task := &SharedTask{}
    var payloadBytes, depsBytes []byte
    var createdAtStr, updatedAtStr string
    err := row.Scan(
        &task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
        &task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
        &createdAtStr, &updatedAtStr,
    )

    if err == sql.ErrNoRows {
        return nil, errors.New("task not found")
    } else if err != nil {
        return nil, err
    }

    if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
        task.CreatedAt = t
    }
    if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
        task.UpdatedAt = t
    }

    if len(payloadBytes) > 0 {
        raw := json.RawMessage(payloadBytes)
        task.Payload = &raw
    }
    if len(depsBytes) > 0 {
        task.Dependencies = json.RawMessage(depsBytes)
    }

    return task, nil
}

func (s *SqliteTaskStore) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	query := `UPDATE shared_tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
	_, err := s.db.ExecContext(ctx, query, status, id)
	return err
}


func (s *SqliteTaskStore) SanitizeBacklog(ctx context.Context) error {
	_, err := s.db.ExecContext(ctx, "DELETE FROM agent_missions WHERE status = 'blocked'")
	return err
}

func (s *SqliteTaskStore) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	_, err := s.db.ExecContext(ctx, `
		UPDATE agent_missions
		SET status = 'blocked',
		    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '
' END || ?
		WHERE id = ?`, blockers, missionID)
	return err
}

func (s *SqliteTaskStore) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*SharedTask, error) {
    query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks
        WHERE organization_id = ?
    `
    rows, err := s.db.QueryContext(ctx, query, organizationID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*SharedTask
	for rows.Next() {
		task := &SharedTask{}
		var payloadBytes, depsBytes []byte
		var createdAtStr, updatedAtStr string
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
			&task.AgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes,
			&createdAtStr, &updatedAtStr,
		)
		if err != nil {
			return nil, err
		}
		if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
			task.CreatedAt = t
		}
		if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
			task.UpdatedAt = t
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		tasks = append(tasks, task)
	}

	return tasks, nil
}
