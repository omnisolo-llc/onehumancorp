package orchestration

import (
	"context"
	"errors"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTaskDB struct {
	ID              string
	OrganizationID  string
	ParentPlanID    *string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Dependencies    *string // JSON string
	CreatedAt       string
	UpdatedAt       string
}

type SharedTaskRepository struct {
	dbProvider db.Provider
	mu         sync.Mutex // For SQLite concurrent assignment locking
}

func NewSharedTaskRepository(dbProvider db.Provider) *SharedTaskRepository {
	return &SharedTaskRepository{
		dbProvider: dbProvider,
	}
}

// ClaimTask attempts to claim a task. Returns the task ID and an error if one occurred.
func (r *SharedTaskRepository) ClaimTask(ctx context.Context, agentID string) (*SharedTaskDB, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	if r.dbProvider.IsSQLite() {
		return r.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
	}
	return r.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (r *SharedTaskRepository) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SharedTaskDB, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// In SQLite we use a simple SELECT then UPDATE in a transaction, protected by application mutex
	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE organization_id = $1 AND status = 'PENDING'
		LIMIT 1
	`
	row := tx.QueryRow(ctx, query, orgID)

	task := &SharedTaskDB{}
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
		&task.Description, &task.Status, &task.AssignedAgentID,
		&task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
	); err != nil {
		// Could be sql.ErrNoRows or pgx.ErrNoRows. We handle it generally
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil // No task found
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	return task, nil
}

func (r *SharedTaskRepository) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SharedTaskDB, error) {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// In Postgres we can use FOR UPDATE SKIP LOCKED
	query := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE organization_id = $1 AND status = 'PENDING'
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`
	row := tx.QueryRow(ctx, query, orgID)

	task := &SharedTaskDB{}
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
		&task.Description, &task.Status, &task.AssignedAgentID,
		&task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
	); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil // No task found
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	return task, nil
}
