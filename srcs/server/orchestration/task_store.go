package orchestration

import (
	"context"
	"errors"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type DecompositionTaskStore struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewDecompositionTaskStore(dbProvider db.Provider) *DecompositionTaskStore {
	return &DecompositionTaskStore{
		dbProvider: dbProvider,
	}
}

func (s *DecompositionTaskStore) ClaimTask(ctx context.Context, agentID string) (*SharedTaskDecomposition, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	if s.dbProvider.IsSQLite() {
		return s.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
	}
	return s.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (s *DecompositionTaskStore) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SharedTaskDecomposition, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
        SELECT st.id, st.organization_id, st.title, st.description, st.status, st.assigned_agent_id, st.priority, st.payload, st.parent_plan_id, st.dependencies, st.locked_until, st.created_at, st.updated_at
        FROM shared_tasks_decomposition st
        WHERE st.status = 'PENDING' AND st.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM json_each(st.dependencies) AS dep
            JOIN shared_tasks_decomposition d ON d.id = dep.value
            WHERE d.status != 'DONE'
        )
        LIMIT 1
    `
	row := tx.QueryRow(ctx, query, orgID)

	var task SharedTaskDecomposition
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description,
		&task.Status, &task.AssignedAgentID, &task.Priority, &task.Payload,
		&task.ParentPlanID, &task.Dependencies, &task.LockedUntil,
		&task.CreatedAt, &task.UpdatedAt,
	); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = &agentID
	return &task, nil
}

func (s *DecompositionTaskStore) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SharedTaskDecomposition, error) {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
        SELECT st.id, st.organization_id, st.title, st.description, st.status, st.assigned_agent_id, st.priority, st.payload, st.parent_plan_id, st.dependencies, st.locked_until, st.created_at, st.updated_at
        FROM shared_tasks_decomposition st
        WHERE st.status = 'PENDING' AND st.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep
            JOIN shared_tasks_decomposition d ON d.id = dep::uuid
            WHERE d.status != 'DONE'
        )
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
	row := tx.QueryRow(ctx, query, orgID)

	var task SharedTaskDecomposition
	if err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description,
		&task.Status, &task.AssignedAgentID, &task.Priority, &task.Payload,
		&task.ParentPlanID, &task.Dependencies, &task.LockedUntil,
		&task.CreatedAt, &task.UpdatedAt,
	); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = &agentID
	return &task, nil
}
