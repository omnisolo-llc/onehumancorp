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

type DBProvider interface {
	IsSQLite() bool
	ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
	QueryRowContext(ctx context.Context, query string, args ...any) RowScanner
}

type RowScanner interface {
	Scan(dest ...any) error
}

// Ensure sql.Row implements RowScanner (it does implicitly)

type TasksDB struct {
	db DBProvider
	mu sync.Mutex
}

func NewTasksDB(db DBProvider) *TasksDB {
	return &TasksDB{
		db: db,
	}
}

func (to *TasksDB) ClaimTask(ctx context.Context, orgID, agentID string) (*SharedTask, error) {
	if to.db.IsSQLite() {
		to.mu.Lock()
		defer to.mu.Unlock()

		row := to.db.QueryRowContext(ctx, "SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'PENDING' AND organization_id = $1 LIMIT 1", orgID)

		task := &SharedTask{}
		err := row.Scan(&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return nil, nil // No task found
			}
			return nil, err
		}

		_, err = to.db.ExecContext(ctx, "UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID)
		if err != nil {
			return nil, err
		}

		task.Status = "ASSIGNED"
		task.AssignedAgentID = &agentID
		return task, nil

	} else {
		row := to.db.QueryRowContext(ctx, "SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'PENDING' AND organization_id = $1 FOR UPDATE SKIP LOCKED LIMIT 1", orgID)

		task := &SharedTask{}
		err := row.Scan(&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return nil, nil // No task found
			}
			return nil, err
		}

		_, err = to.db.ExecContext(ctx, "UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID)
		if err != nil {
			return nil, err
		}

		task.Status = "ASSIGNED"
		task.AssignedAgentID = &agentID
		return task, nil
	}
}
