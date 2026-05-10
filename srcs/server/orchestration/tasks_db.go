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
	ID              string           `json:"id"`
	OrganizationID  string           `json:"organization_id"`
	ParentPlanID    *string          `json:"parent_plan_id,omitempty"`
	Title           string           `json:"title"`
	Description     *string          `json:"description,omitempty"`
	Status          string           `json:"status"`
	AssignedAgentID *string          `json:"assigned_agent_id,omitempty"`
	Dependencies    json.RawMessage  `json:"dependencies,omitempty"`
	Priority        string           `json:"priority,omitempty"`
	Payload         *json.RawMessage `json:"payload,omitempty"`
	CreatedAt       time.Time        `json:"created_at"`
	UpdatedAt       time.Time        `json:"updated_at"`
}

type TaskStore interface {
	ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error)
	CreateTask(ctx context.Context, task *SharedTask) error
	GetTask(ctx context.Context, id string, organizationID string) (*SharedTask, error)
	UpdateTaskStatus(ctx context.Context, id string, status string) error
	GetTasksByOrganization(ctx context.Context, organizationID string) ([]*SharedTask, error)
	PollDelegatedTasks(ctx context.Context, limit int) ([]*SharedTask, error)
	ReportMissionHandover(ctx context.Context, missionID string, blockers string) error
}

// PostgresTaskStore implementation
type PostgresTaskStore struct {
	db *sql.DB
}

func NewPostgresTaskStore(db *sql.DB) *PostgresTaskStore {
	return &PostgresTaskStore{db: db}
}

// ClaimTask atomically retrieves and assigns an available PENDING task to the given agent ID.
// It ensures concurrency safety, utilizing database locks for Postgres.
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
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND organization_id = $1 AND NOT EXISTS (
            SELECT 1 FROM shared_tasks dep
            WHERE dep.id = ANY(SELECT jsonb_array_elements_text(shared_tasks.dependencies)::text)
            AND dep.status != 'COMPLETED'
        )
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	row := tx.QueryRowContext(ctx, query, organizationID)

	task := &SharedTask{}
	var depsBytes, payloadBytes []byte
	err = row.Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
		&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &task.CreatedAt, &task.UpdatedAt,
	)

	if err == sql.ErrNoRows {
		return nil, nil // No tasks available
	} else if err != nil {
		return nil, err
	}

	if len(depsBytes) > 0 {
		task.Dependencies = json.RawMessage(depsBytes)
	}
	if len(payloadBytes) > 0 {
		raw := json.RawMessage(payloadBytes)
		task.Payload = &raw
	}

	// Update status to ASSIGNED and set assigned_agent_id
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
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
	task.AssignedAgentID = &agentID

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
		INSERT INTO shared_tasks (id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
		RETURNING created_at, updated_at
	`

	var depsBytes, payloadBytes []byte
	if len(task.Dependencies) > 0 {
		depsBytes = []byte(task.Dependencies)
	} else {
		depsBytes = []byte("[]")
	}
	if task.Payload != nil {
		payloadBytes = []byte(*task.Payload)
	}

	err = tx.QueryRowContext(ctx, query,
		task.ID, task.OrganizationID, task.ParentPlanID, task.Title, task.Description, task.Status,
		task.AssignedAgentID, depsBytes, task.Priority, payloadBytes,
	).Scan(&task.CreatedAt, &task.UpdatedAt)

	if err != nil {
		return err
	}

	return tx.Commit()
}

func (s *PostgresTaskStore) GetTask(ctx context.Context, id string, organizationID string) (*SharedTask, error) {
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
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
		FROM shared_tasks
		WHERE id = $1 AND organization_id = $2
	`
	row := tx.QueryRowContext(ctx, query, id, organizationID)

	task := &SharedTask{}
	var depsBytes, payloadBytes []byte
	err = row.Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
		&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &task.CreatedAt, &task.UpdatedAt,
	)

	if err == sql.ErrNoRows {
		return nil, errors.New("task not found")
	} else if err != nil {
		return nil, err
	}

	if len(depsBytes) > 0 {
		task.Dependencies = json.RawMessage(depsBytes)
	}
	if len(payloadBytes) > 0 {
		raw := json.RawMessage(payloadBytes)
		task.Payload = &raw
	}

	return task, nil
}

func (s *PostgresTaskStore) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	query := `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	_, err := s.db.ExecContext(ctx, query, status, id)
	return err
}

func (s *PostgresTaskStore) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*SharedTask, error) {
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
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
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
		var depsBytes, payloadBytes []byte
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
			&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
		}
		tasks = append(tasks, task)
	}

	return tasks, nil
}

func (s *PostgresTaskStore) PollDelegatedTasks(ctx context.Context, limit int) ([]*SharedTask, error) {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
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
		var depsBytes, payloadBytes []byte
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
			&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
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

func (s *PostgresTaskStore) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	_, err := s.db.ExecContext(ctx, `
		UPDATE agent_missions
		SET status = 'blocked',
		    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '
' END || $1
		WHERE id = $2`, blockers, missionID)
	return err
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

	// Find a pending task with all dependencies completed.
	// Since SQLite doesn't have jsonb_array_elements, we'd need a more complex way or handle it in Go.
	// For simplicity in KAIROS Phase 1, we will implement a simplified check.
	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND organization_id = ?
		LIMIT 100
	`
	rows, err := tx.QueryContext(ctx, query, organizationID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var candidateTask *SharedTask
	var candidateDepsBytes, candidatePayloadBytes []byte
	var candidateCreatedAtStr, candidateUpdatedAtStr string

	for rows.Next() {
		task := &SharedTask{}
		var depsBytes, payloadBytes []byte
		var createdAtStr, updatedAtStr string
		err = rows.Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
			&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &createdAtStr, &updatedAtStr,
		)
		if err != nil {
			return nil, err
		}

		// Check dependencies
		canClaim := true
		if len(depsBytes) > 2 { // Not "[]"
			var deps []string
			if err := json.Unmarshal(depsBytes, &deps); err == nil {
				for _, depID := range deps {
					var depStatus string
					err := tx.QueryRowContext(ctx, "SELECT status FROM shared_tasks WHERE id = ?", depID).Scan(&depStatus)
					if err != nil || depStatus != "COMPLETED" {
						canClaim = false
						break
					}
				}
			}
		}

		if canClaim {
			candidateTask = task
			candidateDepsBytes = depsBytes
			candidatePayloadBytes = payloadBytes
			candidateCreatedAtStr = createdAtStr
			candidateUpdatedAtStr = updatedAtStr
			break
		}
	}

	if candidateTask == nil {
		return nil, nil // No tasks available
	}

	// Simplistic time parsing for SQLite timestamp strings
	if t, err := time.Parse(time.RFC3339, candidateCreatedAtStr); err == nil {
		candidateTask.CreatedAt = t
	} else if t, err := time.Parse("2006-01-02 15:04:05", candidateCreatedAtStr); err == nil {
		candidateTask.CreatedAt = t
	}
	if t, err := time.Parse(time.RFC3339, candidateUpdatedAtStr); err == nil {
		candidateTask.UpdatedAt = t
	} else if t, err := time.Parse("2006-01-02 15:04:05", candidateUpdatedAtStr); err == nil {
		candidateTask.UpdatedAt = t
	}

	if len(candidateDepsBytes) > 0 {
		candidateTask.Dependencies = json.RawMessage(candidateDepsBytes)
	}
	if len(candidatePayloadBytes) > 0 {
		raw := json.RawMessage(candidatePayloadBytes)
		candidateTask.Payload = &raw
	}

	// Update status to ASSIGNED and set assigned_agent_id
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ? AND status = 'PENDING'
	`
	_, err = tx.ExecContext(ctx, updateQuery, agentID, candidateTask.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	candidateTask.Status = "ASSIGNED"
	candidateTask.AssignedAgentID = &agentID

	return candidateTask, nil
}

func (s *SqliteTaskStore) CreateTask(ctx context.Context, task *SharedTask) error {
	s.mutex.Lock()
	defer s.mutex.Unlock()

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `
		INSERT INTO shared_tasks (id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	var depsBytes, payloadBytes []byte
	if len(task.Dependencies) > 0 {
		depsBytes = []byte(task.Dependencies)
	} else {
		depsBytes = []byte("[]")
	}
	if task.Payload != nil {
		payloadBytes = []byte(*task.Payload)
	}

	if task.ID == "" {
		task.ID = "id-" + time.Now().Format("20060102150405.000000")
	}

	if task.Status == "" {
		task.Status = "PENDING"
	}

	_, err = tx.ExecContext(ctx, query,
		task.ID, task.OrganizationID, task.ParentPlanID, task.Title, task.Description, task.Status,
		task.AssignedAgentID, depsBytes, task.Priority, payloadBytes,
	)

	if err != nil {
		return err
	}
	return tx.Commit()
}

func (s *SqliteTaskStore) GetTask(ctx context.Context, id string, organizationID string) (*SharedTask, error) {
	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
		FROM shared_tasks
		WHERE id = ? AND organization_id = ?
	`
	row := s.db.QueryRowContext(ctx, query, id, organizationID)

	task := &SharedTask{}
	var depsBytes, payloadBytes []byte
	var createdAtStr, updatedAtStr string
	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
		&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &createdAtStr, &updatedAtStr,
	)

	if err == sql.ErrNoRows {
		return nil, errors.New("task not found")
	} else if err != nil {
		return nil, err
	}

	if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
		task.CreatedAt = t
	} else if t, err := time.Parse("2006-01-02 15:04:05", createdAtStr); err == nil {
		task.CreatedAt = t
	}
	if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
		task.UpdatedAt = t
	} else if t, err := time.Parse("2006-01-02 15:04:05", updatedAtStr); err == nil {
		task.UpdatedAt = t
	}

	if len(depsBytes) > 0 {
		task.Dependencies = json.RawMessage(depsBytes)
	}
	if len(payloadBytes) > 0 {
		raw := json.RawMessage(payloadBytes)
		task.Payload = &raw
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
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
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
		var depsBytes, payloadBytes []byte
		var createdAtStr, updatedAtStr string
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
			&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &createdAtStr, &updatedAtStr,
		)
		if err != nil {
			return nil, err
		}
		if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
			task.CreatedAt = t
		} else if t, err := time.Parse("2006-01-02 15:04:05", createdAtStr); err == nil {
			task.CreatedAt = t
		}
		if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
			task.UpdatedAt = t
		} else if t, err := time.Parse("2006-01-02 15:04:05", updatedAtStr); err == nil {
			task.UpdatedAt = t
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
		}
		tasks = append(tasks, task)
	}

	return tasks, nil
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
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, priority, payload, created_at, updated_at
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
		var depsBytes, payloadBytes []byte
		var createdAtStr, updatedAtStr string
		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status,
			&task.AssignedAgentID, &depsBytes, &task.Priority, &payloadBytes, &createdAtStr, &updatedAtStr,
		)
		if err != nil {
			return nil, err
		}
		if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
			task.CreatedAt = t
		} else if t, err := time.Parse("2006-01-02 15:04:05", createdAtStr); err == nil {
			task.CreatedAt = t
		}
		if t, err := time.Parse(time.RFC3339, updatedAtStr); err == nil {
			task.UpdatedAt = t
		} else if t, err := time.Parse("2006-01-02 15:04:05", updatedAtStr); err == nil {
			task.UpdatedAt = t
		}
		if len(depsBytes) > 0 {
			task.Dependencies = json.RawMessage(depsBytes)
		}
		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
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

func (s *SqliteTaskStore) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	_, err := s.db.ExecContext(ctx, `
		UPDATE agent_missions
		SET status = 'blocked',
		    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '
' END || ?
		WHERE id = ?`, blockers, missionID)
	return err
}
