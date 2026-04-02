package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
)

// ClaimTask claims a swarm task for an agent.
func (r *SqliteHubRepository) ClaimTask(ctx context.Context, taskID, agentID string) error {
	res, err := r.pool.Exec(ctx, `
		UPDATE swarm_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ? AND status = 'PENDING'
	`, agentID, taskID)

	if err != nil {
		return fmt.Errorf("sqlite: claim task error: %w", err)
	}

	if res == 0 {
		return fmt.Errorf("task %s not available for claiming", taskID)
	}

	return nil
}

// CompleteTask marks a swarm task as completed.
func (r *SqliteHubRepository) CompleteTask(ctx context.Context, taskID string) error {
	res, err := r.pool.Exec(ctx, `
		UPDATE swarm_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = ?
	`, taskID)

	if err != nil {
		return fmt.Errorf("sqlite: complete task error: %w", err)
	}

	if res == 0 {
		return fmt.Errorf("task %s not found", taskID)
	}

	return nil
}

// CreateTask creates a new swarm task.
func (r *SqliteHubRepository) CreateTask(ctx context.Context, missionID, title string, payload map[string]interface{}) (string, error) {
	var taskID string
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	err = r.pool.QueryRow(ctx, `
		INSERT INTO swarm_tasks (mission_id, title, status, payload)
		VALUES (?, ?, 'PENDING', ?)
		RETURNING id
	`, missionID, title, string(payloadBytes)).Scan(&taskID)

	if err != nil {
		return "", fmt.Errorf("sqlite: create task error: %w", err)
	}

	return taskID, nil
}
