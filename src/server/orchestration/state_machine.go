package orchestration

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/src/server/telemetry"

	"github.com/onehumancorp/mono/src/server/db"
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
	dbProvider    db.Provider
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
	var updatedAt time.Time
	query := `SELECT status, parent_task_id, workflow_state, updated_at FROM ohc_tasks WHERE id = $1`
	if !sm.dbProvider.IsSQLite() {
		query += ` FOR UPDATE`
	}
	err = tx.QueryRow(ctx, query, taskID).Scan(&currentState, &parentTaskID, &workflowState, &updatedAt)
	if err != nil {
		return fmt.Errorf("failed to find task: %w", err)
	}

	var nextState string

	if event == EventSubTaskCompleted {
		nextState = TaskStateDone
		_, err = tx.Exec(ctx, `UPDATE ohc_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateDone, taskID)
		if err != nil {
			return fmt.Errorf("failed to update subtask state: %w", err)
		}

		if parentTaskID != nil && *parentTaskID != "" {
			var parentState string
			pQuery := `SELECT status FROM ohc_tasks WHERE id = $1`
			if !sm.dbProvider.IsSQLite() {
				pQuery += ` FOR UPDATE`
			}
			err = tx.QueryRow(ctx, pQuery, *parentTaskID).Scan(&parentState)
			if err != nil {
				return fmt.Errorf("failed to query parent task: %w", err)
			}

			var incompleteCount int
			err = tx.QueryRow(ctx, `SELECT count(*) FROM ohc_tasks WHERE parent_task_id = $1 AND status != $2`, *parentTaskID, TaskStateDone).Scan(&incompleteCount)
			if err != nil {
				return fmt.Errorf("failed to query subtasks: %w", err)
			}

			if incompleteCount == 0 {
				_, err = tx.Exec(ctx, `UPDATE ohc_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateDone, *parentTaskID)
				if err != nil {
					return fmt.Errorf("failed to update parent state: %w", err)
				}
			}
		}

	} else if event == EventTaskCompleted {
		nextState = TaskStateDone
		_, err = tx.Exec(ctx, `UPDATE ohc_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateDone, taskID)
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
					JOIN ohc_tasks st ON td.depends_on_task_id = st.id
					WHERE td.task_id = $1 AND st.status != $2
				`, depTaskID, TaskStateDone).Scan(&incompleteCount)

				if err == nil && incompleteCount == 0 {
					// All dependencies met, transition to READY
					_, _ = tx.Exec(ctx, `UPDATE ohc_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = $3`, TaskStateReady, depTaskID, TaskStateBlocked)
				}
			}
		}
	} else if event == EventSubTaskFailed {
		nextState = TaskStateFailed
		_, err = tx.Exec(ctx, `UPDATE ohc_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, TaskStateFailed, taskID)
		if err != nil {
			return fmt.Errorf("failed to update task state: %w", err)
		}
	} else if event == EventDecompositionComplete {
		nextState = TaskStateExecuting
		workflowStateUpdate := `{"last_event": "DecompositionComplete"}`
		_, err = tx.Exec(ctx, `UPDATE ohc_tasks SET status = $1, workflow_state = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`, TaskStateExecuting, workflowStateUpdate, taskID)
		if err != nil {
			return fmt.Errorf("failed to update task state: %w", err)
		}
	}

	err = tx.Commit(ctx)
	if err == nil && nextState != "" && nextState != currentState && !updatedAt.IsZero() {
		transition := strings.ToLower(currentState) + "_to_" + strings.ToLower(nextState)
		telemetry.RecordAgentTransitionLatency(ctx, transition, time.Since(updatedAt).Seconds())
	}
	return err
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

	var currentState string
	var updatedAt time.Time
	err = tx.QueryRow(ctx, `SELECT status, updated_at FROM ohc_tasks WHERE id = $1`, taskID).Scan(&currentState, &updatedAt)
	if err != nil && err.Error() != "sql: no rows in result set" {
		return fmt.Errorf("failed to get current state: %w", err)
	}

	_, err = tx.Exec(ctx, `UPDATE ohc_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, newState, taskID)
	if err != nil {
		return fmt.Errorf("failed to transition state: %w", err)
	}
	err = tx.Commit(ctx)
	if err == nil && currentState != "" && currentState != newState && !updatedAt.IsZero() {
		transition := strings.ToLower(currentState) + "_to_" + strings.ToLower(newState)
		telemetry.RecordAgentTransitionLatency(ctx, transition, time.Since(updatedAt).Seconds())
	}
	return err
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
		JOIN ohc_tasks st ON td.depends_on_task_id = st.id
		WHERE td.task_id = $1 AND st.status != $2
	`, taskID, TaskStateDone).Scan(&incompleteCount)
	if err != nil {
		return false, fmt.Errorf("failed to check dependencies: %w", err)
	}

	return incompleteCount == 0, nil
}
