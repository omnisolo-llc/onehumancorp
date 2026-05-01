package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"

	"github.com/onehumancorp/ohc/srcs/server/domain"
)

type DBProvider interface {
	IsSQLite() bool
	DB() *sql.DB
}

type TasksDB struct {
	provider DBProvider
	mu       sync.Mutex
}

func NewTasksDB(provider DBProvider) *TasksDB {
	return &TasksDB{
		provider: provider,
	}
}

func (to *TasksDB) ClaimTask(ctx context.Context, agentID string) (*domain.SharedTask, error) {
	if to.provider.IsSQLite() {
		return to.claimTaskSQLite(ctx, agentID)
	}
	return to.claimTaskPostgres(ctx, agentID)
}

func (to *TasksDB) claimTaskPostgres(ctx context.Context, agentID string) (*domain.SharedTask, error) {
	db := to.provider.DB()
	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	row := tx.QueryRowContext(ctx, query)
	var task domain.SharedTask
	var deps []byte

	err = row.Scan(&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &deps, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, err
	}

	if len(deps) > 0 {
		var dependencies []string
		if err := json.Unmarshal(deps, &dependencies); err != nil {
			return nil, err
		}
		task.Dependencies = dependencies
	}

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

	return &task, nil
}

func (to *TasksDB) claimTaskSQLite(ctx context.Context, agentID string) (*domain.SharedTask, error) {
	to.mu.Lock()
	defer to.mu.Unlock()

	db := to.provider.DB()
	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING'
		LIMIT 1
	`
	row := tx.QueryRowContext(ctx, query)
	var task domain.SharedTask
	var deps []byte

	err = row.Scan(&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &deps, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, err
	}

	if len(deps) > 0 {
		var dependencies []string
		if err := json.Unmarshal(deps, &dependencies); err != nil {
			return nil, err
		}
		task.Dependencies = dependencies
	}

	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ?
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

	return &task, nil
}
