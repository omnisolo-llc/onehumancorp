package orchestration

import (
	"context"
	"database/sql"
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
	db *sql.DB
	redisClient rueidis.Client
	mu sync.Mutex // For SQLite concurrent updates
}

func NewTaskStateMachine(db *sql.DB) *TaskStateMachine {
	return &TaskStateMachine{db: db}
}

func (sm *TaskStateMachine) SetRedisClient(client rueidis.Client) {
	sm.redisClient = client
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID string, event TaskEvent) error {
	// Handle sqlite syntax difference for tests vs postgres
	var isSqlite bool
	err := sm.db.QueryRow("SELECT sqlite_version()").Scan(new(string))
	isSqlite = err == nil

	if isSqlite {
		sm.mu.Lock()
		defer sm.mu.Unlock()
	} else if sm.redisClient != nil {
		lockKey := fmt.Sprintf("lock:task:%s", taskID)
		lockVal := fmt.Sprintf("%d", time.Now().UnixNano())
		cmd := sm.redisClient.B().Set().Key(lockKey).Value(lockVal).Nx().Ex(10 * time.Second).Build()
		err := sm.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return fmt.Errorf("could not acquire lock for task: %s", taskID)
			}
			return err
		}
		defer func() {
			script := rueidis.NewLuaScript("if redis.call('get',KEYS[1]) == ARGV[1] then return redis.call('del',KEYS[1]) else return 0 end")
			_ = script.Exec(context.Background(), sm.redisClient, []string{lockKey}, []string{lockVal}).Error()
		}()
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
