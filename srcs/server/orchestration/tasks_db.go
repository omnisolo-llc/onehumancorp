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
	ID             string
	OrganizationID string
	Title          string
	Description    *string
	Status         string
	AgentID        *string
	Priority       string
	Payload        *json.RawMessage
	ParentPlanID   *string
	Dependencies   json.RawMessage
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

type TaskStore interface {
	ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error)
	CreateTask(ctx context.Context, task *SharedTask) error
	GetTask(ctx context.Context, id string) (*SharedTask, error)
	UpdateTaskStatus(ctx context.Context, id string, status string) error
	GetTasksByOrganization(ctx context.Context, organizationID string) ([]*SharedTask, error)
}

// PostgresTaskStore implementation
type PostgresTaskStore struct {
	db *sql.DB
}

func NewPostgresTaskStore(db *sql.DB) *PostgresTaskStore {
	return &PostgresTaskStore{db: db}
}

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
		FROM shared_tasks t1
		WHERE status = 'PENDING' AND organization_id = $1
		  AND NOT EXISTS (
			  SELECT 1 FROM jsonb_array_elements_text(COALESCE(t1.dependencies, '[]'::jsonb)) AS dep
			  LEFT JOIN shared_tasks t2 ON t2.id::text = dep
			  WHERE t2.status != 'COMPLETED' OR t2.status IS NULL
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
	`
	_, err = tx.ExecContext(ctx, updateQuery, agentID, task.ID)
	if err != nil {
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

	// Update and fetch a pending task atomically using UPDATE ... RETURNING
	query := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', agent_id = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT t1.id
			FROM shared_tasks t1
			WHERE t1.status = 'PENDING' AND t1.organization_id = ?
			  AND NOT EXISTS (
				  SELECT 1 FROM json_each(t1.dependencies) AS dep
				  LEFT JOIN shared_tasks t2 ON t2.id = dep.value
				  WHERE t2.status != 'COMPLETED' OR t2.status IS NULL
			  )
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
	`
	row := tx.QueryRowContext(ctx, query, agentID, organizationID)

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

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	return task, nil
}

func (s *SqliteTaskStore) CreateTask(ctx context.Context, task *SharedTask) error {
	s.mutex.Lock()
	defer s.mutex.Unlock()

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

	_, err := s.db.ExecContext(ctx, query,
        task.ID, task.OrganizationID, task.Title, task.Description, task.Status,
		task.AgentID, task.Priority, payloadBytes, task.ParentPlanID, depsBytes,
	)

    if err == nil {
        task.CreatedAt = time.Now()
        task.UpdatedAt = time.Now()
    }

	return err
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
