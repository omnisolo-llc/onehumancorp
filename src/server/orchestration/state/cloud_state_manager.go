package state

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/redis/rueidis"
)

type CloudStateManager struct {
	dbProvider  db.Provider
	redisClient rueidis.Client
}

func NewCloudStateManager(provider db.Provider, redisClient rueidis.Client) *CloudStateManager {
	return &CloudStateManager{
		dbProvider:  provider,
		redisClient: redisClient,
	}
}

func (m *CloudStateManager) acquireLock(ctx context.Context, taskID string) (func(), error) {
	if m.redisClient == nil {
		return func() {}, nil
	}

	lockKey := "task_lock:" + taskID
	// Attempt to acquire Redis distributed lock with 30 second expiry
	cmd := m.redisClient.B().Set().Key(lockKey).Value("locked").Nx().Ex(30*time.Second).Build()
	err := m.redisClient.Do(ctx, cmd).Error()
	if err != nil {
		if err == rueidis.Nil {
			return nil, fmt.Errorf("failed to acquire redis lock for task %s: lock already held", taskID)
		}
		return nil, fmt.Errorf("failed to acquire redis lock for task %s: %w", taskID, err)
	}

	release := func() {
		delCmd := m.redisClient.B().Del().Key(lockKey).Build()
		_ = m.redisClient.Do(ctx, delCmd).Error()
	}

	return release, nil
}


func (m *CloudStateManager) TransitionState(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	release, err := m.acquireLock(ctx, taskID)
	if err != nil {
		return err
	}
	defer release()

	tx, err := m.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// In cloud mode, use FOR UPDATE to lock row
	var currentStatus string
	query := "SELECT status FROM swarm_tasks WHERE id = $1 FOR UPDATE"
	if m.dbProvider.IsSQLite() {
		query = "SELECT status FROM swarm_tasks WHERE id = $1"
	}
	err = tx.QueryRow(ctx, query, taskID).Scan(&currentStatus)
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

func (m *CloudStateManager) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	tx, err := m.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	// Fetch a pending task using SKIP LOCKED
	var task Task
	var depsStr string
	query := `
		SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id
		FROM swarm_tasks
		WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
		LIMIT 1 FOR UPDATE SKIP LOCKED
	`
	if m.dbProvider.IsSQLite() {
		query = `
			SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id
			FROM swarm_tasks
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			LIMIT 1
		`
	}
	err = tx.QueryRow(ctx, query).Scan(&task.ID, &task.MissionID, &task.ParentPlanID, &depsStr, &task.Title, &task.Status, &task.AssignedAgentID)

	if err != nil {
		return nil, err
	}

	_ = json.Unmarshal([]byte(depsStr), &task.Dependencies)

	// Mark as locked
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

func (m *CloudStateManager) MarkTaskCompleted(ctx context.Context, taskID string) error {
	release, err := m.acquireLock(ctx, taskID)
	if err != nil {
		return err
	}
	defer release()

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

func (m *CloudStateManager) GetTaskStatus(ctx context.Context, taskID string) (string, error) {
	var status string
	err := m.dbProvider.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1", taskID).Scan(&status)
	if err != nil {
		return "", err
	}
	return status, nil
}
