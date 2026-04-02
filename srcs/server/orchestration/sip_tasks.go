package orchestration

import (
	"context"
	"errors"
	"fmt"
	"os"
	"time"

	"github.com/redis/rueidis"
)

// ClaimTask attempts to atomically claim a task for an agent.
func (s *SIPDB) ClaimTask(ctx context.Context, taskID, agentID string) (*Task, error) {
	isStandalone := envBoolDefault("OHC_STANDALONE", false)

	if isStandalone {
		// Wait for local throttle lock
		select {
		case throttleSemaphore <- struct{}{}:
			defer func() { <-throttleSemaphore }()
		case <-ctx.Done():
			return nil, ctx.Err()
		}
	} else {
		// Use Redis Distributed Lock
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" && s.redisClient != nil {
			lockKey := "lock:task:" + taskID
			cmd := s.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Ex(30 * time.Second).Build()
			err := s.redisClient.Do(ctx, cmd).Error()
			if err != nil {
				if err == rueidis.Nil {
					return nil, errors.New("task already locked")
				}
				return nil, fmt.Errorf("redis lock error: %w", err)
			}
		}
	}

	// Now try to update the database state using Optimistic Locking
	// Expected state is status = 'PENDING'
	query := `
		UPDATE swarm_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = $2, updated_at = CURRENT_TIMESTAMP
		WHERE id = $3 AND status = 'PENDING'
		RETURNING id, mission_id, title, status, assigned_agent_id, locked_until, payload, created_at, updated_at
	`

	// Some SQLite drivers might need ? instead of $1, let's use the DB wrapper which we know supports $1 (mostly).
	// Actually we should just execute and see.
	lockedUntil := time.Now().UTC().Add(30 * time.Minute)

	row := s.db.QueryRow(ctx, query, agentID, lockedUntil, taskID)

	var t Task
	var lockedUntilRaw interface{}
	err := row.Scan(&t.ID, &t.MissionID, &t.Title, &t.Status, &t.AssignedAgentID, &lockedUntilRaw, &t.Payload, &t.CreatedAt, &t.UpdatedAt)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			// Fallback with ? just in case the driver is picky
			queryFallback := `
				UPDATE swarm_tasks
				SET status = 'IN_PROGRESS', assigned_agent_id = ?, locked_until = ?, updated_at = CURRENT_TIMESTAMP
				WHERE id = ? AND status = 'PENDING'
				RETURNING id, mission_id, title, status, assigned_agent_id, locked_until, payload, created_at, updated_at
			`
			rowFallback := s.db.QueryRow(ctx, queryFallback, agentID, lockedUntil, taskID)
			errFallback := rowFallback.Scan(&t.ID, &t.MissionID, &t.Title, &t.Status, &t.AssignedAgentID, &lockedUntilRaw, &t.Payload, &t.CreatedAt, &t.UpdatedAt)
			if errFallback != nil {
				if errFallback.Error() == "sql: no rows in result set" {
					return nil, errors.New("task not found, not PENDING, or concurrent modification")
				}
				return nil, fmt.Errorf("failed to claim task (fallback syntax): %w", errFallback)
			}
			t.LockedUntil = parseTimeRobust(lockedUntilRaw)
			return &t, nil
		}
		return nil, fmt.Errorf("failed to claim task: %w", err)
	}

	t.LockedUntil = parseTimeRobust(lockedUntilRaw)

	return &t, nil
}

func parseTimeRobust(val interface{}) time.Time {
	switch v := val.(type) {
	case time.Time:
		return v
	case string:
		t, _ := time.Parse(time.RFC3339, v)
		return t
	default:
		return time.Time{}
	}
}
