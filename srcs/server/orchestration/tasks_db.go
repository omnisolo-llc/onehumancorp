package orchestration

import (
	"context"
	"errors"
	"fmt"
	"sync"
	"encoding/json"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SwarmTaskDB struct {
	ID              string
	MissionID       string
	OrganizationID  string
	Dependencies    string // JSON string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	CreatedAt       string
	UpdatedAt       string
}

type SharedTaskOrchestrator struct {
	dbProvider db.Provider
	mu         sync.Mutex // For SQLite concurrent assignment locking
}

func NewSharedTaskOrchestrator(dbProvider db.Provider) *SharedTaskOrchestrator {
	return &SharedTaskOrchestrator{
		dbProvider: dbProvider,
	}
}

// ClaimTask attempts to claim a task.
func (to *SharedTaskOrchestrator) ClaimTask(ctx context.Context, agentID string) (*SwarmTaskDB, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	if to.dbProvider.IsSQLite() {
		return to.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
	}
	return to.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (to *SharedTaskOrchestrator) insertTransition(ctx context.Context, tx db.Tx, taskID, from, toState, agentID, reason string) error {
	id := uuid.New().String()
	_, err := tx.Exec(ctx, `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
		VALUES ($1, $2, 'task', $3, $4, $5, $6, CURRENT_TIMESTAMP)
	`, id, taskID, from, toState, agentID, reason)
	return err
}

func (to *SharedTaskOrchestrator) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SwarmTaskDB, error) {
	to.mu.Lock()
	defer to.mu.Unlock()

	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// SQLite DAG: check task_dependencies table
	// We claim the first pending task whose dependencies are either empty or all completed.
	query := `
		SELECT st.id, st.mission_id, st.organization_id, st.dependencies, st.title, st.description, st.status, st.assigned_agent_id, st.created_at, st.updated_at
		FROM shared_tasks st
		WHERE st.organization_id = $1 AND st.status = 'PENDING'
		AND (SELECT COUNT(*) FROM task_dependencies std INNER JOIN shared_tasks d ON std.depends_on_task_id = d.id WHERE std.task_id = st.id AND d.status != 'COMPLETED') = 0
		LIMIT 1
	`
	row := tx.QueryRow(ctx, query, orgID)

	var selectedTask SwarmTaskDB
	if err := row.Scan(
		&selectedTask.ID, &selectedTask.MissionID, &selectedTask.OrganizationID, &selectedTask.Dependencies, &selectedTask.Title,
		&selectedTask.Description, &selectedTask.Status, &selectedTask.AssignedAgentID, &selectedTask.CreatedAt, &selectedTask.UpdatedAt,
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
	if _, err = tx.Exec(ctx, updateQuery, agentID, selectedTask.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := to.insertTransition(ctx, tx, selectedTask.ID, "PENDING", "ASSIGNED", agentID, "Task claimed by agent"); err != nil {
		return nil, fmt.Errorf("failed to insert transition: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	selectedTask.Status = "ASSIGNED"
	selectedTask.AssignedAgentID = &agentID
	return &selectedTask, nil
}

func (to *SharedTaskOrchestrator) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SwarmTaskDB, error) {
	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Postgres DAG using task_dependencies with SKIP LOCKED
	query := `
		SELECT st.id, st.mission_id, st.organization_id, st.dependencies::text, st.title, st.description, st.status, st.assigned_agent_id, st.created_at, st.updated_at
		FROM shared_tasks st
		WHERE st.organization_id = $1 AND st.status = 'PENDING'
		AND (SELECT COUNT(*) FROM task_dependencies std INNER JOIN shared_tasks d ON std.depends_on_task_id = d.id WHERE std.task_id = st.id AND d.status != 'COMPLETED') = 0
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`
	row := tx.QueryRow(ctx, query, orgID)

	task := &SwarmTaskDB{}
	if err := row.Scan(
		&task.ID, &task.MissionID, &task.OrganizationID, &task.Dependencies, &task.Title,
		&task.Description, &task.Status, &task.AssignedAgentID, &task.CreatedAt, &task.UpdatedAt,
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
	if _, err = tx.Exec(ctx, updateQuery, agentID, task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := to.insertTransition(ctx, tx, task.ID, "PENDING", "ASSIGNED", agentID, "Task claimed by agent"); err != nil {
		return nil, fmt.Errorf("failed to insert transition: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	return task, nil
}

// TransitionTask allows moving a task between valid states
func (to *SharedTaskOrchestrator) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Validate current state
	var current string
	if err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", taskID).Scan(&current); err != nil {
		return fmt.Errorf("failed to fetch task %s: %w", taskID, err)
	}

	if current != fromState {
		return fmt.Errorf("task %s is in state %s, expected %s", taskID, current, fromState)
	}

	// Update state
	if _, err := tx.Exec(ctx, "UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", toState, taskID); err != nil {
		return err
	}

	if err := to.insertTransition(ctx, tx, taskID, fromState, toState, agentID, reason); err != nil {
		return err
	}

	return tx.Commit(ctx)
}
