package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"time"
)

// Task represents a pending or executing mission in the swarm.
type Task struct {
	ID              string
	MissionID       string
	Title           string
	Status          string // PENDING, IN_PROGRESS, COMPLETED, FAILED
	AssignedAgentID string
	Payload         json.RawMessage
	LockedUntil     time.Time
}

// ClaimTask attempts to atomically claim a task from the global queue.
// It returns true if the task was successfully claimed by the agent.
func (h *Hub) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	sip := h.SIPDB()
	if sip == nil {
		return false, errors.New("SIPDB is not initialized")
	}

	// Throttle standalone database requests.
	if err := acquireThrottle(ctx); err != nil {
		return false, err
	}
	defer releaseThrottle()

	// In cloud mode, Redis locks are often preferred, but since we already
	// have a robust PostgreSQL database attached to SIPDB, we can use
	// standard optimistic/pessimistic concurrency. Let's use a simple UPDATE.
	var rowsAffected int64
	var err error

	// A task can be claimed if it is PENDING, or if its lock has expired.
	now := time.Now().UTC()
	lockExpiry := now.Add(5 * time.Minute).Format("2006-01-02 15:04:05")
	nowStr := now.Format("2006-01-02 15:04:05")

	err = withRetry(ctx, func() error {
		rowsAffected, err = sip.db.Exec(ctx,
			`UPDATE swarm_tasks
			 SET status = 'IN_PROGRESS',
			     assigned_agent_id = ?,
			     locked_until = ?
			 WHERE id = ?
			   AND (status = 'PENDING' OR (status = 'IN_PROGRESS' AND locked_until < ?))`,
			agentID, lockExpiry, taskID, nowStr)
		return err
	})

	if err != nil {
		return false, err
	}

	return rowsAffected > 0, nil
}
