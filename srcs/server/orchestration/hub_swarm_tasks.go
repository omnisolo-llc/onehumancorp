package orchestration

import (
	"context"
	"fmt"
	"time"
)

// ClaimTask attempts to claim a task. It uses rueidis for distributed locking in cloud mode,
// and delegates to the underlying repository for the DB-level claim.
func (h *Hub) ClaimTask(ctx context.Context, taskID, agentID string) error {
	h.mu.RLock()
	client := h.redisClient
	h.mu.RUnlock()

	if client != nil {
		// Use distributed lock. We only attempt to acquire the lock and if it succeeds we proceed to DB update.
		// If it fails, another agent is actively processing this.
		lockKey := "lock:task:" + taskID

		cmd := client.B().Set().Key(lockKey).Value(agentID).Nx().Ex(30 * time.Second).Build()
		res := client.Do(ctx, cmd)
		if res.Error() != nil {
			return fmt.Errorf("could not acquire lock for task %s: %w", taskID, res.Error())
		}
	}

	if h.repo != nil {
		err := h.repo.ClaimTask(ctx, taskID, agentID)
		if err != nil && client != nil {
			// If claim fails in the DB (e.g., someone else claimed it before our lock, or it's not pending), release the lock.
			delCmd := client.B().Del().Key("lock:task:" + taskID).Build()
			_ = client.Do(context.Background(), delCmd)
		}
		return err
	}

	return fmt.Errorf("no repository configured")
}

// CompleteTask marks a task as completed.
func (h *Hub) CompleteTask(ctx context.Context, taskID string) error {
	if h.repo != nil {
		return h.repo.CompleteTask(ctx, taskID)
	}
	return fmt.Errorf("no repository configured")
}

// CreateTask creates a new task.
func (h *Hub) CreateTask(ctx context.Context, missionID, title string, payload map[string]interface{}) (string, error) {
	if h.repo != nil {
		return h.repo.CreateTask(ctx, missionID, title, payload)
	}
	return "", fmt.Errorf("no repository configured")
}
