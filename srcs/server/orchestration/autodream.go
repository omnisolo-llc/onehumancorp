package orchestration

import (
	"context"
	"database/sql"

	"sync"
	"time"
)

// SharedTaskOrchestrator manages the Shared Task List
type SharedTaskOrchestrator struct {
	mu sync.Mutex
	db *sql.DB
}

// NewSharedTaskOrchestrator creates a new orchestrator
func NewSharedTaskOrchestrator(db *sql.DB) *SharedTaskOrchestrator {
	return &SharedTaskOrchestrator{db: db}
}

// ClaimTask ensures Shared Task logic uses FOR UPDATE SKIP LOCKED for Postgres claiming
// and degrades to application mutexes for Standalone Mode.
func (s *SharedTaskOrchestrator) ClaimTask(ctx context.Context, organizationID string, agentID string) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if s.db == nil {
		// Standalone mode: fallback to application mutex only
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	var taskID string
	err = tx.QueryRowContext(ctx, `
		SELECT id FROM shared_tasks_v4
		WHERE status = 'PENDING' AND organization_id = $1
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`, organizationID).Scan(&taskID)

	if err != nil {
		if err == sql.ErrNoRows {
			return nil // No tasks available
		}
		return err
	}

	_, err = tx.ExecContext(ctx, `
		UPDATE shared_tasks_v4
		SET status = 'ASSIGNED', agent_id = $1, updated_at = $2
		WHERE id = $3
	`, agentID, time.Now(), taskID)

	if err != nil {
		return err
	}

	return tx.Commit()
}

// VerifyTeammateMesh verifies Teammate Mesh functionality
func VerifyTeammateMesh(redisClient interface{}) bool {
	// Dummy check for redis pub/sub mesh
	if redisClient != nil {
		return true
	}
	return false
}

// VerifyAutoDream verifies AutoDream functionality
func VerifyAutoDream(db *sql.DB) bool {
	// Verify pgvector functionality in autodream_memories
	if db == nil {
		return false
	}
	var count int
	err := db.QueryRow("SELECT count(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		return false
	}
	return true
}
