package mesh

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"
)

// Mission represents a task in the ohc_tasks.mission_queue
type Mission struct {
	MissionID     string          `json:"mission_id"`
	Title         string          `json:"title"`
	Status        string          `json:"status"`
	AssignedAgent *string         `json:"assigned_agent"`
	Priority      string          `json:"priority"`
	Payload       json.RawMessage `json:"payload"`
	CreatedAt     time.Time       `json:"created_at"`
	UpdatedAt     time.Time       `json:"updated_at"`
}

// ClaimMission attempts to claim a queued mission for the given agent using FOR UPDATE SKIP LOCKED.
func ClaimMission(ctx context.Context, db *sql.DB, agentID string) (*Mission, error) {
	if db == nil {
		return nil, fmt.Errorf("db connection is nil")
	}

	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	query := `
		UPDATE ohc_tasks.mission_queue
		SET status = 'IN_PROGRESS',
		    assigned_agent = $1,
		    updated_at = NOW()
		WHERE mission_id = (
		    SELECT mission_id
		    FROM ohc_tasks.mission_queue
		    WHERE status = 'QUEUED'
		    FOR UPDATE SKIP LOCKED
		    LIMIT 1
		)
		RETURNING mission_id, title, status, assigned_agent, priority, payload, created_at, updated_at
	`

	row := tx.QueryRowContext(ctx, query, agentID)

	var m Mission
	if err := row.Scan(
		&m.MissionID,
		&m.Title,
		&m.Status,
		&m.AssignedAgent,
		&m.Priority,
		&m.Payload,
		&m.CreatedAt,
		&m.UpdatedAt,
	); err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task found
		}
		return nil, fmt.Errorf("failed to scan claimed mission: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &m, nil
}
