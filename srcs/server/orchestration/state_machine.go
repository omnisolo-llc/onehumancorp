package orchestration

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"log/slog"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type Event string

const (
	EventSubTaskCompleted Event = "SubTaskCompleted"
	EventSubTaskFailed    Event = "SubTaskFailed"
	EventDecompositionComplete Event = "DecompositionComplete"
)

type TaskStateMachine struct {
	dbProvider db.Provider
}

func NewTaskStateMachine(dbProvider db.Provider) *TaskStateMachine {
	return &TaskStateMachine{
		dbProvider: dbProvider,
	}
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID string, event Event) error {
	switch event {
	case EventSubTaskCompleted:
		return sm.handleSubTaskCompleted(ctx, taskID)
	case EventSubTaskFailed:
		return sm.handleSubTaskFailed(ctx, taskID)
	default:
		slog.Warn("unhandled event", "event", event, "task_id", taskID)
		return nil
	}
}

func (sm *TaskStateMachine) handleSubTaskCompleted(ctx context.Context, taskID string) error {
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Fetch the parent task ID of this completed task
	var parentTaskID *string
	err = tx.QueryRow(ctx, "SELECT parent_task_id FROM shared_tasks WHERE id = $1", taskID).Scan(&parentTaskID)
	if err != nil {
		return fmt.Errorf("failed to fetch parent_task_id for task %s: %w", taskID, err)
	}

	if parentTaskID == nil || *parentTaskID == "" {
		// Not a subtask or parent is unknown. Nothing more to do.
		return tx.Commit(ctx)
	}

	// Lock the parent task row
	var parentStatus string
	var query string
	if sm.dbProvider.IsSQLite() {
		query = "SELECT status FROM shared_tasks WHERE id = $1"
	} else {
		query = "SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE"
	}

	err = tx.QueryRow(ctx, query, *parentTaskID).Scan(&parentStatus)
	if err != nil {
		return fmt.Errorf("failed to fetch parent task %s: %w", *parentTaskID, err)
	}

	if parentStatus == "DONE" || parentStatus == "FAILED" || parentStatus == "VERIFYING" {
		// Parent already handled
		return tx.Commit(ctx)
	}

	// Check if all child tasks are DONE
	var incompleteCount int
	err = tx.QueryRow(ctx, "SELECT COUNT(*) FROM shared_tasks WHERE parent_task_id = $1 AND status != 'DONE'", *parentTaskID).Scan(&incompleteCount)
	if err != nil {
		return fmt.Errorf("failed to count incomplete subtasks for %s: %w", *parentTaskID, err)
	}

	if incompleteCount == 0 {
		// All subtasks are DONE, transition parent
		_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'VERIFYING' WHERE id = $1", *parentTaskID)
		if err != nil {
			return fmt.Errorf("failed to update parent task %s to VERIFYING: %w", *parentTaskID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

func (sm *TaskStateMachine) handleSubTaskFailed(ctx context.Context, taskID string) error {
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var parentTaskID *string
	err = tx.QueryRow(ctx, "SELECT parent_task_id FROM shared_tasks WHERE id = $1", taskID).Scan(&parentTaskID)
	if err != nil {
		return fmt.Errorf("failed to fetch parent_task_id for task %s: %w", taskID, err)
	}

	if parentTaskID == nil || *parentTaskID == "" {
		return tx.Commit(ctx)
	}

	var parentStatus string
	var query string
	if sm.dbProvider.IsSQLite() {
		query = "SELECT status FROM shared_tasks WHERE id = $1"
	} else {
		query = "SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE"
	}

	err = tx.QueryRow(ctx, query, *parentTaskID).Scan(&parentStatus)
	if err != nil {
		if strings.Contains(err.Error(), "no rows") {
			return tx.Commit(ctx) // Parent might be deleted
		}
		return fmt.Errorf("failed to fetch parent task %s: %w", *parentTaskID, err)
	}

	if parentStatus == "DONE" || parentStatus == "FAILED" {
		return tx.Commit(ctx)
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'FAILED' WHERE id = $1", *parentTaskID)
	if err != nil {
		return fmt.Errorf("failed to update parent task %s to FAILED: %w", *parentTaskID, err)
	}

	return tx.Commit(ctx)
}

// Decomposer represents the generic decomposition logic interface needed.
type KAIROSDecomposer interface {
	DecomposeTask(ctx context.Context, organizationID, parentPlanID, parentTaskID, prompt string) error
}

func generateSMID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}

func (sm *TaskStateMachine) HandleHighLevelRequest(ctx context.Context, decomposer KAIROSDecomposer, organizationID, parentPlanID, prompt string) (string, error) {
	// Create PENDING task
	taskID := generateSMID()

	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return "", fmt.Errorf("failed to begin transaction: %w", err)
	}

	insertQuery := `
		INSERT INTO shared_tasks (id, organization_id, title, status, parent_plan_id)
		VALUES ($1, $2, 'High Level Request', 'PENDING', $3)
	`
	_, err = tx.Exec(ctx, insertQuery, taskID, organizationID, parentPlanID)
	if err != nil {
		tx.Rollback(ctx)
		return "", fmt.Errorf("failed to insert pending task: %w", err)
	}

	// Transition to DECOMPOSING
	updateQuery := `UPDATE shared_tasks SET status = 'DECOMPOSING' WHERE id = $1`
	_, err = tx.Exec(ctx, updateQuery, taskID)
	if err != nil {
		tx.Rollback(ctx)
		return "", fmt.Errorf("failed to transition to DECOMPOSING: %w", err)
	}

	err = tx.Commit(ctx)
	if err != nil {
		return "", fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Decompose task into subtasks
	err = decomposer.DecomposeTask(ctx, organizationID, parentPlanID, taskID, prompt)
	if err != nil {
		// Update parent state to FAILED
		sm.handleSubTaskFailed(ctx, taskID) // using handleSubTaskFailed helper conceptually
		_, _ = sm.dbProvider.Exec(ctx, "UPDATE shared_tasks SET status = 'FAILED' WHERE id = $1", taskID)
		return taskID, fmt.Errorf("failed to decompose task: %w", err)
	}

	// Simulate EventDecompositionComplete (transition to EXECUTING)
	// After successful decomposition, child tasks are queued, parent moves to EXECUTING
	_, err = sm.dbProvider.Exec(ctx, "UPDATE shared_tasks SET status = 'EXECUTING' WHERE id = $1", taskID)
	if err != nil {
		return taskID, fmt.Errorf("failed to transition to EXECUTING: %w", err)
	}

	return taskID, nil
}
