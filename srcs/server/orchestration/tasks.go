package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"time"
)

// Task represents a shared task in the swarm.
type Task struct {
	ID              string
	MissionID       string
	Title           string
	Status          string
	AssignedAgentID string
	LockedUntil     *time.Time
	Payload         json.RawMessage
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// ClaimTask attempts to claim a pending task for the given agent using a distributed lock.
// It falls back to standard row locking if SQLite is used.
func (h *Hub) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	if h.sipDB == nil || h.sipDB.db == nil {
		return false, errors.New("SIPDB not initialized")
	}

	// This is a simplified lock mechanism leveraging the database itself.
	// For OHC_MULTITENANT, Redis distributed lock is typically preferred,
	// but here we use the DB as standard fallback (SQLite or Postgres).
	db := h.sipDB.db

	tx, err := db.Begin(ctx)
	if err != nil {
		return false, err
	}
	defer tx.Rollback(ctx)

	// Using optimistic locking by shifting the state check to the UPDATE statement
	// instead of doing SELECT ... FOR UPDATE (which might not be supported uniformly)

	newLock := time.Now().Add(30 * time.Minute)

	// Try to update only if PENDING, FAILED, or lock expired
	updateQuery := `
		UPDATE swarm_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = $2, updated_at = $3
		WHERE id = $4
		  AND (status IN ('PENDING', 'FAILED') OR (status = 'IN_PROGRESS' AND locked_until < $5))
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, agentID, newLock, time.Now(), taskID, time.Now())
	if err != nil {
		return false, err
	}

	if rowsAffected == 0 {
		return false, nil // Could not update, already claimed by someone else or doesn't exist
	}

	if err := tx.Commit(ctx); err != nil {
		return false, err
	}

	return true, nil
}
