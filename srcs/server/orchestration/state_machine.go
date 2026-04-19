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
	TaskStateBlocked     = "BLOCKED"
	TaskStateReady       = "READY"
	TaskStateDecomposing = "DECOMPOSING"
	TaskStateExecuting   = "EXECUTING"
	TaskStateVerifying   = "VERIFYING"
	TaskStateDone        = "DONE"
	TaskStateFailed      = "FAILED"

	EventSubTaskCompleted      = "SubTaskCompleted"
	EventTaskCompleted         = "task.completed"
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

	} else if event == EventTaskCompleted {
		_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateDone, taskID)
		if err != nil {
			return fmt.Errorf("failed to update task state to done: %w", err)
		}

		// Find dependent tasks and check if they can transition to READY
		rows, err := tx.Query(ctx, `SELECT task_id FROM swarm_task_dependencies WHERE depends_on_task_id = $1`, taskID)
		if err == nil {
			defer rows.Close()
			var dependentTasks []string
			for rows.Next() {
				var depTaskID string
				if err := rows.Scan(&depTaskID); err == nil {
					dependentTasks = append(dependentTasks, depTaskID)
				}
			}
			rows.Close() // Close early before doing more queries in the same tx

			for _, depTaskID := range dependentTasks {
				var incompleteCount int
				err = tx.QueryRow(ctx, `
					SELECT COUNT(*)
					FROM swarm_task_dependencies td
					JOIN shared_tasks st ON td.depends_on_task_id = st.id
					WHERE td.task_id = $1 AND st.status != $2
				`, depTaskID, TaskStateDone).Scan(&incompleteCount)

				if err == nil && incompleteCount == 0 {
					// All dependencies met, transition to READY
					_, _ = tx.Exec(ctx, `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = $3`, TaskStateReady, depTaskID, TaskStateBlocked)
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


// TransitionState changes the state of a task and checks dependencies.
func (sm *TaskStateMachine) TransitionState(ctx context.Context, taskID string, newState string) error {
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

	_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, newState, taskID)
	if err != nil {
		return fmt.Errorf("failed to transition state: %w", err)
	}
	return tx.Commit(ctx)
}

// CheckDependencies verifies if all prerequisites are met for a task.
func (sm *TaskStateMachine) CheckDependencies(ctx context.Context, taskID string) (bool, error) {
	if sm.mutexProvider != nil {
		mx := sm.mutexProvider.NewMutex("sm:deps:" + taskID)
		if err := mx.Lock(ctx, 30*time.Second); err != nil {
			return false, fmt.Errorf("failed to acquire state machine lock: %w", err)
		}
		defer mx.Unlock(ctx)
	}

	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return false, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var incompleteCount int
	err = tx.QueryRow(ctx, `
		SELECT COUNT(*)
		FROM swarm_task_dependencies td
		JOIN shared_tasks st ON td.depends_on_task_id = st.id
		WHERE td.task_id = $1 AND st.status != $2
	`, taskID, TaskStateDone).Scan(&incompleteCount)
	if err != nil {
		return false, fmt.Errorf("failed to check dependencies: %w", err)
	}

	return incompleteCount == 0, nil
}
