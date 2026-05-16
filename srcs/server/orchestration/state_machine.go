package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"strings"
	"sync"
)

type TaskEvent string

const (
	EventSubTaskCompleted      TaskEvent = "SubTaskCompleted"
	EventSubTaskFailed         TaskEvent = "SubTaskFailed"
	EventDecompositionComplete TaskEvent = "DecompositionComplete"
)

type TaskStateMachine struct {
	db *sql.DB
	mu sync.Mutex // For SQLite concurrent updates
}

func NewTaskStateMachine(db *sql.DB) *TaskStateMachine {
	return &TaskStateMachine{db: db}
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID string, event TaskEvent) error {
	// Handle sqlite syntax difference for tests vs postgres
	var isSqlite bool
	err := sm.db.QueryRow("SELECT sqlite_version()").Scan(new(string))
	isSqlite = err == nil

	if isSqlite {
		sm.mu.Lock()
		defer sm.mu.Unlock()
	}

	tx, err := sm.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// 1. Lock the parent task row
	var status string
	var workflowState sql.NullString
	query := "SELECT status, workflow_state FROM ohc_tasks WHERE id = $1"
	if !isSqlite {
		query += " FOR UPDATE"
	}
	err = tx.QueryRowContext(ctx, query, taskID).Scan(&status, &workflowState)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return fmt.Errorf("task not found: %s", taskID)
		}
		return err
	}

	if status == "DONE" || status == "FAILED" {
		return nil // Already in terminal state
	}

	switch event {
	case EventSubTaskCompleted:
		// Check if all subtasks are complete
		var incompleteCount int
		err = tx.QueryRowContext(ctx, "SELECT COUNT(*) FROM ohc_tasks WHERE parent_task_id = $1 AND status != 'DONE'", taskID).Scan(&incompleteCount)
		if err != nil {
			return err
		}

		if incompleteCount == 0 {
			_, err = tx.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'VERIFYING' WHERE id = $1", taskID)
			if err != nil {
				return err
			}
		}

	case EventSubTaskFailed:
		_, err = tx.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'FAILED' WHERE id = $1", taskID)
		if err != nil {
			return err
		}

	case EventDecompositionComplete:
		if status == "DECOMPOSING" {
			_, err = tx.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'EXECUTING' WHERE id = $1", taskID)
			if err != nil {
				return err
			}
		}
	}

	_ = strings.ToLower(status)

	return tx.Commit()
}
