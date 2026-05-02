package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"sync"
	"time"
)

type SharedTask struct {
	ID              string          `json:"id" db:"id"`
	OrganizationID  string          `json:"organization_id" db:"organization_id"`
	ParentPlanID    *string         `json:"parent_plan_id" db:"parent_plan_id"`
	Title           string          `json:"title" db:"title"`
	Description     *string         `json:"description" db:"description"`
	Status          string          `json:"status" db:"status"`
	AssignedAgentID *string         `json:"assigned_agent_id" db:"assigned_agent_id"`
	Dependencies    json.RawMessage `json:"dependencies" db:"dependencies"`
	CreatedAt       time.Time       `json:"created_at" db:"created_at"`
	UpdatedAt       time.Time       `json:"updated_at" db:"updated_at"`
}

type TaskDB struct {
	db       *sql.DB
	isSQLite bool
	mu       sync.Mutex
}

func NewTaskDB(db *sql.DB, isSQLite bool) *TaskDB {
	return &TaskDB{
		db:       db,
		isSQLite: isSQLite,
	}
}

func (t *TaskDB) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	if t.isSQLite {
		return t.claimTaskSQLite(ctx, agentID)
	}
	return t.claimTaskPostgres(ctx, agentID)
}

func (t *TaskDB) claimTaskPostgres(ctx context.Context, agentID string) (*SharedTask, error) {
	tx, err := t.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`

	var task SharedTask
	var parentPlanID sql.NullString
	var description sql.NullString
	var assignedAgentID sql.NullString
	var deps []byte

	err = tx.QueryRowContext(ctx, query).Scan(
		&task.ID,
		&task.OrganizationID,
		&parentPlanID,
		&task.Title,
		&description,
		&task.Status,
		&assignedAgentID,
		&deps,
		&task.CreatedAt,
		&task.UpdatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to select task: %w", err)
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
	if deps != nil {
		task.Dependencies = json.RawMessage(deps)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID

	updateQuery := `UPDATE shared_tasks SET status = $1, assigned_agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`
	_, err = tx.ExecContext(ctx, updateQuery, task.Status, task.AssignedAgentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &task, nil
}

func (t *TaskDB) claimTaskSQLite(ctx context.Context, agentID string) (*SharedTask, error) {
	t.mu.Lock()
	defer t.mu.Unlock()

	tx, err := t.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING'
		LIMIT 1
	`

	var task SharedTask
	var parentPlanID sql.NullString
	var description sql.NullString
	var assignedAgentID sql.NullString
	var deps []byte

	err = tx.QueryRowContext(ctx, query).Scan(
		&task.ID,
		&task.OrganizationID,
		&parentPlanID,
		&task.Title,
		&description,
		&task.Status,
		&assignedAgentID,
		&deps,
		&task.CreatedAt,
		&task.UpdatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to select task: %w", err)
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
	if deps != nil {
		task.Dependencies = json.RawMessage(deps)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID

	updateQuery := `UPDATE shared_tasks SET status = ?, assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
	_, err = tx.ExecContext(ctx, updateQuery, task.Status, task.AssignedAgentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &task, nil
}
