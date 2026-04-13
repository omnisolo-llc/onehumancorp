package orchestration

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

const (
	TaskStatePending    = "PENDING"
	TaskStateDecomposing = "DECOMPOSING"
	TaskStateExecuting   = "EXECUTING"
	TaskStateVerifying   = "VERIFYING"
	TaskStateDone        = "DONE"
	TaskStateFailed      = "FAILED"

	EventSubTaskCompleted    = "SubTaskCompleted"
	EventSubTaskFailed       = "SubTaskFailed"
	EventDecompositionComplete = "DecompositionComplete"
)

type TaskStateMachine struct {
	dbProvider db.Provider
}

func NewTaskStateMachine(dbProvider db.Provider) *TaskStateMachine {
	return &TaskStateMachine{dbProvider: dbProvider}
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID string, event string) error {
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var query string
	if sm.dbProvider.IsSQLite() {
		query = `SELECT status, parent_task_id FROM shared_tasks WHERE id = $1`
	} else {
		query = `SELECT status, parent_task_id FROM shared_tasks WHERE id = $1 FOR UPDATE`
	}

	var status string
	var parentTaskIDNullable *string
	var parentTaskID *string
	if err := tx.QueryRow(ctx, query, taskID).Scan(&status, &parentTaskIDNullable); err != nil {
		return err
	}
	if parentTaskIDNullable != nil {
		parentTaskID = parentTaskIDNullable
	}

	if event == EventSubTaskCompleted && parentTaskID != nil {
		var parentStatus string
		var parentQuery string
		if sm.dbProvider.IsSQLite() {
			parentQuery = `SELECT status FROM shared_tasks WHERE id = $1`
		} else {
			parentQuery = `SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE`
		}
		if err := tx.QueryRow(ctx, parentQuery, *parentTaskID).Scan(&parentStatus); err != nil {
			return err
		}

		// Check if all children are DONE
		var pendingChildren int
		childQuery := fmt.Sprintf(`SELECT COUNT(*) FROM shared_tasks WHERE parent_task_id = $1 AND status != '%s'`, TaskStateDone)
		if err := tx.QueryRow(ctx, childQuery, *parentTaskID).Scan(&pendingChildren); err != nil {
			return err
		}

		if pendingChildren == 0 {
			updateParentQuery := fmt.Sprintf(`UPDATE shared_tasks SET status = '%s' WHERE id = $1`, TaskStateVerifying)
			if _, err := tx.Exec(ctx, updateParentQuery, *parentTaskID); err != nil {
				return err
			}
		}
	} else if event == EventSubTaskFailed && parentTaskID != nil {
		updateParentQuery := fmt.Sprintf(`UPDATE shared_tasks SET status = '%s' WHERE id = $1`, TaskStateFailed)
		if _, err := tx.Exec(ctx, updateParentQuery, *parentTaskID); err != nil {
			return err
		}
	} else if event == EventDecompositionComplete {
		updateQuery := fmt.Sprintf(`UPDATE shared_tasks SET status = '%s' WHERE id = $1`, TaskStateExecuting)
		if _, err := tx.Exec(ctx, updateQuery, taskID); err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}
