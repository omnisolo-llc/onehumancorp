package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
)

type SwarmTask struct {
	ID              string
	MissionID       string
	Title           string
	Status          string
	AssignedAgentID string
	LockedUntil     time.Time
	Payload         map[string]interface{}
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type SwarmLongTermMemory struct {
	ID        string
	Topic     string
	Summary   string
	Embedding []float32
	CreatedAt time.Time
}

// Ensure Hub can interact with Swarm Tasks

func (r *PgHubRepository) ClaimTask(ctx context.Context, taskID, agentID string) error {
	return pgWithRetry(ctx, func() error {
		// Use a simple atomic UPDATE to claim the task if it's PENDING or lock has expired.
		// Since we also have distributed locks, this acts as the DB-level state transition.
		res, err := r.pool.Exec(ctx, `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = $2 AND status = 'PENDING'
		`, agentID, taskID)

		if err != nil {
			return fmt.Errorf("pg: claim task error: %w", err)
		}

		if res == 0 {
			return fmt.Errorf("task %s not available for claiming", taskID)
		}

		return nil
	})
}

func (r *PgHubRepository) CompleteTask(ctx context.Context, taskID string) error {
	return pgWithRetry(ctx, func() error {
		res, err := r.pool.Exec(ctx, `
			UPDATE swarm_tasks
			SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
			WHERE id = $1
		`, taskID)

		if err != nil {
			return fmt.Errorf("pg: complete task error: %w", err)
		}

		if res == 0 {
			return fmt.Errorf("task %s not found", taskID)
		}

		return nil
	})
}

func (r *PgHubRepository) CreateTask(ctx context.Context, missionID, title string, payload map[string]interface{}) (string, error) {
	var taskID string
	err := pgWithRetry(ctx, func() error {
		payloadBytes, err := json.Marshal(payload)
		if err != nil {
			return err
		}

		err = r.pool.QueryRow(ctx, `
			INSERT INTO swarm_tasks (mission_id, title, status, payload)
			VALUES ($1, $2, 'PENDING', $3)
			RETURNING id
		`, missionID, title, string(payloadBytes)).Scan(&taskID)

		if err != nil {
			return fmt.Errorf("pg: create task error: %w", err)
		}
		return nil
	})

	return taskID, err
}
