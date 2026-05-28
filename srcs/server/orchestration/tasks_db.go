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

type TasksDB struct {
	db       *sql.DB
	isSQLite bool
	mu       sync.Mutex
}

func NewTasksDB(db *sql.DB, isSQLite bool) *TasksDB {
	return &TasksDB{
		db:       db,
		isSQLite: isSQLite,
	}
}

func (to *TasksDB) ClaimTask(ctx context.Context, organizationID, agentID string) (*SharedTask, error) {
	if to.isSQLite {
		to.mu.Lock()
		defer to.mu.Unlock()
	}

	tx, err := to.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback()

	var query string
	if to.isSQLite {
		query = `
			SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'PENDING' AND organization_id = ?
			LIMIT 1
		`
	} else {
		query = `
			SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'PENDING' AND organization_id = $1
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		`
	}

	row := tx.QueryRowContext(ctx, query, organizationID)

	var task SharedTask
	var dependencies []byte
	var createdAt, updatedAt string

	if to.isSQLite {
		err = row.Scan(
			&task.ID,
			&task.OrganizationID,
			&task.ParentPlanID,
			&task.Title,
			&task.Description,
			&task.Status,
			&task.AssignedAgentID,
			&dependencies,
			&createdAt,
			&updatedAt,
		)
		if err == nil {
			task.CreatedAt, _ = time.Parse("2006-01-02 15:04:05-07:00", createdAt)
			task.UpdatedAt, _ = time.Parse("2006-01-02 15:04:05-07:00", updatedAt)
		}
	} else {
		err = row.Scan(
			&task.ID,
			&task.OrganizationID,
			&task.ParentPlanID,
			&task.Title,
			&task.Description,
			&task.Status,
			&task.AssignedAgentID,
			&dependencies,
			&task.CreatedAt,
			&task.UpdatedAt,
		)
	}

	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No pending task found
		}
		return nil, fmt.Errorf("failed to claim task: %w", err)
	}

	task.Dependencies = dependencies

	now := time.Now()
	var updateQuery string
	if to.isSQLite {
		updateQuery = `
			UPDATE shared_tasks
			SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = ?
			WHERE id = ?
		`
	} else {
		updateQuery = `
			UPDATE shared_tasks
			SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = $2
			WHERE id = $3
		`
	}

	_, err = tx.ExecContext(ctx, updateQuery, agentID, now, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit tx: %w", err)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	task.UpdatedAt = now

	return &task, nil
}
