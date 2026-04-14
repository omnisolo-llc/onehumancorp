package repository

import (
	"context"
	"errors"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTaskV4 struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Priority        string
	Payload         *string
	LockedUntil     *string
	ParentPlanID    *string
	Dependencies    string
	CreatedAt       string
	UpdatedAt       string
}

type TaskRepository interface {
	ClaimTask(ctx context.Context, agentID string) (*SharedTaskV4, error)
}

type taskRepositoryImpl struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewTaskRepository(dbProvider db.Provider) TaskRepository {
	return &taskRepositoryImpl{
		dbProvider: dbProvider,
	}
}

func (r *taskRepositoryImpl) ClaimTask(ctx context.Context, agentID string) (*SharedTaskV4, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	if r.dbProvider.IsSQLite() {
		return r.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
	}
	return r.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (r *taskRepositoryImpl) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SharedTaskV4, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
        SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
        FROM shared_tasks_v4
        WHERE status = 'PENDING' AND organization_id = $1
        LIMIT 1
    `
	row := tx.QueryRow(ctx, query, orgID)

	var task SharedTaskV4
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description,
		&task.Status, &task.AssignedAgentID, &task.Priority, &task.Payload,
		&task.ParentPlanID, &task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
	); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if _, err = tx.Exec(ctx, "UPDATE shared_tasks_v4 SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	return &task, nil
}

func (r *taskRepositoryImpl) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SharedTaskV4, error) {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
        SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
        FROM shared_tasks_v4
        WHERE status = 'PENDING' AND organization_id = $1
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
	row := tx.QueryRow(ctx, query, orgID)

	var task SharedTaskV4
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description,
		&task.Status, &task.AssignedAgentID, &task.Priority, &task.Payload,
		&task.ParentPlanID, &task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
	); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if _, err = tx.Exec(ctx, "UPDATE shared_tasks_v4 SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	return &task, nil
}
