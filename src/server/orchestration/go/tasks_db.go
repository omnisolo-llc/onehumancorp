package orchestration

import (
	"context"
	"database/sql"
	"fmt"
)

type Task struct {
	ID       int
	Status   string
	Priority string
	AgentID  sql.NullString
}

type TaskDB struct {
	db *sql.DB
}

func NewTaskDB(db *sql.DB) *TaskDB {
	return &TaskDB{db: db}
}

// ClaimTask claims a pending task for the given agent using FOR UPDATE SKIP LOCKED.
func (tdb *TaskDB) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	// Start a transaction
	tx, err := tdb.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	// Query for a pending task and lock it
	query := `
		SELECT id, status, priority, agent_id
		FROM shared_tasks
		WHERE status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	var task Task
	err = tx.QueryRowContext(ctx, query).Scan(&task.ID, &task.Status, &task.Priority, &task.AgentID)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No tasks available
		}
		return nil, fmt.Errorf("failed to select pending task: %w", err)
	}

	// Update the task status to IN_PROGRESS and assign the agent ID
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', agent_id = $1
		WHERE id = $2
	`
	_, err = tx.ExecContext(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	// Commit the transaction
	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Update the task struct to reflect the new state
	task.Status = "IN_PROGRESS"
	task.AgentID = sql.NullString{String: agentID, Valid: true}

	return &task, nil
}
