package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"sync"
	"time"

	"github.com/redis/rueidis"
)

type TaskEvent string

const (
	EventSubTaskCompleted      TaskEvent = "SubTaskCompleted"
	EventSubTaskFailed         TaskEvent = "SubTaskFailed"
	EventDecompositionComplete TaskEvent = "DecompositionComplete"
)

type TaskStateMachine struct {
	db            *sql.DB
	redisClient   rueidis.Client
	meshTransport MeshTransport
	mu            sync.Mutex // For SQLite concurrent updates
	isSqlite      bool
}

func NewTaskStateMachine(db *sql.DB, redisClient rueidis.Client, meshTransport MeshTransport) *TaskStateMachine {
	var isSqlite bool
	if db != nil {
		err := db.QueryRow("SELECT sqlite_version()").Scan(new(string))
		isSqlite = err == nil
	}

	return &TaskStateMachine{
		db:            db,
		redisClient:   redisClient,
		meshTransport: meshTransport,
		isSqlite:      isSqlite,
	}
}

// Transition performs a state transition for a task, guarded by a distributed Redis lock.
func (sm *TaskStateMachine) Transition(ctx context.Context, taskID string, fromState string, toState string) error {
	agentID, ok := ctx.Value("agent_id").(string)
	if !ok || agentID == "" {
		agentID = "system_agent"
	}

	// 1. Acquire Redis lock
	lockKey := fmt.Sprintf("mesh:lock:%s", taskID)

	if sm.redisClient != nil {
		cmd := sm.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Ex(30 * time.Second).Build()
		res := sm.redisClient.Do(ctx, cmd)
		if res.Error() != nil {
			if rueidis.IsRedisNil(res.Error()) {
				return fmt.Errorf("could not acquire lock for task %s", taskID)
			}
			return fmt.Errorf("failed to acquire lock: %w", res.Error())
		}
	} else {
		// Mock logic for simple test execution without redis
		sm.mu.Lock()
		defer sm.mu.Unlock()
	}

	// Release lock function
	releaseLock := func() {
		if sm.redisClient != nil {
			sm.redisClient.Do(ctx, sm.redisClient.B().Del().Key(lockKey).Build())
		}
	}

	// 2. Perform DB transition
	if sm.isSqlite && sm.redisClient != nil { // only lock again if redis didn't mock lock it
		sm.mu.Lock()
		defer sm.mu.Unlock()
	}

	tx, err := sm.db.BeginTx(ctx, nil)
	if err != nil {
		releaseLock()
		return err
	}
	defer tx.Rollback()

	var currentStatus string
	query := "SELECT status FROM ohc_tasks WHERE id = $1"
	if !sm.isSqlite {
		query += " FOR UPDATE"
	}

	err = tx.QueryRowContext(ctx, query, taskID).Scan(&currentStatus)
	if err != nil {
		releaseLock()
		if errors.Is(err, sql.ErrNoRows) {
			return fmt.Errorf("task not found: %s", taskID)
		}
		return err
	}

	if currentStatus != fromState {
		releaseLock()
		return fmt.Errorf("invalid state transition: current state is %s, expected %s", currentStatus, fromState)
	}

	_, err = tx.ExecContext(ctx, "UPDATE ohc_tasks SET status = $1 WHERE id = $2", toState, taskID)
	if err != nil {
		releaseLock()
		return err
	}

	if err := tx.Commit(); err != nil {
		releaseLock()
		return err
	}

	// Release lock successfully
	releaseLock()

	// 3. Broadcast Event
	if sm.meshTransport != nil {
		transitionData := map[string]string{
			"task_id":    taskID,
			"from_state": fromState,
			"to_state":   toState,
			"agent_id":   agentID,
		}
		dataBytes, _ := json.Marshal(transitionData)
		rawMsg := json.RawMessage(dataBytes)
		msg := MeshMessage{
			AgentID:   agentID,
			EventType: "StateTransition",
			Data:      &rawMsg,
			Channel:   "orchestration",
		}

		msgBytes, _ := json.Marshal(msg)
		_ = sm.meshTransport.Publish(ctx, "orchestration", msgBytes)
	}

	return nil
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID string, event TaskEvent) error {
	if sm.isSqlite {
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
	if !sm.isSqlite {
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
