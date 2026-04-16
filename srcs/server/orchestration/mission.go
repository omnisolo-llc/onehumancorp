package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

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

// ClaimMission executes the SELECT ... FOR UPDATE SKIP LOCKED query to claim a mission
// from ohc_tasks.mission_queue. It returns the claimed mission.
func ClaimMission(ctx context.Context, database db.Provider, agentID string) (*Mission, error) {
	tx, err := database.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var missionID string
	var selectQuery string

	if database.IsSQLite() {
		// SQLite fallback. Use UPDATE ... RETURNING with returning syntax combined to avoid race condition or use IMMEDIATE tx if possible.
		// Wait, database/sql driver might just use RETURNING. Let's do a trick using UPDATE ... RETURNING directly for SQLite 3.35+.
		selectQuery = `
			UPDATE mission_queue
			SET status = 'IN_PROGRESS', assigned_agent = 'worker-1', updated_at = CURRENT_TIMESTAMP
			WHERE mission_id = (SELECT mission_id FROM mission_queue WHERE status = 'QUEUED' LIMIT 1)
			RETURNING mission_id
		`
		// Hmm, actually since we must return title, priority, etc, we can use update returning:
		updateQuery := `
			UPDATE mission_queue
			SET status = 'IN_PROGRESS', assigned_agent = ?, updated_at = CURRENT_TIMESTAMP
			WHERE mission_id = (SELECT mission_id FROM mission_queue WHERE status = 'QUEUED' LIMIT 1)
			RETURNING mission_id, title, priority, payload, created_at, updated_at
		`
		var m Mission
		err = tx.QueryRow(ctx, updateQuery, agentID).Scan(&m.MissionID, &m.Title, &m.Priority, &m.Payload, &m.CreatedAt, &m.UpdatedAt)
		if err != nil {
			return nil, err
		}
		m.Status = "IN_PROGRESS"
		m.AssignedAgent = &agentID

		if err := tx.Commit(ctx); err != nil {
			return nil, fmt.Errorf("failed to commit transaction: %w", err)
		}
		return &m, nil
	}

	// Postgres with FOR UPDATE SKIP LOCKED
	selectQuery = `
		SELECT mission_id
		FROM ohc_tasks.mission_queue
		WHERE status = 'QUEUED'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`

	err = tx.QueryRow(ctx, selectQuery).Scan(&missionID)
	if err != nil {
		// Could be sql.ErrNoRows if nothing is available
		return nil, err
	}

	updateQuery := `
		UPDATE ohc_tasks.mission_queue
		SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = CURRENT_TIMESTAMP
		WHERE mission_id = $2
		RETURNING title, priority, payload, created_at, updated_at
	`

	var m Mission
	m.MissionID = missionID
	m.Status = "IN_PROGRESS"
	m.AssignedAgent = &agentID

	err = tx.QueryRow(ctx, updateQuery, agentID, missionID).Scan(&m.Title, &m.Priority, &m.Payload, &m.CreatedAt, &m.UpdatedAt)
	if err != nil {
		return nil, fmt.Errorf("failed to update mission: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &m, nil
}
