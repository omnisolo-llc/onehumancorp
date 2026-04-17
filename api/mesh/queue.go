package mesh

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
)

// EnqueueMission adds a new mission to the ohc_tasks.mission_queue
func EnqueueMission(ctx context.Context, db *sql.DB, title, priority string, payload json.RawMessage) (string, error) {
	if db == nil {
		return "", fmt.Errorf("db connection is nil")
	}

	query := `
		INSERT INTO ohc_tasks.mission_queue (title, priority, payload)
		VALUES ($1, $2, $3)
		RETURNING mission_id
	`
	var missionID string
	err := db.QueryRowContext(ctx, query, title, priority, payload).Scan(&missionID)
	if err != nil {
		return "", fmt.Errorf("failed to enqueue mission: %w", err)
	}

	return missionID, nil
}

// CompleteMission marks an IN_PROGRESS mission as DONE
func CompleteMission(ctx context.Context, db *sql.DB, missionID, agentID string) error {
	if db == nil {
		return fmt.Errorf("db connection is nil")
	}

	query := `
		UPDATE ohc_tasks.mission_queue
		SET status = 'DONE',
		    updated_at = NOW()
		WHERE mission_id = $1 AND assigned_agent = $2 AND status = 'IN_PROGRESS'
	`
	res, err := db.ExecContext(ctx, query, missionID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete mission: %w", err)
	}

	rows, err := res.RowsAffected()
	if err != nil {
		return fmt.Errorf("failed to check rows affected: %w", err)
	}
	if rows == 0 {
		return fmt.Errorf("mission not found, not in progress, or not assigned to agent")
	}

	return nil
}
