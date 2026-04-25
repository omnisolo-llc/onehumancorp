package mesh

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/redis/go-redis/v9"
)

// SubAgentTask represents a task in the ohc_tasks.sub_agent_queue
type SubAgentTask struct {
	ID           string          `json:"id"`
	ParentTaskID string          `json:"parent_task_id"`
	Payload      json.RawMessage `json:"payload"`
	Status       string          `json:"status"`
	WorkerID     *string         `json:"worker_id"`
	CreatedAt    time.Time       `json:"created_at"`
	UpdatedAt    time.Time       `json:"updated_at"`
}

// QueueOrchestrator manages distributed queuing via Redis and PostgreSQL
type QueueOrchestrator struct {
	db     *sql.DB
	redis  *redis.Client
	isSQLite bool
}

// NewQueueOrchestrator creates a new queue orchestrator
func NewQueueOrchestrator(db *sql.DB, redis *redis.Client, isSQLite bool) *QueueOrchestrator {
	return &QueueOrchestrator{
		db:     db,
		redis:  redis,
		isSQLite: isSQLite,
	}
}

// EnqueueSubTask adds a task to the sub-agent queue
func (q *QueueOrchestrator) EnqueueSubTask(ctx context.Context, parentTaskID string, payload json.RawMessage) (string, error) {
	if q.db == nil {
		return "", fmt.Errorf("db connection is nil")
	}

	var query string
	if q.isSQLite {
		query = `
			INSERT INTO sub_agent_queue (parent_task_id, payload, status)
			VALUES ($1, $2, 'QUEUED')
			RETURNING id
		`
	} else {
		query = `
			INSERT INTO ohc_tasks.sub_agent_queue (parent_task_id, payload, status)
			VALUES ($1, $2, 'QUEUED')
			RETURNING id
		`
	}
	var taskID string
	err := q.db.QueryRowContext(ctx, query, parentTaskID, payload).Scan(&taskID)
	if err != nil {
		return "", fmt.Errorf("failed to enqueue sub-task: %w", err)
	}

	if q.redis != nil {
		event := map[string]string{
			"type": "new_sub_task",
			"task_id": taskID,
		}
		data, _ := json.Marshal(event)
		q.redis.Publish(ctx, "kairos:sub_tasks", data)
	}

	return taskID, nil
}

// ClaimSubTask claims a pending sub-task for a worker
func (q *QueueOrchestrator) ClaimSubTask(ctx context.Context, workerID string) (*SubAgentTask, error) {
	if q.db == nil {
		return nil, fmt.Errorf("db connection is nil")
	}

	var query string
	if q.isSQLite {
		query = `
			UPDATE sub_agent_queue
			SET status = 'IN_PROGRESS', worker_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM sub_agent_queue WHERE status = 'QUEUED' LIMIT 1
			)
			RETURNING id, parent_task_id, payload, status, worker_id, created_at, updated_at
		`
	} else {
		query = `
			UPDATE ohc_tasks.sub_agent_queue
			SET status = 'IN_PROGRESS', worker_id = $1, updated_at = NOW()
			WHERE id = (
				SELECT id FROM ohc_tasks.sub_agent_queue WHERE status = 'QUEUED' FOR UPDATE SKIP LOCKED LIMIT 1
			)
			RETURNING id, parent_task_id, payload, status, worker_id, created_at, updated_at
		`
	}

	row := q.db.QueryRowContext(ctx, query, workerID)
	var task SubAgentTask
	err := row.Scan(
		&task.ID,
		&task.ParentTaskID,
		&task.Payload,
		&task.Status,
		&task.WorkerID,
		&task.CreatedAt,
		&task.UpdatedAt,
	)

	if err == sql.ErrNoRows {
		return nil, nil // No pending tasks
	}
	if err != nil {
		return nil, fmt.Errorf("failed to claim sub-task: %w", err)
	}

	return &task, nil
}

// CompleteSubTask marks a sub-task as completed
func (q *QueueOrchestrator) CompleteSubTask(ctx context.Context, taskID, workerID string) error {
	if q.db == nil {
		return fmt.Errorf("db connection is nil")
	}

	var query string
	if q.isSQLite {
		query = `
			UPDATE sub_agent_queue
			SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
			WHERE id = $1 AND worker_id = $2 AND status = 'IN_PROGRESS'
			RETURNING id
		`
	} else {
		query = `
			UPDATE ohc_tasks.sub_agent_queue
			SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
			WHERE id = $1 AND worker_id = $2 AND status = 'IN_PROGRESS'
			RETURNING id
		`
	}
	var returnedID string
	err := q.db.QueryRowContext(ctx, query, taskID, workerID).Scan(&returnedID)
	if err != nil {
		if strings.Contains(err.Error(), "no rows") {
			return fmt.Errorf("sub-task not found, not in progress, or not assigned to worker")
		}
		return fmt.Errorf("failed to complete sub-task: %w", err)
	}

	return nil
}

// EnqueueMission adds a new mission to the ohc_tasks.mission_queue
func EnqueueMission(ctx context.Context, db *sql.DB, title, priority string, payload json.RawMessage) (string, error) {
	isSQLite := db != nil && fmt.Sprintf("%T", db.Driver()) == "*sqlite.Driver"

	if db == nil {
		return "", fmt.Errorf("db connection is nil")
	}

	var query string
	if isSQLite {
		query = `
			INSERT INTO mission_queue (title, priority, payload)
			VALUES ($1, $2, $3)
			RETURNING mission_id
		`
	} else {
		query = `
			INSERT INTO ohc_tasks.mission_queue (title, priority, payload)
			VALUES ($1, $2, $3)
			RETURNING mission_id
		`
	}
	var missionID string
	err := db.QueryRowContext(ctx, query, title, priority, payload).Scan(&missionID)
	if err != nil {
		return "", fmt.Errorf("failed to enqueue mission: %w", err)
	}

	return missionID, nil
}

// CompleteMission marks an IN_PROGRESS mission as DONE
func CompleteMission(ctx context.Context, db *sql.DB, missionID, agentID string) error {
	isSQLite := db != nil && fmt.Sprintf("%T", db.Driver()) == "*sqlite.Driver"

	if db == nil {
		return fmt.Errorf("db connection is nil")
	}

	var query string
	if isSQLite {
		query = `
			UPDATE mission_queue
			SET status = 'DONE',
			    updated_at = CURRENT_TIMESTAMP
			WHERE mission_id = $1 AND assigned_agent = $2 AND status = 'IN_PROGRESS'
			RETURNING mission_id
		`
	} else {
		query = `
			UPDATE ohc_tasks.mission_queue
			SET status = 'DONE',
			    updated_at = NOW()
			WHERE mission_id = $1 AND assigned_agent = $2 AND status = 'IN_PROGRESS'
			RETURNING mission_id
		`
	}
	var returnedID string
	err := db.QueryRowContext(ctx, query, missionID, agentID).Scan(&returnedID)
	if err != nil {
		if strings.Contains(err.Error(), "no rows") {
			return fmt.Errorf("mission not found, not in progress, or not assigned to agent")
		}
		return fmt.Errorf("failed to complete mission: %w", err)
	}

	return nil
}
