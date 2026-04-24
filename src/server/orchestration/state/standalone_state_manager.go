package state

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type StandaloneStateManager struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewStandaloneStateManager(provider db.Provider) *StandaloneStateManager {
	return &StandaloneStateManager{
		dbProvider: provider,
	}
}

func (m *StandaloneStateManager) TransitionState(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	tx, err := m.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var currentStatus string
	err = tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", taskID).Scan(&currentStatus)
	if err != nil {
		return fmt.Errorf("task not found: %w", err)
	}

	if currentStatus != fromState {
		return fmt.Errorf("task is not in expected state %s, actual state: %s", fromState, currentStatus)
	}

	// Enforce DAG: if toState is EXECUTING, parents must be COMPLETED
	if toState == "EXECUTING" {
		var depsStr string
		err = tx.QueryRow(ctx, "SELECT dependencies FROM swarm_tasks WHERE id = $1", taskID).Scan(&depsStr)
		if err != nil {
			return err
		}

		var deps []string
		if err := json.Unmarshal([]byte(depsStr), &deps); err != nil {
			return fmt.Errorf("failed to parse dependencies: %w", err)
		}

		if len(deps) > 0 {
			for _, depID := range deps {
				var depStatus string
				err = tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", depID).Scan(&depStatus)
				if err != nil {
					return fmt.Errorf("failed to check dependency %s: %w", depID, err)
				}
				if depStatus != "COMPLETED" {
					return fmt.Errorf("dependency %s is not COMPLETED (current: %s)", depID, depStatus)
				}
			}
		}
	}

	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = $1 WHERE id = $2", toState, taskID)
	if err != nil {
		return err
	}

	_, err = tx.Exec(ctx,
		"INSERT INTO state_machine_transitions (entity_id, entity_type, from_state, to_state, agent_id, reason) VALUES ($1, $2, $3, $4, $5, $6)",
		taskID, "task", fromState, toState, agentID, reason,
	)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (m *StandaloneStateManager) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	tx, err := m.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task Task
	var depsStr string
	err = tx.QueryRow(ctx, `
		SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id
		FROM swarm_tasks
		WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
		LIMIT 1
	`).Scan(&task.ID, &task.MissionID, &task.ParentPlanID, &depsStr, &task.Title, &task.Status, &task.AssignedAgentID)

	if err != nil {
		return nil, err
	}

	_ = json.Unmarshal([]byte(depsStr), &task.Dependencies)

	lockedUntil := time.Now().Add(5 * time.Minute)
	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET assigned_agent_id = $1, locked_until = $2 WHERE id = $3", agentID, lockedUntil, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.AssignedAgentID = &agentID

	return &task, nil
}

func (m *StandaloneStateManager) MarkTaskCompleted(ctx context.Context, taskID string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	tx, err := m.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED', locked_until = NULL WHERE id = $1", taskID)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (m *StandaloneStateManager) GetTaskStatus(ctx context.Context, taskID string) (string, error) {
	var status string
	err := m.dbProvider.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", taskID).Scan(&status)
	if err != nil {
		return "", err
	}
	return status, nil
}
