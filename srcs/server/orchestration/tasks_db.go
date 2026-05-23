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
	ID              string
	OrganizationID  string
	ParentPlanID    *string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Dependencies    json.RawMessage
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type Provider interface {
	IsSQLite() bool
}

type DBWrapper interface {
	Provider() Provider
	QueryRowContext(ctx context.Context, query string, args ...interface{}) *sql.Row
	ExecContext(ctx context.Context, query string, args ...interface{}) (sql.Result, error)
	BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error)
}

type DefaultProvider struct {
	isSQLite bool
}

func (p *DefaultProvider) IsSQLite() bool {
	return p.isSQLite
}

type DefaultDBWrapper struct {
	db       *sql.DB
	provider Provider
}

func (w *DefaultDBWrapper) Provider() Provider {
	return w.provider
}

func (w *DefaultDBWrapper) QueryRowContext(ctx context.Context, query string, args ...interface{}) *sql.Row {
	return w.db.QueryRowContext(ctx, query, args...)
}

func (w *DefaultDBWrapper) ExecContext(ctx context.Context, query string, args ...interface{}) (sql.Result, error) {
	return w.db.ExecContext(ctx, query, args...)
}

func (w *DefaultDBWrapper) BeginTx(ctx context.Context, opts *sql.TxOptions) (*sql.Tx, error) {
	return w.db.BeginTx(ctx, opts)
}

type TasksDB struct {
	dbWrapper DBWrapper
	mu        sync.Mutex
}

func NewTasksDB(dbWrapper DBWrapper) *TasksDB {
	return &TasksDB{
		dbWrapper: dbWrapper,
	}
}

func (t *TasksDB) ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error) {
	if t.dbWrapper.Provider().IsSQLite() {
		t.mu.Lock()
		defer t.mu.Unlock()

		tx, err := t.dbWrapper.BeginTx(ctx, nil)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback()

		var task SharedTask
		var desc sql.NullString
		var parentPlanID sql.NullString
		var assignedAgent sql.NullString
		var deps []byte

		query := `SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		          FROM shared_tasks WHERE status = 'PENDING' AND organization_id = $1 LIMIT 1`

		err = tx.QueryRowContext(ctx, query, organizationID).Scan(
			&task.ID, &task.OrganizationID, &parentPlanID, &task.Title, &desc, &task.Status, &assignedAgent, &deps, &task.CreatedAt, &task.UpdatedAt,
		)

		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return nil, nil // No pending task
			}
			return nil, err
		}

		if desc.Valid {
			task.Description = &desc.String
		}
		if parentPlanID.Valid {
			task.ParentPlanID = &parentPlanID.String
		}
		if assignedAgent.Valid {
			task.AssignedAgentID = &assignedAgent.String
		}
		task.Dependencies = deps

		updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.ExecContext(ctx, updateQuery, agentID, task.ID)
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

	} else {
		tx, err := t.dbWrapper.BeginTx(ctx, nil)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback()

		var task SharedTask
		var desc sql.NullString
		var parentPlanID sql.NullString
		var assignedAgent sql.NullString
		var deps []byte

		query := `SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		          FROM shared_tasks WHERE status = 'PENDING' AND organization_id = $1 FOR UPDATE SKIP LOCKED LIMIT 1`

		err = tx.QueryRowContext(ctx, query, organizationID).Scan(
			&task.ID, &task.OrganizationID, &parentPlanID, &task.Title, &desc, &task.Status, &assignedAgent, &deps, &task.CreatedAt, &task.UpdatedAt,
		)

		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return nil, nil // No pending task
			}
			return nil, err
		}

		if desc.Valid {
			task.Description = &desc.String
		}
		if parentPlanID.Valid {
			task.ParentPlanID = &parentPlanID.String
		}
		if assignedAgent.Valid {
			task.AssignedAgentID = &assignedAgent.String
		}
		task.Dependencies = deps

		updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.ExecContext(ctx, updateQuery, agentID, task.ID)
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
}
