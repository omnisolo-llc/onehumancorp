package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SharedTaskStruct represents the schema in the database.
type SharedTaskStruct struct {
	ID              string
	OrganizationID  string
	ParentPlanID    *string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Dependencies    []string
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type TasksDB struct {
	dbWrapper db.Provider
	mu        sync.Mutex
}

func NewTasksDB(dbWrapper db.Provider) *TasksDB {
	return &TasksDB{
		dbWrapper: dbWrapper,
	}
}

// ClaimTask claims a pending task for the given agentID.
func (t *TasksDB) ClaimTask(ctx context.Context, agentID string) (*SharedTaskStruct, error) {
	if t.dbWrapper.IsSQLite() {
		t.mu.Lock()
		defer t.mu.Unlock()

		tx, err := t.dbWrapper.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		row := tx.QueryRow(ctx, `SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'PENDING' LIMIT 1`)
		return t.processClaim(ctx, tx, row, agentID)
	}

	tx, err := t.dbWrapper.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	row := tx.QueryRow(ctx, `SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED LIMIT 1`)
	return t.processClaim(ctx, tx, row, agentID)
}

func (t *TasksDB) processClaim(ctx context.Context, tx db.Tx, row db.Row, agentID string) (*SharedTaskStruct, error) {
	var task SharedTaskStruct
	var depsJSON []byte

	err := row.Scan(
		&task.ID,
		&task.OrganizationID,
		&task.ParentPlanID,
		&task.Title,
		&task.Description,
		&task.Status,
		&task.AssignedAgentID,
		&depsJSON,
		&task.CreatedAt,
		&task.UpdatedAt,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task found
		}
		return nil, err
	}

	if len(depsJSON) > 0 {
		if err := json.Unmarshal(depsJSON, &task.Dependencies); err != nil {
			return nil, err
		}
	}

	_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, agentID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	return &task, nil
}
