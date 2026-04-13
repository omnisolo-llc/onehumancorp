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

	// SQLite DAG: We will fetch all pending tasks for the org and manually check dependencies in application code since SQLite JSON functions can be complex.
	// For simplicity in this demo, let's assume we can just claim the first pending task whose dependencies are either empty or all completed.
	// To strictly follow SQLite, we just query all PENDING tasks and check.
	rows, err := tx.Query(ctx, `
		SELECT id, mission_id, organization_id, dependencies, title, description, status, assigned_agent_id, created_at, updated_at
		FROM swarm_tasks
		WHERE organization_id = $1 AND status = 'PENDING'
	`, orgID)
	if err != nil {
		return nil, err
	}

	var candidates []SwarmTaskDB
	for rows.Next() {
		var task SwarmTaskDB
		if err := rows.Scan(
			&task.ID, &task.MissionID, &task.OrganizationID, &task.Dependencies, &task.Title,
			&task.Description, &task.Status, &task.AssignedAgentID, &task.CreatedAt, &task.UpdatedAt,
		); err != nil {
			rows.Close()
			return nil, err
		}
		candidates = append(candidates, task)
	}
	rows.Close()

	var selectedTask *SwarmTaskDB
	for _, task := range candidates {
		var deps []string
		if task.Dependencies != "" {
			_ = json.Unmarshal([]byte(task.Dependencies), &deps)
		}
		canClaim := true
		for _, depID := range deps {
			var depStatus string
			err := tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", depID).Scan(&depStatus)
			if err != nil || depStatus != "COMPLETED" {
				canClaim = false
				break
			}
		}
		if canClaim {
			selectedTask = &task
			break
		}
	}

	if selectedTask == nil {
		return nil, nil // No task found
	}

	updateQuery := `
		UPDATE swarm_tasks
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
	return selectedTask, nil
}

func (to *SharedTaskOrchestrator) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SwarmTaskDB, error) {
	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Postgres DAG using JSONB array elements
	query := `
		SELECT id, mission_id, organization_id, dependencies::text, title, description, status, assigned_agent_id, created_at, updated_at
		FROM swarm_tasks t
		WHERE organization_id = $1 AND status = 'PENDING'
		AND NOT EXISTS (
			SELECT 1 FROM jsonb_array_elements_text(CASE WHEN t.dependencies IS NULL OR jsonb_typeof(t.dependencies) != 'array' THEN '[]'::jsonb ELSE t.dependencies END) as dep_id
			JOIN swarm_tasks st ON st.id::text = dep_id
			WHERE st.status != 'COMPLETED'
		)
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
		UPDATE swarm_tasks
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
	if err := tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", taskID).Scan(&current); err != nil {
		return fmt.Errorf("failed to fetch task %s: %w", taskID, err)
	}

	if current != fromState {
		return fmt.Errorf("task %s is in state %s, expected %s", taskID, current, fromState)
	}

	// Update state
	if _, err := tx.Exec(ctx, "UPDATE swarm_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", toState, taskID); err != nil {
		return err
	}

	if err := to.insertTransition(ctx, tx, taskID, fromState, toState, agentID, reason); err != nil {
		return err
	}

	return tx.Commit(ctx)
}
