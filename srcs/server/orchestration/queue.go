package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/google/uuid"
)

// Task represents a queued sub-agent task.
type Task struct {
	ID             string
	OrganizationID string
	ParentTaskID   string
	Payload        map[string]interface{}
	Status         string
	WorkerID       string
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

// TaskQueue is the interface for managing sub-agent tasks.
type TaskQueue interface {
	// Enqueue adds a new task to the queue.
	Enqueue(ctx context.Context, task *Task) error
	// Dequeue attempts to fetch and lock a pending task from the queue.
	Dequeue(ctx context.Context, workerID string) (*Task, error)
	// Acknowledge marks a task as successfully completed.
	Acknowledge(ctx context.Context, taskID string) error
}

// Ensure interface implementations
var _ TaskQueue = (*SQLiteTaskQueue)(nil)
var _ TaskQueue = (*PostgresTaskQueue)(nil)

// NewTaskQueue creates the appropriate TaskQueue implementation based on the environment.
func NewTaskQueue(provider db.Provider) TaskQueue {
	if provider.IsSQLite() {
		return &SQLiteTaskQueue{provider: provider}
	}
	return &PostgresTaskQueue{provider: provider}
}

// SQLiteTaskQueue uses an atomic UPDATE subquery to simulate a queue in SQLite.
type SQLiteTaskQueue struct {
	provider db.Provider
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, task *Task) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.Status == "" {
		task.Status = "QUEUED"
	}
	now := time.Now()
	if task.CreatedAt.IsZero() {
		task.CreatedAt = now
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = now
	}

	payloadBytes, err := json.Marshal(task.Payload)
	if err != nil {
		return err
	}

	query := `
		INSERT INTO sub_agent_queue (
			id, organization_id, parent_task_id, payload, status, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7
		)
	`
	_, err = q.provider.Exec(ctx, query,
		task.ID, task.OrganizationID, task.ParentTaskID, string(payloadBytes),
		task.Status, task.CreatedAt.Format(time.RFC3339Nano), task.UpdatedAt.Format(time.RFC3339Nano),
	)
	return err
}

func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, workerID string) (*Task, error) {
	query := `
		UPDATE sub_agent_queue
		SET status = 'RUNNING', worker_id = $1, updated_at = $2
		WHERE id IN (
			SELECT id FROM sub_agent_queue
			WHERE status = 'QUEUED'
			ORDER BY created_at ASC LIMIT 1
		)
		RETURNING id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
	`

	var t Task
	var payloadStr string
	var wID sql.NullString
	var createdAt, updatedAt string

	now := time.Now().Format(time.RFC3339Nano)
	err := q.provider.QueryRow(ctx, query, workerID, now).Scan(
		&t.ID, &t.OrganizationID, &t.ParentTaskID, &payloadStr, &t.Status, &wID, &createdAt, &updatedAt,
	)

	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil // Empty queue
	} else if err != nil {
		return nil, err
	}

	if wID.Valid {
		t.WorkerID = wID.String
	}
	if parsed, err := time.Parse(time.RFC3339Nano, createdAt); err == nil {
		t.CreatedAt = parsed
	}
	if parsed, err := time.Parse(time.RFC3339Nano, updatedAt); err == nil {
		t.UpdatedAt = parsed
	}
	if err := json.Unmarshal([]byte(payloadStr), &t.Payload); err != nil {
		return nil, fmt.Errorf("failed to unmarshal payload: %w", err)
	}

	return &t, nil
}

func (q *SQLiteTaskQueue) Acknowledge(ctx context.Context, taskID string) error {
	query := `UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = $1 WHERE id = $2`
	_, err := q.provider.Exec(ctx, query, time.Now().Format(time.RFC3339Nano), taskID)
	return err
}

// PostgresTaskQueue uses SKIP LOCKED for efficient concurrent queueing.
type PostgresTaskQueue struct {
	provider db.Provider
}

func (q *PostgresTaskQueue) Enqueue(ctx context.Context, task *Task) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.Status == "" {
		task.Status = "QUEUED"
	}
	now := time.Now()
	if task.CreatedAt.IsZero() {
		task.CreatedAt = now
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = now
	}

	payloadBytes, err := json.Marshal(task.Payload)
	if err != nil {
		return err
	}

	query := `
		INSERT INTO sub_agent_queue (
			id, organization_id, parent_task_id, payload, status, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7
		)
	`
	_, err = q.provider.Exec(ctx, query,
		task.ID, task.OrganizationID, task.ParentTaskID, string(payloadBytes),
		task.Status, task.CreatedAt, task.UpdatedAt,
	)
	return err
}

func (q *PostgresTaskQueue) Dequeue(ctx context.Context, workerID string) (*Task, error) {
	query := `
		UPDATE sub_agent_queue
		SET status = 'RUNNING', worker_id = $1, updated_at = $2
		WHERE id = (
			SELECT id FROM sub_agent_queue
			WHERE status = 'QUEUED'
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
	`

	var t Task
	var payloadStr string
	var wID sql.NullString
	var createdAt, updatedAt time.Time

	now := time.Now()
	err := q.provider.QueryRow(ctx, query, workerID, now).Scan(
		&t.ID, &t.OrganizationID, &t.ParentTaskID, &payloadStr, &t.Status, &wID, &createdAt, &updatedAt,
	)

	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil // Empty queue
	} else if err != nil {
		return nil, err
	}

	if wID.Valid {
		t.WorkerID = wID.String
	}
	t.CreatedAt = createdAt
	t.UpdatedAt = updatedAt
	if err := json.Unmarshal([]byte(payloadStr), &t.Payload); err != nil {
		return nil, fmt.Errorf("failed to unmarshal payload: %w", err)
	}

	return &t, nil
}

func (q *PostgresTaskQueue) Acknowledge(ctx context.Context, taskID string) error {
	query := `UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = $1 WHERE id = $2`
	_, err := q.provider.Exec(ctx, query, time.Now(), taskID)
	return err
}
