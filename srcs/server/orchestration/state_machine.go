package orchestration

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

const (
	TaskStatePending     = "PENDING"
	TaskStateDecomposing = "DECOMPOSING"
	TaskStateExecuting   = "EXECUTING"
	TaskStateVerifying   = "VERIFYING"
	TaskStateDone        = "DONE"
	TaskStateFailed      = "FAILED"

	EventSubTaskCompleted      = "SubTaskCompleted"
	EventSubTaskFailed         = "SubTaskFailed"
	EventDecompositionComplete = "DecompositionComplete"
)

type TaskStateMachine struct {
	dbProvider db.Provider
	mutexProvider MutexProvider
}

func NewTaskStateMachine(provider db.Provider, redisClient rueidis.Client) *TaskStateMachine {
	ctx := context.Background()
	mp, _ := NewMutexProvider(ctx, provider, redisClient)
	return &TaskStateMachine{dbProvider: provider, mutexProvider: mp}
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID string, event string) error {
	if sm.mutexProvider != nil {
		mx := sm.mutexProvider.NewMutex("sm:" + taskID)
		if err := mx.Lock(ctx, 30*time.Second); err != nil {
			return fmt.Errorf("failed to acquire state machine lock: %w", err)
		}
		defer mx.Unlock(ctx)
	}

	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var currentState string
	var parentTaskID *string
	var workflowState *string
	query := `SELECT status, parent_task_id, workflow_state FROM shared_tasks WHERE id = $1`
	if !sm.dbProvider.IsSQLite() {
		query += ` FOR UPDATE`
	}
	err = tx.QueryRow(ctx, query, taskID).Scan(&currentState, &parentTaskID, &workflowState)
	if err != nil {
		return fmt.Errorf("failed to find task: %w", err)
	}

	if event == EventSubTaskCompleted {
		_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateDone, taskID)
		if err != nil {
			return fmt.Errorf("failed to update subtask state: %w", err)
		}

		if parentTaskID != nil && *parentTaskID != "" {
			var parentState string
			pQuery := `SELECT status FROM shared_tasks WHERE id = $1`
			if !sm.dbProvider.IsSQLite() {
				pQuery += ` FOR UPDATE`
			}
			err = tx.QueryRow(ctx, pQuery, *parentTaskID).Scan(&parentState)
			if err != nil {
				return fmt.Errorf("failed to query parent task: %w", err)
			}

			var incompleteCount int
			err = tx.QueryRow(ctx, `SELECT count(*) FROM shared_tasks WHERE parent_task_id = $1 AND status != $2`, *parentTaskID, TaskStateDone).Scan(&incompleteCount)
			if err != nil {
				return fmt.Errorf("failed to query subtasks: %w", err)
			}

			if incompleteCount == 0 {
				_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateDone, *parentTaskID)
				if err != nil {
					return fmt.Errorf("failed to update parent state: %w", err)
				}
			}
		}
	} else if event == EventSubTaskFailed {
		_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateFailed, taskID)
		if err != nil {
			return fmt.Errorf("failed to update task state: %w", err)
		}
	} else if event == EventDecompositionComplete {
		workflowStateUpdate := `{"last_event": "DecompositionComplete"}`
		_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = $1, workflow_state = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`, TaskStateExecuting, workflowStateUpdate, taskID)
		if err != nil {
			return fmt.Errorf("failed to update task state: %w", err)
		}
	}

	return tx.Commit(ctx)
}
