package orchestration

import (
	"context"
	"database/sql"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/db"
)

const (
	StatePending               = "PENDING"
	StateDecomposing           = "DECOMPOSING"
	StateExecutingSubtasks     = "EXECUTING_SUBTASKS"
	StateVerifying             = "VERIFYING"
	StateCompleted             = "COMPLETED"
	StateDone                  = "DONE"
	StateFailed                = "FAILED"
	EventSubTaskCompleted      = "SubTaskCompleted"
	EventSubTaskFailed         = "SubTaskFailed"
	EventDecompositionComplete = "DecompositionComplete"
)

type TaskStateMachine struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewTaskStateMachine(dbProvider db.Provider) *TaskStateMachine {
	return &TaskStateMachine{dbProvider: dbProvider}
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID, event string) error {
	if sm.dbProvider.IsSQLite() {
		sm.mu.Lock()
		defer sm.mu.Unlock()
	}
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var parentTaskID sql.NullString
	var status string

	query := "SELECT parent_task_id, status FROM shared_tasks WHERE id = $1"
	if !sm.dbProvider.IsSQLite() {
		query += " FOR UPDATE"
	}
	err = tx.QueryRow(ctx, query, taskID).Scan(&parentTaskID, &status)
	if err != nil {
		return err
	}

	if event == EventSubTaskCompleted && parentTaskID.Valid {
		var parentStatus string
		query = "SELECT status FROM shared_tasks WHERE id = $1"
		if !sm.dbProvider.IsSQLite() {
			query += " FOR UPDATE"
		}
		err = tx.QueryRow(ctx, query, parentTaskID.String).Scan(&parentStatus)
		if err != nil {
			return err
		}

		var count int
		err = tx.QueryRow(ctx, "SELECT COUNT(*) FROM shared_tasks WHERE parent_task_id = $1 AND status NOT IN ('DONE', 'COMPLETED')", parentTaskID.String).Scan(&count)
		if err != nil {
			return err
		}

		if count == 0 {
			_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'VERIFYING' WHERE id = $1", parentTaskID.String)
			if err != nil {
				return err
			}
		}
	} else if event == EventDecompositionComplete {
		_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'EXECUTING_SUBTASKS' WHERE id = $1", taskID)
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}
