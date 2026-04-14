package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type DecompositionTaskOrchestrator struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewDecompositionTaskOrchestrator(dbProvider db.Provider) *DecompositionTaskOrchestrator {
	return &DecompositionTaskOrchestrator{
		dbProvider: dbProvider,
	}
}

func (to *DecompositionTaskOrchestrator) ClaimTask(ctx context.Context, agentID string) (*SharedTaskDecomposition, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	if to.dbProvider.IsSQLite() {
		return to.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
	}
	return to.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (to *DecompositionTaskOrchestrator) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SharedTaskDecomposition, error) {
	to.mu.Lock()
	defer to.mu.Unlock()

	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	row := tx.QueryRow(ctx, "SELECT id, title, status FROM shared_tasks_decomposition WHERE status = 'PENDING' AND organization_id = ? LIMIT 1", orgID)
	var task SharedTaskDecomposition
	err = row.Scan(&task.ID, &task.Title, &task.Status)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // gracefully return nil if no tasks left
		}
		return nil, fmt.Errorf("failed to scan task: %w", err)
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = ? WHERE id = ? AND organization_id = ?", agentID, task.ID, orgID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	err = tx.Commit(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to commit: %w", err)
	}

	return &task, nil
}

func (to *DecompositionTaskOrchestrator) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SharedTaskDecomposition, error) {
	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	row := tx.QueryRow(ctx, "SELECT id, title, status FROM shared_tasks_decomposition WHERE status = 'PENDING' AND organization_id = $1 FOR UPDATE SKIP LOCKED LIMIT 1", orgID)
	var task SharedTaskDecomposition
	err = row.Scan(&task.ID, &task.Title, &task.Status)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" {
			return nil, nil // gracefully return nil if no tasks left
		}
		return nil, fmt.Errorf("failed to scan task: %w", err)
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1 WHERE id = $2 AND organization_id = $3", agentID, task.ID, orgID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	err = tx.Commit(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to commit: %w", err)
	}

	return &task, nil
}

func (to *DecompositionTaskOrchestrator) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	var err error
	if to.dbProvider.IsSQLite() {
		to.mu.Lock()
		defer to.mu.Unlock()
		_, err = to.dbProvider.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = ? WHERE id = ? AND organization_id = ? AND assigned_agent_id = ? AND status = ?", toState, taskID, claims.OrganizationID, agentID, fromState)
	} else {
		_, err = to.dbProvider.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = $1 WHERE id = $2 AND organization_id = $3 AND assigned_agent_id = $4 AND status = $5", toState, taskID, claims.OrganizationID, agentID, fromState)
	}

	if err != nil {
		return fmt.Errorf("failed to transition task: %w", err)
	}
	return nil
}
