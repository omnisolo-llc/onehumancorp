package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"
)

// DbWrapper is a placeholder interface for database operations
// to mock the actual db wrapper used in OHC since we are implementing a single layer.
type DbWrapper interface {
	IsSQLite() bool
	ExecContext(ctx context.Context, query string, args ...interface{}) (sql.Result, error)
	QueryRowContext(ctx context.Context, query string, args ...interface{}) *sql.Row
	BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error)
}

// Task represents a shared task
type Task struct {
	ID              string          `json:"id"`
	OrganizationID  string          `json:"organization_id"`
	ParentPlanID    *string         `json:"parent_plan_id"`
	Title           string          `json:"title"`
	Description     *string         `json:"description"`
	Status          string          `json:"status"`
	AssignedAgentID *string         `json:"assigned_agent_id"`
	Dependencies    json.RawMessage `json:"dependencies"`
	CreatedAt       time.Time       `json:"created_at"`
	UpdatedAt       time.Time       `json:"updated_at"`
}

// TasksDB handles data access for shared tasks
type TasksDB struct {
	db DbWrapper
	mu sync.Mutex // Mutex for SQLite standalone mode
}

// NewTasksDB creates a new TasksDB
func NewTasksDB(db DbWrapper) *TasksDB {
	return &TasksDB{
		db: db,
	}
}

// ClaimTask claims a pending task for the given agent
func (t *TasksDB) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	if t.db.IsSQLite() {
		return t.claimTaskSQLite(ctx, agentID)
	}
	return t.claimTaskPostgres(ctx, agentID)
}

func (t *TasksDB) claimTaskPostgres(ctx context.Context, agentID string) (*Task, error) {
	tx, err := t.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var task Task
	var parentPlanID sql.NullString
	var description sql.NullString
	var assignedAgentID sql.NullString
	var dependencies []byte

	err = tx.QueryRowContext(ctx, `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`).Scan(
		&task.ID,
		&task.OrganizationID,
		&parentPlanID,
		&task.Title,
		&description,
		&task.Status,
		&assignedAgentID,
		&dependencies,
		&task.CreatedAt,
		&task.UpdatedAt,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, err
	}

	if parentPlanID.Valid {
		task.ParentPlanID = &parentPlanID.String
	}
	if description.Valid {
		task.Description = &description.String
	}
	if assignedAgentID.Valid {
		task.AssignedAgentID = &assignedAgentID.String
	}
	if dependencies != nil {
		task.Dependencies = json.RawMessage(dependencies)
	}

	_, err = tx.ExecContext(ctx, `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`, agentID, task.ID)
	if err != nil {
		return nil, err
	}

	err = tx.Commit()
	if err != nil {
		return nil, err
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID

	return &task, nil
}

func (t *TasksDB) claimTaskSQLite(ctx context.Context, agentID string) (*Task, error) {
	t.mu.Lock()
	defer t.mu.Unlock()

	tx, err := t.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var task Task
	var parentPlanID sql.NullString
	var description sql.NullString
	var assignedAgentID sql.NullString
	var dependencies []byte
	var createdAt string
	var updatedAt string

	err = tx.QueryRowContext(ctx, `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING'
		LIMIT 1
	`).Scan(
		&task.ID,
		&task.OrganizationID,
		&parentPlanID,
		&task.Title,
		&description,
		&task.Status,
		&assignedAgentID,
		&dependencies,
		&createdAt,
		&updatedAt,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, err
	}

	if parentPlanID.Valid {
		task.ParentPlanID = &parentPlanID.String
	}
	if description.Valid {
		task.Description = &description.String
	}
	if assignedAgentID.Valid {
		task.AssignedAgentID = &assignedAgentID.String
	}
	if dependencies != nil {
		task.Dependencies = json.RawMessage(dependencies)
	}

	// Parse timestamps
	if t, err := time.Parse(time.RFC3339, createdAt); err == nil {
		task.CreatedAt = t
	}
	if t, err := time.Parse(time.RFC3339, updatedAt); err == nil {
		task.UpdatedAt = t
	}

	_, err = tx.ExecContext(ctx, `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`, agentID, task.ID)
	if err != nil {
		return nil, err
	}

	err = tx.Commit()
	if err != nil {
		return nil, err
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID

	return &task, nil
}
