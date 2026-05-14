package db

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"
)

type DB struct {
	db *sql.DB
    isSQLite bool
}

type Task struct {
	ID           string
	ParentTaskID sql.NullString
	AgentID      sql.NullString
	Status       string
	Payload      json.RawMessage
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

func (d *DB) IsSQLite() bool {
	return d.isSQLite
}

func (d *DB) AcquireTask(ctx context.Context, agentID string) (*Task, error) {
	var task Task
	var payloadStr string

	if d.IsSQLite() {
		// SQLite fallback
		query := `UPDATE tasks SET status = 'RUNNING', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = (SELECT id FROM tasks WHERE status = 'PENDING' LIMIT 1) RETURNING id, parent_task_id, agent_id, status, payload, created_at, updated_at`
		err := d.db.QueryRowContext(ctx, query, agentID).Scan(
			&task.ID, &task.ParentTaskID, &task.AgentID, &task.Status, &payloadStr, &task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
	} else {
		// Postgres
		query := `UPDATE tasks SET status = 'RUNNING', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = (SELECT id FROM tasks WHERE status = 'PENDING' LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, parent_task_id, agent_id, status, payload, created_at, updated_at`
		err := d.db.QueryRowContext(ctx, query, agentID).Scan(
			&task.ID, &task.ParentTaskID, &task.AgentID, &task.Status, &payloadStr, &task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
	}
	task.Payload = json.RawMessage(payloadStr)
	return &task, nil
}
