package interop

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"os"
)

type Mission struct {
	MissionID     string          `json:"mission_id"`
	Title         string          `json:"title"`
	Status        string          `json:"status"`
	AssignedAgent string          `json:"assigned_agent"`
	Priority      string          `json:"priority"`
	Payload       json.RawMessage `json:"payload"`
}

func ClaimMission(ctx context.Context, db *sql.DB, agentID string) (*Mission, error) {
	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var m Mission
	var payload []byte
    var query string
    var updateQuery string

    if os.Getenv("OHC_STANDALONE") == "true" {
        query = `
            SELECT mission_id, title, status, COALESCE(assigned_agent, ''), priority, payload
            FROM ohc_tasks.mission_queue
            WHERE status = 'QUEUED'
            LIMIT 1
        `
        updateQuery = `
            UPDATE ohc_tasks.mission_queue
            SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = CURRENT_TIMESTAMP
            WHERE mission_id = $2
        `
    } else {
        query = `
            SELECT mission_id, title, status, COALESCE(assigned_agent, ''), priority, payload
            FROM ohc_tasks.mission_queue
            WHERE status = 'QUEUED'
            FOR UPDATE SKIP LOCKED
            LIMIT 1
        `
        updateQuery = `
            UPDATE ohc_tasks.mission_queue
            SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = NOW()
            WHERE mission_id = $2
        `
    }

	err = tx.QueryRowContext(ctx, query).Scan(&m.MissionID, &m.Title, &m.Status, &m.AssignedAgent, &m.Priority, &payload)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task found
		}
		return nil, err
	}
	m.Payload = json.RawMessage(payload)

	_, err = tx.ExecContext(ctx, updateQuery, agentID, m.MissionID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	m.Status = "IN_PROGRESS"
	m.AssignedAgent = agentID
	return &m, nil
}
